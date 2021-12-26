use crate::gui::components::menu::MainMenuAndWidgets;
use crate::gui::CubensisGuiWidget;
use crate::ResourceCollection;
use egui::CtxRef;

pub struct CameraWidget;

impl CubensisGuiWidget<CameraWidget> for MainMenuAndWidgets {
    fn draw(&self, context: &CtxRef, resource_collection: &ResourceCollection) {
        if !self.camera_widget_enabled {
            return;
        }
        let camera = &resource_collection.camera.arcball_camera;
        //todo: replace camera. probably drop arcball as an option until I understand what's happening
        egui::containers::Window::new("Camera Info")
            .resizable(true)
            .collapsible(false)
            .auto_sized()
            .min_height(100.0)
            .show(context, |ui| {
                ui.label(format!("Translation: {:?}", camera.translation));
                ui.label(format!(
                    "Center Translation: {:?}",
                    camera.center_translation
                ));
                ui.label(format!("Rotation: {:?}", camera.rotation));
                ui.label(format!("Camera: {:?}", camera.camera));
                ui.label(format!("Inverse Camera: {:?}", camera.inv_camera));
                ui.label(format!("Zoom Speed: {:?}", camera.zoom_speed));
                ui.label(format!(
                    "Perspective Projection: {:?}",
                    camera.perspective_projection
                ));
                ui.label(format!("Projection: {:?}", camera.projection));
            });
    }

    fn menu_title(&self) -> String {
        "Camera Info".to_string()
    }

    fn toggle(&mut self) -> &mut bool {
        &mut self.camera_widget_enabled
    }

    fn is_enabled(&self) -> bool {
        self.camera_widget_enabled
    }
}
