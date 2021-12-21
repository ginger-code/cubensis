use crate::configuration::Configuration;
use crate::events::CubensisEvent;

pub trait CubensisRendererPlugin {
    fn new(
        event_proxy: winit::event_loop::EventLoopProxy<CubensisEvent>,
        configuration: Configuration,
    ) -> Self;
    fn handle_event(&mut self, event: &CubensisEvent);
    fn start(&mut self);
    fn shutdown(&mut self);
}

pub trait CubensisPluginCollection {
    fn new(
        event_proxy: winit::event_loop::EventLoopProxy<CubensisEvent>,
        configuration: Configuration,
    ) -> Self;
    fn handle_event(&mut self, event: &CubensisEvent);
    fn start_all(&mut self);
    fn shutdown(&mut self);
}
