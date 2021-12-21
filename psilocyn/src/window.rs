use hyphae::events::CubensisEvent;
use winit::platform::windows::WindowBuilderExtWindows;

pub(crate) trait CubensisWindowBuilder {
    fn create_window(&self) -> winit::window::Window;
}
#[cfg(not(windows))]
impl CubensisWindowBuilder for winit::event_loop::EventLoop<CubensisEvent> {
    fn create_window(&self) -> winit::window::Window {
        let icon = load_window_icon();
        winit::window::WindowBuilder::new()
            .with_title("Cubensis")
            .with_window_icon(Some(icon.clone()))
            .with_inner_size(winit::dpi::PhysicalSize::new(800 as u32, 800 as u32))
            .build(self)
            .unwrap()
    }
}
#[cfg(windows)]
impl CubensisWindowBuilder for winit::event_loop::EventLoop<CubensisEvent> {
    fn create_window(&self) -> winit::window::Window {
        let icon = load_window_icon();
        winit::window::WindowBuilder::new()
            .with_title("Cubensis")
            .with_theme(Some(winit::window::Theme::Dark))
            .with_window_icon(Some(icon.clone()))
            .with_taskbar_icon(Some(icon.clone()))
            .with_inner_size(winit::dpi::PhysicalSize::new(800 as u32, 800 as u32))
            .build(self)
            .unwrap()
    }
}

fn load_window_icon() -> winit::window::Icon {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::load_from_memory(include_bytes!("../../window_icon.png"))
            .unwrap()
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    winit::window::Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap()
}
