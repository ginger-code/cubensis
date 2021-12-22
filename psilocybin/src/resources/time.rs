use hyphae::events::CubensisEvent;
use psilocyn::device::GraphicsDevice;
use psilocyn::resources::CubensisResource;
use wgpu::util::DeviceExt;

pub struct TimeResource {
    graphics: std::rc::Rc<GraphicsDevice>,
    binding_group: u32,
    binding_offset: u32,
    program_start_time: std::time::Instant,
    frame_count: u32,
    time_buffer_data: TimeBufferData,
    time_buffer: wgpu::Buffer,
    average_fps: f32,
}

impl TimeResource {
    pub fn new(
        graphics: std::rc::Rc<GraphicsDevice>,
        binding_group: u32,
        binding_offset: u32,
    ) -> Self {
        log::trace!("Creating time resource");
        let program_start_time = std::time::Instant::now();
        let frame_count = 0 as u32;
        let time_buffer_data = TimeBufferData::new();
        let time_buffer = graphics
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Time Uniform Buffer"),
                contents: bytemuck::cast_slice(&[time_buffer_data]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });
        let average_fps = 0.0;
        Self {
            graphics,
            program_start_time,
            frame_count,
            time_buffer_data,
            time_buffer,
            average_fps,
            binding_group,
            binding_offset,
        }
    }

    pub fn get_start_time(&self) -> std::time::Instant {
        log::trace!("Retrieving program start time");
        self.program_start_time.clone()
    }
    pub fn get_elapsed_time(&self) -> std::time::Duration {
        log::trace!("Retrieving time elapsed since program started");
        self.program_start_time.elapsed()
    }
    pub fn get_frame_count(&self) -> u32 {
        log::trace!("Retrieving current frame index");
        self.frame_count
    }
    pub fn get_average_fps(&self) -> f32 {
        log::trace!("Retrieving average FPS");
        self.average_fps
    }
}

impl CubensisResource for TimeResource {
    fn update(&mut self, time_delta: std::time::Duration) -> bool {
        log::trace!("Updating time resource");
        self.frame_count += 1;
        self.average_fps = 1.0 / time_delta.as_secs_f32();
        self.time_buffer_data
            .update(self.get_frame_count(), self.get_elapsed_time(), time_delta);
        self.graphics.queue.write_buffer(
            &self.time_buffer,
            0,
            bytemuck::cast_slice(&[self.time_buffer_data]),
        );
        false
    }
    fn resize(&mut self) {
        log::trace!("Resizing time resource");
    }
    fn get_bind_group_layout_entries(&self) -> Vec<wgpu::BindGroupLayoutEntry> {
        log::trace!("Retrieving time resource bind group layout entries");
        vec![wgpu::BindGroupLayoutEntry {
            binding: self.binding_offset,
            visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }]
    }
    fn get_bind_group_entries(&self) -> Vec<wgpu::BindGroupEntry> {
        log::trace!("Retrieving time resource bind group entries");
        vec![wgpu::BindGroupEntry {
            binding: self.binding_offset,
            resource: self.time_buffer.as_entire_binding(),
        }]
    }

    fn handle_or_capture_event(&mut self, _event: &winit::event::Event<'_, CubensisEvent>) -> bool {
        log::trace!("Handling event in time resource");
        false
    }

    fn binding_group(&self) -> u32 {
        self.binding_group
    }

    fn binding_offset(&self) -> u32 {
        self.binding_offset
    }

    fn binding_size() -> u32 {
        1
    }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct TimeBufferData {
    frame_index: u32,
    time_seconds: f32,
    frame_time_seconds: f32,
}
impl TimeBufferData {
    fn new() -> Self {
        Self {
            frame_index: 0,
            time_seconds: 0.0,
            frame_time_seconds: 0.0,
        }
    }
    fn update(
        &mut self,
        frame_index: u32,
        time_since_start: std::time::Duration,
        time_delta: std::time::Duration,
    ) {
        log::trace!("Updating time resource data");
        self.frame_index = frame_index;
        self.time_seconds = time_since_start.as_secs_f32();
        self.frame_time_seconds = time_delta.as_secs_f32();
    }
}
