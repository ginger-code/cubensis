use hyphae::events::CubensisEvent;
use psilocyn::device::GraphicsDevice;
use psilocyn::resources::CubensisResource;
use std::time::Duration;
use wgpu::util::DeviceExt;
use wgpu::{BindGroupEntry, BindGroupLayoutEntry};
use winit::event::{Event, WindowEvent};

mod arcball;

pub struct CameraResource {
    graphics: std::rc::Rc<GraphicsDevice>,
    binding_group: u32,
    binding_offset: u32,
    camera_buffer: wgpu::Buffer,
    pub arcball_camera: arcball::ArcballCamera,
    previous_mouse_position: Option<winit::dpi::PhysicalPosition<f64>>,
    mouse_button_pressed: [bool; 2],
    requires_update: bool,
}
impl CameraResource {
    pub fn new(
        graphics: std::rc::Rc<GraphicsDevice>,
        binding_group: u32,
        binding_offset: u32,
    ) -> Self {
        let size = graphics.get_size();
        let arcball_camera = arcball::ArcballCamera::new(
            cgmath::vec3(0.0, 0.0, 0.0),
            1.0,
            [size.width as f32, size.height as f32],
        );
        let camera_buffer = graphics
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Camera Uniform Buffer"),
                contents: bytemuck::cast_slice(&[arcball_camera]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });
        let previous_mouse_position = None;
        let mouse_button_pressed = [false, false];
        let requires_update = false;
        Self {
            graphics,
            binding_group,
            binding_offset,
            camera_buffer,
            arcball_camera,
            previous_mouse_position,
            mouse_button_pressed,
            requires_update,
        }
    }
}

impl CubensisResource for CameraResource {
    fn update(&mut self, _time_delta: Duration) -> bool {
        if self.requires_update {
            self.graphics.queue.write_buffer(
                &self.camera_buffer,
                0,
                bytemuck::cast_slice(&[self.arcball_camera]),
            );
            self.requires_update = false;
        }
        false
    }

    fn resize(&mut self) {
        let size = self.graphics.get_size();
        self.arcball_camera
            .resize(size.width as f32, size.height as f32);
    }

    fn get_bind_group_layout_entries(&self) -> Vec<BindGroupLayoutEntry> {
        log::trace!("Retrieving camera resource bind group layout entries");
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

    fn get_bind_group_entries(&self) -> Vec<BindGroupEntry> {
        log::trace!("Retrieving camera resource bind group entries");
        vec![wgpu::BindGroupEntry {
            binding: self.binding_offset,
            resource: self.camera_buffer.as_entire_binding(),
        }]
    }

    fn handle_or_capture_event(&mut self, event: &Event<'_, CubensisEvent>) -> bool {
        match event {
            Event::WindowEvent { ref event, .. } => match event {
                WindowEvent::CursorMoved { position, .. } => {
                    if self.previous_mouse_position.is_none() {
                        self.previous_mouse_position = Some(position.clone());
                        return false;
                    } else {
                        let prev = self.previous_mouse_position.unwrap();
                        if self.mouse_button_pressed[0] {
                            self.arcball_camera.rotate(
                                cgmath::vec2(prev.x as f32, prev.y as f32),
                                cgmath::vec2(position.x as f32, position.y as f32),
                            );
                            self.requires_update = true;
                        } else if self.mouse_button_pressed[1] {
                            let mouse_delta = cgmath::vec2(
                                (position.x - prev.x) as f32,
                                (position.y - prev.y) as f32,
                            );
                            self.arcball_camera.pan(mouse_delta);
                            self.requires_update = true;
                        }
                    }
                }
                WindowEvent::MouseWheel { ref delta, .. } => {
                    let y = match delta {
                        winit::event::MouseScrollDelta::LineDelta(_, y) => *y,
                        winit::event::MouseScrollDelta::PixelDelta(p) => p.y.to_owned() as f32,
                    };
                    self.arcball_camera.zoom(y, 0.16);
                    self.requires_update = true;
                }
                WindowEvent::MouseInput {
                    ref state,
                    ref button,
                    ..
                } => {
                    if button == &winit::event::MouseButton::Left {
                        self.mouse_button_pressed[0] =
                            state == &winit::event::ElementState::Pressed;
                    } else if button == &winit::event::MouseButton::Right {
                        self.mouse_button_pressed[1] =
                            state == &winit::event::ElementState::Pressed;
                    }
                }
                _ => {}
            },
            _ => {}
        }
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
