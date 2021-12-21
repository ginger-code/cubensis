use crate::gui::CubensisGuiComponent;
use crate::ResourceCollection;
use egui::CtxRef;
use epi::Frame;
use hyphae::events::CubensisEvent;
use std::time::Duration;
use winit::event::Event;

pub struct TopMenu {}

impl TopMenu {
    pub fn new() -> Self {
        Self {}
    }
}

impl CubensisGuiComponent for TopMenu {
    fn update(&mut self, _: Duration) {}

    fn draw(&self, context: &CtxRef, _: &Frame, _: &ResourceCollection) {
        egui::containers::TopBottomPanel::new(egui::panel::TopBottomSide::Top, "Main Menu").show(
            context,
            |ui| {
                ui.label("Hi");
            },
        );
    }

    fn handle_event(&mut self, _: &Event<CubensisEvent>) {}
}
