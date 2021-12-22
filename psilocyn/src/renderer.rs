use crate::device::GraphicsDevice;
use crate::gui::{CubensisGuiApp, GuiHost};
use crate::mesh::{CubensisMeshRenderPass, Mesh};
use crate::presentation::presenter::CubensisPresenter;
use crate::presentation::PresentationPass;
use crate::resources::{CubensisMeshSpawner, CubensisResourceCollection};
use crate::window::CubensisWindowBuilder;
use hyphae::configuration::library::Library;
use hyphae::configuration::Configuration;
use hyphae::events::CubensisEvent;
use hyphae::plugins::CubensisPluginCollection;
use hyphae::scene::Scene;
use std::rc::Rc;
use winit::event::{Event, WindowEvent};
use winit::event_loop::ControlFlow;
use winit::window::WindowId;

pub struct Renderer<
    ResourceCollection: 'static + CubensisResourceCollection,
    Gui: 'static + CubensisGuiApp<ResourceCollection>,
    Plugins: 'static + CubensisPluginCollection,
    const HISTORY_DEPTH: usize,
> {
    window: Rc<winit::window::Window>,
    graphics: Rc<GraphicsDevice>,
    resource_collection: ResourceCollection,
    gui_host: GuiHost,
    presentation_pass: PresentationPass<HISTORY_DEPTH>,
    meshes: Vec<Mesh>,
    gui: Gui,
    _scene: Scene,
    _start_time: std::time::Instant,
    last_frame_time: std::time::Instant,
    plugins: Plugins,
    _library: Library,
}

