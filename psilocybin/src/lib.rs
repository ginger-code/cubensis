use crate::gui::GuiApp;
use crate::resources::ResourceCollection;
use hyphae::configuration::Configuration;
use hyphae::plugins::CubensisPluginCollection;
use psilocyn::renderer::Renderer;

pub mod gui;
pub mod resources;

pub fn run<Plugins: 'static + CubensisPluginCollection, const HISTORY_DEPTH: usize>(
    configuration: Configuration,
) -> ! {
    Renderer::<ResourceCollection, GuiApp, Plugins, HISTORY_DEPTH>::run(configuration)
}
