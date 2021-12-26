use crate::gui::main_menu::MainMenu;
use crate::gui::CubensisGuiWidget;
use crate::ResourceCollection;

pub struct AudioWidget;

impl CubensisGuiWidget<AudioWidget> for MainMenu {
    fn draw(&self, context: &egui::CtxRef, resource_collection: &ResourceCollection) {
        if !self.audio_widget_enabled {
            return;
        }
        let audio_stream_info = resource_collection.audio.get_stream_info();
        egui::containers::Window::new("Audio Info")
            .auto_sized()
            .resizable(true)
            .collapsible(false)
            .min_height(100.0)
            .show(context, |ui| {
                ui.label(format!("Audio Device: {}", audio_stream_info.device_name));
            });
    }

    fn menu_title(&self) -> String {
        "Audio Info".to_string()
    }

    fn toggle(&mut self) -> &mut bool {
        &mut self.audio_widget_enabled
    }

    fn is_enabled(&self) -> bool {
        self.audio_widget_enabled
    }
}
