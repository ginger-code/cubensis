use crate::gui::widgets::audio_widget::AudioWidget;
use crate::gui::widgets::camera_widget::CameraWidget;
use crate::gui::{CubensisGuiComponent, CubensisGuiWidget};
use crate::ResourceCollection;
use egui::CtxRef;
use epi::Frame;
use hyphae::events::CubensisEvent;
use std::time::Duration;
use winit::event::Event;

pub struct MainMenuAndWidgets {
    pub(crate) audio_widget_enabled: bool,
    pub(crate) camera_widget_enabled: bool,
}

impl MainMenuAndWidgets {
    pub fn new() -> Self {
        Self {
            audio_widget_enabled: false,
            camera_widget_enabled: false,
        }
    }

    pub fn draw_widget_menu(&mut self, ui: &mut egui::Ui) {
        egui::menu::bar(ui, |ui| {
            egui::menu::menu(ui, "Widgets", |ui| {
                let widget = self as &mut dyn CubensisGuiWidget<AudioWidget>;
                widget.draw_menu_option(ui);
                let widget = self as &mut dyn CubensisGuiWidget<CameraWidget>;
                widget.draw_menu_option(ui);
            });
        });
    }
    fn draw_widgets(&self, context: &egui::CtxRef, resource_collection: &ResourceCollection) {
        let widget = self as &dyn CubensisGuiWidget<AudioWidget>;
        widget.draw(context, resource_collection);
        let widget = self as &dyn CubensisGuiWidget<CameraWidget>;
        widget.draw(context, resource_collection);
    }
}

impl CubensisGuiComponent for MainMenuAndWidgets {
    fn update(&mut self, _: Duration) {}

    fn draw(&mut self, context: &CtxRef, _: &Frame, resource_collection: &ResourceCollection) {
        egui::containers::TopBottomPanel::new(egui::panel::TopBottomSide::Top, "Main Menu").show(
            context,
            |ui| {
                self.draw_widget_menu(ui);
            },
        );
        self.draw_widgets(context, resource_collection);
    }

    fn handle_event(&mut self, _: &Event<CubensisEvent>) {}
}
