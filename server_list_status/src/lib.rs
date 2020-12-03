use kazyol_lib::server::Server;
use kazyol_lib::events::disable_event::DisableEvent;
use kazyol_lib::event::EventResult::Handled;
use kazyol_lib::event::EventType;
use protocol::*;
use protocol::packet_receive_event::PacketReceiveEvent;

pub struct CustomEvent;

pub struct Plugin;

impl kazyol_lib::plugin::Plugin for Plugin {
    fn init() -> Box<Self> where Self: Sized {
        println!("Hello, World!");
        Box::new(Plugin)
    }

    fn on_enable(&self, server: &mut Server) {
        server.events.get::<PacketReceiveEvent>().expect("Protocol packet receive event not found").add_handler(|e| {
            dbg!(e.get_packet());
            Handled
        });
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