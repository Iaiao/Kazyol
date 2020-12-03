pub mod packet_receive_event;
pub mod serverbound_packet;
pub mod bytebuf;
pub mod connection;
pub mod clientbound_packet;
pub mod packet_send_event;
mod listener;

use crate::listener::ListenerSendAction;
use kazyol_lib::server::Server;
use kazyol_lib::event::EventResult::Handled;
use kazyol_lib::event::EventType;
use kazyol_lib::with_server;
use std::sync::mpsc::Receiver;
use crate::packet_send_event::PacketSendEvent;
use crate::packet_receive_event::PacketReceiveEvent;
use kazyol_lib::events::tick_event::TickEvent;
use kazyol_lib::states::STATES;

pub struct CustomEvent;

pub struct Plugin;

impl kazyol_lib::plugin::Plugin for Plugin {
    fn init() -> Box<Self> where Self: Sized {
        Box::new(Plugin)
    }

    fn on_enable(&self, server: &mut Server) {
        let (tx, rx) = std::sync::mpsc::channel();
        let (_tx2, rx2) = std::sync::mpsc::channel();
        listener::start(tx, rx2);
        STATES.with(|states| {
            let mut states = states.borrow_mut();
            states.set::<Receiver<ListenerSendAction>>(rx);
        });
        server.events.register_event::<PacketSendEvent>(EventType::new());
        server.events.register_event::<PacketReceiveEvent>(EventType::new());
        server.events.get::<TickEvent>().unwrap().add_handler(|_| {
            STATES.with(|states| {
                let states = states.borrow();
                let rx = states.get::<Receiver<ListenerSendAction>>().unwrap();
                while let Ok(action) = rx.try_recv() {
                    with_server!(|server: &mut Server| {
                        match action {
                            ListenerSendAction::SendEvent(event) => {
                                let mut e = event.lock().expect("Unable to lock PacketSendEvent in main thread").clone();
                                let result = server.events.get().unwrap().dispatch_event(&e);
                                e.handled = true;
                                if result.is_cancelled() {
                                    *e.get_packet_mut() = None;
                                }
                                *event.lock().unwrap() = e;
                            }
                            ListenerSendAction::ReceiveEvent(event) => {
                                server.events.get().unwrap().dispatch_event(&event);
                            }
                        }
                    });
                }
            });
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