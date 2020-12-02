pub mod packet_receive_event;
pub mod serverbound_packet;
mod listener;

use kazyol_lib::server::Server;
use kazyol_lib::events::disable_event::DisableEvent;
use kazyol_lib::event::EventResult::Handled;
use kazyol_lib::event::EventType;

pub struct CustomEvent;

pub struct Plugin;

impl kazyol_lib::plugin::Plugin for Plugin {
    fn init() -> Box<Self> where Self: Sized {
        Box::new(Plugin)
    }

    fn on_enable(&self, server: &mut Server) {
        let (tx, rx) = std::sync::mpsc::channel();
        let (tx2, rx2) = std::sync::mpsc::channel();
        listener::start(tx, rx2);
    }

    fn get_name(&self) -> String {
        env!("CARGO_PKG_NAME").to_string()
    }

    fn get_description(&self) -> String {
        env!("CARGO_PKG_DESCRIPTION").to_string()
    }

    fn get_version(&self) -> String {
        env!("CARGO_PKG_VERSION").to_string()
    }

    fn get_authors(&self) -> Vec<String> {
        env!("CARGO_PKG_AUTHORS")
            .split(":")
            .map(ToString::to_string)
            .collect()
    }

    fn get_homepage(&self) -> Option<String> {
        None
    }

    fn get_repository(&self) -> String {
        "TO DO".to_string()
    }

    fn get_dependencies(&self) -> Vec<String> {
        Vec::new()
    }
}