use crate::gui::CubensisGuiComponent;
use crate::ResourceCollection;
use egui::CtxRef;
use epi::Frame;
use hyphae::configuration::library::Library;
use hyphae::events::CubensisEvent;
use std::time::Duration;
use winit::event::Event;

pub struct LibraryPanel {
    is_enabled: bool,
}
impl LibraryPanel {
    pub fn new() -> Self {
        Self { is_enabled: false }
    }
}
impl CubensisGuiComponent for LibraryPanel {
    fn update(&mut self, _: Duration) {}

    fn draw(
        &mut self,
        context: &CtxRef,
        _: &Frame,
        library: &Library,
        _resource_collection: &ResourceCollection,
    ) {
        if !self.is_enabled {
            return;
        }
        egui::panel::SidePanel::new(egui::panel::Side::Left, "Library Panel").show(context, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Scenes");
            });
            egui::ScrollArea::vertical().show(ui, |ui| {
                for scene in library.scenes() {
                    let is_current = scene.0.eq(&library.current_scene().name);

                    ui.with_layout(
                        egui::Layout::top_down(egui::Align::LEFT).with_cross_justify(true),
                        |ui| {
                            if is_current {
                                ui.add_enabled(false, egui::widgets::Button::new(scene.0));
                            } else {
                                let _button =
                                    ui.add_enabled(true, egui::widgets::Button::new(scene.0));
                            }
                        },
                    );
                }
            });
        });
    }

    fn handle_event(&mut self, event: &Event<CubensisEvent>) {
        match event {
            Event::WindowEvent { ref event, .. } => match event {
                &winit::event::WindowEvent::KeyboardInput {
                    input:
                        winit::event::KeyboardInput {
                            state: winit::event::ElementState::Pressed,
                            virtual_keycode: Some(winit::event::VirtualKeyCode::F1),
                            ..
                        },
                    ..
                } => {
                    log::debug!("Toggling Library Panel");
                    self.is_enabled = !self.is_enabled
                }
                _ => {}
            },

            _ => {}
        }
    }
}
