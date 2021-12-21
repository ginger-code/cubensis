use crate::rpc::messages::{RpcRequest, RpcResponse, RpcSeverity};
use hyphae::configuration::Configuration;
use hyphae::events::CubensisEvent;
use hyphae::plugins::CubensisRendererPlugin;
use std::net::TcpStream;
use tungstenite::{accept, Error, Message, Result};
use winit::event_loop::EventLoopProxy;
pub mod messages;

#[derive(Debug)]
pub struct RpcServer {
    configuration: Configuration,
    event_proxy: winit::event_loop::EventLoopProxy<CubensisEvent>,
    thread_handle: Option<std::thread::JoinHandle<()>>,
}

impl CubensisRendererPlugin for RpcServer {
    fn new(
        event_proxy: winit::event_loop::EventLoopProxy<CubensisEvent>,
        configuration: Configuration,
    ) -> Self {
        let event_proxy = event_proxy;
        let thread_handle = None;
        Self {
            configuration,
            event_proxy,
            thread_handle,
        }
    }

    fn handle_event(&mut self, _: &CubensisEvent) {}

    fn start(&mut self) {
        let address = self.configuration.network.get_address();
        if self.thread_handle.is_none() {
            log::debug!("Starting RPC websocket server");
            let event_proxy = self.event_proxy.clone();
            self.thread_handle = Some(std::thread::spawn(move || {
                log::debug!("Creating RPC thread");
                let listener =
                    std::net::TcpListener::bind(&*address).expect("Failed to bind to RPC address");
                log::debug!("RPC Server listening on {}", address);
                while let Ok((stream, _addr)) = listener.accept() {
                    let peer = stream
                        .peer_addr()
                        .expect("connected streams should have a peer address");
                    log::info!("Connection from: {}", peer);
                    let mut websocket = accept(stream).unwrap();
                    if let Err(e) = Self::handle_connection(peer, &mut websocket, &event_proxy) {
                        log::warn!("RPC error encountered");
                        match e {
                            Error::ConnectionClosed | Error::Io(_) => {
                                log::info!("RPC Client disconnected")
                            }
                            err => log::error!("Error processing connection: {:?}", err),
                        }
                    }
                }
            }));
        }
    }

    fn shutdown(&mut self) {
        self.thread_handle = None;
    }
}

impl RpcServer {
    fn handle_connection(
        peer: std::net::SocketAddr,
        websocket: &mut tungstenite::WebSocket<TcpStream>,
        event_proxy: &EventLoopProxy<CubensisEvent>,
    ) -> Result<()> {
        log::info!("New WebSocket connection: {}", peer);
        loop {
            let msg = websocket.read_message()?;
            match msg {
                Message::Text(ref text) => {
                    let request: Option<RpcRequest> = serde_json::from_str(text.as_str()).ok();
                    match request {
                        None => continue,
                        Some(command) => Self::handle_command(event_proxy, websocket, command)?,
                    }
                }
                Message::Binary(ref bin) => {
                    let request: Option<RpcRequest> = serde_json::from_slice(bin.as_slice()).ok();
                    match request {
                        None => continue,
                        Some(command) => Self::handle_command(event_proxy, websocket, command)?,
                    }
                }
                Message::Close(_) => {
                    log::info!("RPC Client disconnected");
                    return Ok(());
                }
                _ => continue,
            }
        }
    }
    fn handle_command(
        event_proxy: &EventLoopProxy<CubensisEvent>,
        sender: &mut tungstenite::WebSocket<TcpStream>,
        command: RpcRequest,
    ) -> Result<()> {
        log::debug!("Received RPC request: {:?}", command);

        match command {
            RpcRequest::SetProject { project_name } => {
                event_proxy
                    .send_event(CubensisEvent::SceneChange(project_name))
                    .unwrap();
                let response = RpcResponse::success(
                    Some("Successfully loaded scene".to_string()),
                    Some(RpcSeverity::Info),
                );
                let response = response.serialize().unwrap();
                sender.write_message(Message::Text(response))?;
            }
        }
        Ok(())
    }
}