impl<
        ResourceCollection: 'static + CubensisResourceCollection,
        Gui: 'static + CubensisGuiApp<ResourceCollection>,
        Plugins: 'static + CubensisPluginCollection,
        const HISTORY_DEPTH: usize,
    > Renderer<ResourceCollection, Gui, Plugins, HISTORY_DEPTH>
{
    fn new(
        window: Rc<winit::window::Window>,
        event_proxy: winit::event_loop::EventLoopProxy<CubensisEvent>,
        configuration: Configuration,
    ) -> Self {
        log::debug!("Creating new renderer");
        let graphics = Rc::new(GraphicsDevice::new(
            configuration.clone(),
            &window,
            event_proxy.clone(),
        ));
        let resource_collection = ResourceCollection::new(graphics.clone(), configuration.clone());
        let gui_host = GuiHost::new(graphics.clone(), window.clone());
        let presentation_pass = PresentationPass::new(graphics.clone());
        let _library = configuration.library.build_library();
        let scene = _library.current_scene();
        let history_bind_group_layout = presentation_pass.get_bind_group_layout();
        let meshes = scene.create_meshes(
            graphics.clone(),
            &resource_collection,
            history_bind_group_layout,
        );
        let gui = Gui::new();
        let start_time = std::time::Instant::now();
        let last_frame_time = start_time.clone();
        let plugins = Plugins::new(event_proxy.clone(), configuration.clone());
        Self {
            window,
            graphics,
            resource_collection,
            gui_host,
            presentation_pass,
            meshes,
            gui,
            _scene: scene,
            _start_time: start_time,
            last_frame_time,
            plugins,
            _library,
        }
    }

    pub fn run(configuration: Configuration) -> ! {
        log::debug!("Running renderer");
        let event_loop: winit::event_loop::EventLoop<CubensisEvent> =
            winit::event_loop::EventLoop::with_user_event();
        let window = Rc::new(event_loop.create_window());
        let proxy = event_loop.create_proxy();
        let mut renderer = Self::new(window.clone(), proxy.clone(), configuration.clone());
        renderer.plugins.start_all();
        event_loop.run(move |event, _window_target, control_flow| {
            log::trace!("Rendering frame");
            renderer.handle_event(&event, control_flow).unwrap();
        });
    }

    fn handle_app_event(
        &mut self,
        event: &CubensisEvent,
        _control_flow: &mut ControlFlow,
    ) -> anyhow::Result<()> {
        self.plugins.handle_event(event);
        log::debug!("Handling application event");
        match event {
            CubensisEvent::FileEdit(path) => self._scene.try_hot_reload(
                path,
                &mut self.meshes,
                &self.resource_collection,
                self.presentation_pass.get_bind_group_layout(),
            ),
            _ => Ok(()),
        }
    }

    fn on_present_finish(&mut self, _control_flow: &mut ControlFlow) -> anyhow::Result<()> {
        log::trace!("Window presented image");
        self.window.request_redraw();
        Ok(())
    }

    fn handle_redraw(&mut self, control_flow: &mut ControlFlow) -> anyhow::Result<()> {
        log::trace!("Handling window redraw request");
        self.update()?;
        match self.render() {
            Ok(_) => {}
            Err(wgpu::SurfaceError::Lost) => {
                log::trace!("Surface was lost");
                self.resize(self.graphics.get_size())
            }
            Err(wgpu::SurfaceError::OutOfMemory) => {
                log::error!("Graphics out-of-memory error");
                *control_flow = winit::event_loop::ControlFlow::Exit
            }
            Err(e) => {
                log::trace!("Unhandled surface error: {:?}", e)
            }
        };
        Ok(())
    }

    fn handle_window_event(
        &mut self,
        window_id: WindowId,
        event: &WindowEvent,
        control_flow: &mut ControlFlow,
    ) -> anyhow::Result<()> {
        log::trace!("Handling window event");
        if window_id != self.window.id() {
            return Ok(());
        }
        match event {
            WindowEvent::Resized(new_inner_size) => {
                log::debug!("Window resized");
                self.resize(*new_inner_size)
            }
            WindowEvent::Moved(_) => {}
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                input:
                    winit::event::KeyboardInput {
                        state: winit::event::ElementState::Pressed,
                        virtual_keycode: Some(winit::event::VirtualKeyCode::Escape),
                        ..
                    },
                ..
            } => {
                log::debug!("Exit signal received, shutting down");
                *control_flow = ControlFlow::Exit
            }
            WindowEvent::Destroyed => {}
            WindowEvent::DroppedFile(_) => {}
            WindowEvent::HoveredFile(_) => {}
            WindowEvent::HoveredFileCancelled => {}
            WindowEvent::ReceivedCharacter(_) => {}
            WindowEvent::Focused(_) => {}
            WindowEvent::KeyboardInput { .. } => {}
            WindowEvent::ModifiersChanged(_) => {}
            WindowEvent::CursorMoved { .. } => {}
            WindowEvent::CursorEntered { .. } => {}
            WindowEvent::CursorLeft { .. } => {}
            WindowEvent::MouseWheel { .. } => {}
            WindowEvent::MouseInput { .. } => {}
            WindowEvent::TouchpadPressure { .. } => {}
            WindowEvent::AxisMotion { .. } => {}
            WindowEvent::Touch(_) => {}
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                log::debug!("Scale factor changed, resizing");
                self.resize(**new_inner_size);
            }
            WindowEvent::ThemeChanged(_) => {}
        };
        Ok(())
    }

    fn _recreate_scene(&mut self, scene: Scene) -> anyhow::Result<()> {
        log::debug!("Recreating meshes");
        let history_bind_group_layout = self.presentation_pass.get_bind_group_layout();
        self.meshes = scene.create_meshes(
            self.graphics.clone(),
            &self.resource_collection,
            history_bind_group_layout,
        );
        self._scene = scene;
        Ok(())
    }

    fn update(&mut self) -> anyhow::Result<()> {
        log::trace!("Updating renderer");
        let current_frame_time = std::time::Instant::now();
        let time_delta = current_frame_time - self.last_frame_time;
        self.last_frame_time = current_frame_time;
        self.resource_collection.update(time_delta);
        self.gui_host.update(time_delta);
        self.gui.update(time_delta);
        Ok(())
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        log::trace!("Rendering");
        self.presentation_pass.start_frame();
        let view = self.presentation_pass.create_presentation_view();
        let depth_buffer = self.presentation_pass.get_depth_texture_view();
        let history_bind_group = self.presentation_pass.get_current_bind_group();
        let mut encoder =
            self.graphics
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });
        let bind_groups = self.resource_collection.get_bind_groups();

        for i in 0..self.meshes.len() {
            let mesh = &self.meshes[i];
            let renderpass_name = format!("Render Pass {}", i);

            let mut renderpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some(renderpass_name.as_str()),
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.01,
                            g: 0.01,
                            b: 0.01,
                            a: 1.0,
                        }),
                        store: true,
                    },
                }],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: depth_buffer,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: true,
                    }),
                    stencil_ops: None,
                }),
            });
            renderpass.draw_mesh_indexed(mesh, bind_groups.as_slice(), history_bind_group);
        }
        encoder.present(
            &mut self.presentation_pass,
            &mut self.gui_host,
            &self.gui,
            &self.resource_collection,
        )?;
        Ok(())
    }

    fn resize(&mut self, size: winit::dpi::PhysicalSize<u32>) {
        log::trace!("Resizing renderer");
        if self.graphics.resize(size) {
            self.resource_collection.resize();
            self.presentation_pass.resize();
        }
    }

    fn handle_event(
        &mut self,
        event: &winit::event::Event<CubensisEvent>,
        control_flow: &mut ControlFlow,
    ) -> anyhow::Result<()> {
        log::trace!("Handling event in renderer");
        if self.gui_host.handle_or_capture_event(event) {
            return Ok(());
        }
        self.gui.handle_event(event);
        self.resource_collection.handle_event(event);
        match event {
            Event::NewEvents(_) => {}
            Event::WindowEvent {
                window_id,
                ref event,
            } => self.handle_window_event(*window_id, event, control_flow)?,
            Event::DeviceEvent { .. } => {}
            Event::UserEvent(ref event) => match self.handle_app_event(event, control_flow) {
                Ok(_) => (),
                Err(err) => {
                    log::warn!("Failed to handle application event {:?}.\r\n", event);
                    log::warn!(
                        "Application event handling error message: {}",
                        err.to_string()
                    )
                }
            },
            Event::Suspended => {}
            Event::Resumed => {}
            Event::MainEventsCleared => self.on_present_finish(control_flow)?,
            Event::RedrawRequested(_) => self.handle_redraw(control_flow)?,
            Event::RedrawEventsCleared => {}
            Event::LoopDestroyed => {}
        };
        Ok(())
    }
}

impl<
        ResourceCollection: 'static + CubensisResourceCollection,
        Gui: 'static + CubensisGuiApp<ResourceCollection>,
        Plugins: 'static + CubensisPluginCollection,
        const HISTORY_DEPTH: usize,
    > Drop for Renderer<ResourceCollection, Gui, Plugins, HISTORY_DEPTH>
{
    fn drop(&mut self) {
        self.plugins.shutdown();
    }
}
