use crate::gui::status_bar::StatusBar;
use crate::gui::top_menu::TopMenu;
use crate::resources::ResourceCollection;
use hyphae::events::CubensisEvent;
use psilocyn::gui::CubensisGuiApp;
use winit::event::Event;

mod status_bar;
mod top_menu;

type GuiComponent = Box<dyn CubensisGuiComponent>;

pub struct GuiApp {
    is_hidden: bool,
    components: Vec<GuiComponent>,
}

impl GuiApp {}

impl CubensisGuiApp<ResourceCollection> for GuiApp {
    fn new() -> Self {
        let is_hidden = false;
        let components: Vec<GuiComponent> =
            vec![Box::new(TopMenu::new()), Box::new(StatusBar::new())];
        Self {
            is_hidden,
            components,
        }
    }

    fn update(&mut self, time_delta: std::time::Duration) {
        log::trace!("Updating GUI app");
        for component in &mut self.components {
            component.update(time_delta);
        }
    }
    fn draw(
        &self,
        context: &egui::CtxRef,
        frame: &epi::Frame,
        resource_collection: &ResourceCollection,
    ) {
        log::trace!("Drawing GUI app");
        if self.is_hidden {
            return;
        }
        for component in &self.components {
            component.draw(context, frame, resource_collection);
        }
    }

    fn handle_event(&mut self, event: &winit::event::Event<CubensisEvent>) {
        match event {
            Event::WindowEvent { ref event, .. } => match event {
                &winit::event::WindowEvent::KeyboardInput {
                    input:
                        winit::event::KeyboardInput {
                            state: winit::event::ElementState::Pressed,
                            virtual_keycode: Some(winit::event::VirtualKeyCode::Grave),
                            ..
                        },
                    ..
                } => {
                    log::debug!("Toggling UI");
                    self.is_hidden = !self.is_hidden
                }
                _ => {}
            },

            _ => {
                for component in &mut self.components {
                    component.handle_event(event);
                }
            }
        }
    }
}

trait CubensisGuiComponent {
    fn update(&mut self, time_delta: std::time::Duration);
    fn draw(
        &self,
        context: &egui::CtxRef,
        frame: &epi::Frame,
        resource_collection: &ResourceCollection,
    );
    fn handle_event(&mut self, event: &winit::event::Event<CubensisEvent>);
}
