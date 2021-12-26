use crate::device::GraphicsDevice;
use crate::resources::CubensisResourceCollection;
use hyphae::configuration::library::Library;
use hyphae::events::CubensisEvent;
use std::rc::Rc;

pub struct RepaintSignal(std::sync::Mutex<winit::event_loop::EventLoopProxy<CubensisEvent>>);

impl epi::RepaintSignal for RepaintSignal {
    fn request_repaint(&self) {
        log::trace!("Requesting GUI repaint");
        self.0
            .lock()
            .unwrap()
            .send_event(CubensisEvent::GuiRedrawRequest {})
            .ok();
    }
}

pub(crate) struct GuiHost {
    graphics: Rc<GraphicsDevice>,
    window: Rc<winit::window::Window>,
    repaint_signal: std::sync::Arc<RepaintSignal>,
    platform: egui_winit_platform::Platform,
    gui_renderpass: egui_wgpu_backend::RenderPass,
    start_time: std::time::Instant,
    previous_frame_time: Option<f32>,
}

impl GuiHost {
    pub fn new(graphics: Rc<GraphicsDevice>, window: Rc<winit::window::Window>) -> Self {
        log::debug!("Creating GUI host");
        let repaint_signal = std::sync::Arc::new(RepaintSignal(std::sync::Mutex::new(
            graphics.get_event_proxy(),
        )));
        let dev = graphics.clone();
        let size = dev.get_size();
        let format = dev.get_format();
        let platform =
            egui_winit_platform::Platform::new(egui_winit_platform::PlatformDescriptor {
                physical_width: size.width as u32,
                physical_height: size.height as u32,
                scale_factor: dev.window_scale_factor,
                font_definitions: egui::FontDefinitions::default(),
                style: Default::default(),
            });
        let gui_renderpass = egui_wgpu_backend::RenderPass::new(&graphics.device, format, 1);
        let start_time = std::time::Instant::now();
        let previous_frame_time = None;
        Self {
            graphics,
            window,
            repaint_signal,
            platform,
            gui_renderpass,
            start_time,
            previous_frame_time,
        }
    }
    pub fn update(&mut self, time_delta: std::time::Duration) {
        log::trace!("Updating GUI");
        self.platform.update_time(time_delta.as_secs_f64());
        self.start_time = std::time::Instant::now();
    }

    ///true if the event was captured
    pub fn handle_or_capture_event(&mut self, event: &winit::event::Event<CubensisEvent>) -> bool {
        let captures = self.platform.captures_event(event);
        log::trace!("Handling GUI event with capture = {}", captures);
        self.platform.handle_event(event);
        captures
    }
}

pub(crate) trait CubensisGuiRenderer<ResourceCollection, Gui>
where
    ResourceCollection: CubensisResourceCollection,
    Gui: CubensisGuiApp<ResourceCollection>,
{
    fn render_gui(
        &mut self,
        gui_host: &mut GuiHost,
        app: &mut Gui,
        library: &Library,
        resource_collection: &ResourceCollection,
        render_target: &wgpu::TextureView,
    );
}

impl<ResourceCollection, Gui> CubensisGuiRenderer<ResourceCollection, Gui> for wgpu::CommandEncoder
where
    ResourceCollection: CubensisResourceCollection,
    Gui: CubensisGuiApp<ResourceCollection>,
{
    fn render_gui(
        &mut self,
        gui_host: &mut GuiHost,
        app: &mut Gui,
        library: &Library,
        resource_collection: &ResourceCollection,
        render_target: &wgpu::TextureView,
    ) {
        log::trace!("Rendering GUI");
        gui_host.platform.begin_frame();
        let mut app_output = epi::backend::AppOutput::default();
        let mut frame = epi::backend::FrameBuilder {
            info: epi::IntegrationInfo {
                name: "Cubensis GUI",
                web_info: None,
                cpu_usage: gui_host.previous_frame_time,
                native_pixels_per_point: Some(gui_host.graphics.window_scale_factor as f32),
                prefer_dark_mode: Some(true),
            },
            tex_allocator: &mut gui_host.gui_renderpass,
            output: &mut app_output,
            repaint_signal: gui_host.repaint_signal.clone(),
        }
        .build();
        app.draw(
            &gui_host.platform.context(),
            &mut frame,
            library,
            resource_collection,
        );
        let (_, paint_commands) = gui_host.platform.end_frame(Some(&gui_host.window));
        let paint_jobs = gui_host.platform.context().tessellate(paint_commands);
        let frame_time = (std::time::Instant::now() - gui_host.start_time).as_secs_f64() as f32;
        gui_host.previous_frame_time = Some(frame_time);
        let extent = gui_host.graphics.create_extent3d(1);
        let screen_descriptor = egui_wgpu_backend::ScreenDescriptor {
            physical_width: extent.width,
            physical_height: extent.height,
            scale_factor: gui_host.graphics.window_scale_factor as f32,
        };
        gui_host.gui_renderpass.update_texture(
            &gui_host.graphics.device,
            &gui_host.graphics.queue,
            &gui_host.platform.context().texture(),
        );
        gui_host
            .gui_renderpass
            .update_user_textures(&gui_host.graphics.device, &gui_host.graphics.queue);
        gui_host.gui_renderpass.update_buffers(
            &gui_host.graphics.device,
            &gui_host.graphics.queue,
            &paint_jobs,
            &screen_descriptor,
        );
        gui_host
            .gui_renderpass
            .execute(self, &render_target, &paint_jobs, &screen_descriptor, None)
            .unwrap();
    }
}

pub trait CubensisGuiApp<ResourceCollection>
where
    ResourceCollection: CubensisResourceCollection,
{
    fn new() -> Self;
    fn update(&mut self, time_delta: std::time::Duration);
    fn draw(
        &mut self,
        context: &egui::CtxRef,
        frame: &epi::Frame,
        library: &Library,
        resource_collection: &ResourceCollection,
    );
    fn handle_event(&mut self, event: &winit::event::Event<CubensisEvent>);
}
