use kazyol_lib::event::EventResult::Handled;
use kazyol_lib::server::Server;
use kazyol_lib::states::States;
use kazyol_lib::with_states;
use protocol::clientbound_packet::ClientboundPacket;
use protocol::connection::State;
use protocol::listener::ConnectionHandle;
use protocol::packet_receive_event::PacketReceiveEvent;
use protocol::serverbound_packet::{HandshakeState, ServerboundPacket};
use std::collections::HashMap;

pub struct Plugin;

impl kazyol_lib::plugin::Plugin for Plugin {
    fn init() -> Box<Self>
    where
        Self: Sized,
    {
        Box::new(Plugin)
    }

    fn on_enable(&self, server: &mut Server) {
        with_states!(
            |states: &mut States| states.set::<HashMap<String, ConnectionHandle>>(HashMap::new())
        );
        server
            .events
            .get::<PacketReceiveEvent>()
            .expect("Protocol packet receive event not found")
            .add_handler(|event| {
                match event.get_packet() {
                    ServerboundPacket::Handshake { state, .. } => {
                        if *state == HandshakeState::Login {
                            event.set_state(State::Login);
                        }
                    }
                    ServerboundPacket::LoginStart { name } => {
                        with_states!(|states: &mut States| {
                            states
                                .get_mut::<HashMap<String, ConnectionHandle>>()
                                .unwrap()
                                .insert(name.clone(), event.handle.clone())
                        });
                        // TODO encryption
                        event.send_packet(ClientboundPacket::LoginSuccess {
                            uuid: event.handle.get_uuid(), // TODO change this to player's UUID
                            username: name.to_string(),
                        })
                    }
                    _ => (),
                }
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
