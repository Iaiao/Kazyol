pub mod bytebuf;
pub mod clientbound_packet;
pub mod connection;
pub mod listener;
pub mod packet_receive_event;
pub mod packet_send_event;
pub mod serverbound_packet;
pub mod structs;

use crate::listener::ListenerSendAction;
use crate::packet_receive_event::PacketReceiveEvent;
use crate::packet_send_event::PacketSendEvent;
use kazyol_lib::event::EventResult::Handled;
use kazyol_lib::event::EventType;
use kazyol_lib::events::tick_event::TickEvent;
use kazyol_lib::server::Server;
use kazyol_lib::states::States;
use kazyol_lib::with_server;
use kazyol_lib::with_states;
use std::sync::mpsc::Receiver;
use std::error::Error;

pub struct CustomEvent;

pub struct Plugin;

impl kazyol_lib::plugin::Plugin for Plugin {
    fn init() -> Result<Self, Box<dyn Error>>
    where
        Self: Sized,
    {
        Ok(Plugin)
    }

    fn on_enable(&self, server: &mut Server) {
        let (tx, rx) = std::sync::mpsc::channel();
        with_states!(|states: &mut States| {
            states.set(rx);
            states.set(listener::start(tx));
        });
        server
            .events
            .register_event::<PacketSendEvent>(EventType::new());
        server
            .events
            .register_event::<PacketReceiveEvent>(EventType::new());
        server.events.get::<TickEvent>().unwrap().add_handler(|_| {
            with_states!(|states: &mut States| {
                let rx = states.get::<Receiver<ListenerSendAction>>().unwrap();
                while let Ok(action) = rx.try_recv() {
                    with_server!(|server: &mut Server| {
                        match action {
                            ListenerSendAction::SendEvent(event) => {
                                let mut e = event
                                    .lock()
                                    .expect("Unable to lock PacketSendEvent in main thread")
                                    .clone();
                                let result = server.events.get().unwrap().dispatch_event(&mut e);
                                e.handled = true;
                                if result.is_cancelled() {
                                    *e.get_packet_mut() = None;
                                }
                                *event.lock().unwrap() = e;
                            }
                            ListenerSendAction::ReceiveEvent(event) => {
                                let mut e = event
                                    .lock()
                                    .expect("Unable to lock PacketReceiveEvent in main thread")
                                    .clone();
                                let _ = server.events.get().unwrap().dispatch_event(&mut e);
                                e.handled = true;
                                *event.lock().unwrap() = e;
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
