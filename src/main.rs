use crate::plugins::PluginCollection;
use hyphae::configuration::Configuration;
use psilocybin::gui::GuiApp;
use psilocybin::resources::ResourceCollection;
use psilocyn::renderer::Renderer;

mod plugins;

const HISTORY_DEPTH: usize = 1;

fn main() -> ! {
    env_logger::init();
    Configuration::create_if_missing().unwrap();
    let configuration = Configuration::load();
    Renderer::<ResourceCollection, GuiApp, PluginCollection, HISTORY_DEPTH>::run(configuration);
}
