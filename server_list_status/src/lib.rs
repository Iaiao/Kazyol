use std::fs::File;
use std::io::Read;

use kazyol_lib::event::EventResult::Handled;
use kazyol_lib::server::Server;
use kazyol_lib::states::States;
use kazyol_lib::with_states;
use protocol::clientbound_packet::ClientboundPacket;
use protocol::connection::State;
use protocol::packet_receive_event::PacketReceiveEvent;
use protocol::packet_send_event::PacketSendEvent;
use protocol::serverbound_packet::ServerboundPacket;
use protocol::structs::HandshakeState;

pub struct Plugin;

const MINECRAFT_VERSION: &'static str = "21w03a";
const PROTOCOL_VERSION: i32 = 0b01000000000000000000000000000000 + 11;
const SERVER_DESCRIPTION: &'static str = "Welcome to §9Kazyol§r!"; // TODO make it configurable

struct ImageBase64(String);

impl kazyol_lib::plugin::Plugin for Plugin {
    fn init() -> Box<Self>
    where
        Self: Sized,
    {
        let icon_file = File::open("server-icon.png");
        let image = if let Ok(mut icon_file) = icon_file {
            let mut bytes = vec![
                0;
                icon_file
                    .metadata()
                    .expect("Cannot get server-icon.png metadata")
                    .len() as usize
            ];
            icon_file
                .read(&mut bytes)
                .expect("Cannot read server-icon.png");
            base64::encode(bytes)
        } else {
            String::new()
        };
        with_states!(|states: &mut States| {
            states.set(ImageBase64(image));
        });
        Box::new(Plugin)
    }

    fn on_enable(&self, server: &mut Server) {
        server
            .events
            .get::<PacketReceiveEvent>()
            .expect("Protocol packet receive event not found")
            .add_handler(|event| {
                match event.get_packet() {
                    ServerboundPacket::Handshake { state, .. } => {
                        if *state == HandshakeState::Status {
                            event.set_state(State::Status);
                            with_states!(|states: &mut States| {
                                event.send_packet(ClientboundPacket::Response {
                                    json: format!(
                                        "{{
    \"version\": {{
        \"name\": \"{}\",
        \"protocol\": {}
    }},
    \"players\": {{
        \"max\": 100,
        \"online\": 1,
        \"sample\": [
            {{
                \"name\": \"ChunkDev\",
                \"id\": \"71f04c4c-47af-42fa-a27c-246a141e8326\"
            }}
        ]
    }},
    \"description\": {{
        \"text\": \"{}\"
    }},
    \"favicon\": \"data:image/png;base64,{}\"
}}",
                                        MINECRAFT_VERSION,
                                        PROTOCOL_VERSION,
                                        SERVER_DESCRIPTION,
                                        states.get::<ImageBase64>().unwrap().0
                                    ),
                                });
                            });
                        }
                    }
                    ServerboundPacket::Ping { payload } => {
                        event.send_packet(ClientboundPacket::Pong { payload: *payload })
                    }
                    _ => (),
                }
                Handled
            });
        server
            .events
            .get::<PacketSendEvent>()
            .unwrap()
            .add_handler(|_event| Handled)
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
