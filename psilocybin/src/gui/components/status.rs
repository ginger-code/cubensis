use crate::gui::CubensisGuiComponent;
use crate::ResourceCollection;
use egui::CtxRef;
use epi::Frame;
use hyphae::events::CubensisEvent;
use std::time::Duration;
use winit::event::Event;

pub struct StatusBar {
    last_frame_time: Duration,
}

impl StatusBar {
    pub fn new() -> Self {
        Self {
            last_frame_time: Duration::from_secs(0),
        }
    }
}

impl CubensisGuiComponent for StatusBar {
    fn update(&mut self, time_delta: Duration) {
        self.last_frame_time = time_delta;
    }

    fn draw(&mut self, context: &CtxRef, _: &Frame, resource_collection: &ResourceCollection) {
        egui::containers::TopBottomPanel::new(egui::panel::TopBottomSide::Bottom, "Status Bar")
            .show(context, |ui| {
                let frame_number = resource_collection.time.get_frame_count();
                let run_time = resource_collection.time.get_elapsed_time();
                let fps = resource_collection.time.get_average_fps();
                ui.label(format!(
                    "{:.1} Frames/Second\t|\tRunning for {:.2} seconds\t|\tLast Frame Time was {:.3} μs\t|\tFrame number {}",
                    fps,
                    run_time.as_secs_f32(),
                    self.last_frame_time.as_micros(),
                    frame_number
                ));
            });
    }

    fn handle_event(&mut self, _: &Event<CubensisEvent>) {}
}
