use hyphae::configuration::Configuration;
use hyphae::events::CubensisEvent;
use hyphae::plugins::{CubensisPluginCollection, CubensisRendererPlugin};
use spore::file_watcher::FileWatcher;
use spore::rpc::RpcServer;

pub struct PluginCollection {
    rpc: RpcServer,
    file_watcher: FileWatcher,
}

impl PluginCollection {}

impl CubensisPluginCollection for PluginCollection {
    fn new(
        event_proxy: winit::event_loop::EventLoopProxy<CubensisEvent>,
        configuration: Configuration,
    ) -> Self {
        log::debug!("Creating new plugin collection");
        let rpc = RpcServer::new(event_proxy.clone(), configuration.clone());
        let file_watcher = FileWatcher::new(event_proxy.clone(), configuration.clone());
        Self { rpc, file_watcher }
    }

    fn handle_event(&mut self, _event: &CubensisEvent) {
        log::trace!("Handling event in plugin collection");
    }

    fn start_all(&mut self) {
        log::debug!("Starting all plugins");
        self.rpc.start();
        self.file_watcher.start();
    }

    fn shutdown(&mut self) {
        log::debug!("Stopping all plugins");
        self.rpc.shutdown();
        self.file_watcher.shutdown();
    }
}
