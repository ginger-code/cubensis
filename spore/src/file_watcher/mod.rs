use hyphae::configuration::Configuration;
use hyphae::events::CubensisEvent;
use hyphae::events::CubensisEvent::FileEdit;
use hyphae::plugins::CubensisRendererPlugin;
use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use std::sync::mpsc::channel;
use std::time::Duration;
use winit::event_loop::EventLoopProxy;

pub struct FileWatcher {
    _configuration: Configuration, //planned
    event_proxy: winit::event_loop::EventLoopProxy<CubensisEvent>,
    thread_handle: Option<std::thread::JoinHandle<()>>,
}

impl FileWatcher {}

impl CubensisRendererPlugin for FileWatcher {
    fn new(event_proxy: EventLoopProxy<CubensisEvent>, _configuration: Configuration) -> Self {
        let event_proxy = event_proxy;
        let thread_handle = None;
        Self {
            _configuration,
            event_proxy,
            thread_handle,
        }
    }

    fn handle_event(&mut self, _: &CubensisEvent) {}

    fn start(&mut self) {
        if self.thread_handle.is_none() {
            log::debug!("Starting file watcher");
            let event_proxy = self.event_proxy.clone();
            self.thread_handle = Some(std::thread::spawn(move || {
                let (tx, rx) = channel();
                let mut watcher = watcher(tx, Duration::from_millis(500)).unwrap();
                watcher
                    .watch(Configuration::config_directory(), RecursiveMode::Recursive)
                    .unwrap();
                loop {
                    match rx.recv() {
                        Ok(event) => match event {
                            DebouncedEvent::Create(path) | DebouncedEvent::Write(path) => {
                                log::debug!("Detected change to file '{:?}'", &path);
                                event_proxy.send_event(FileEdit(path)).unwrap();
                            }
                            _ => {}
                        },
                        Err(e) => log::warn!("File watcher error: {}", e.to_string()),
                    }
                }
            }));
        }
    }

    fn shutdown(&mut self) {
        self.thread_handle = None;
    }
}
