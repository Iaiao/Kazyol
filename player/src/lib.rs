use kazyol_lib::event::EventResult::Handled;
use kazyol_lib::server::Server;
use kazyol_lib::states::States;
use kazyol_lib::with_states;
use protocol::clientbound_packet::ClientboundPacket;
use protocol::connection::State;
use protocol::listener::ConnectionHandle;
use protocol::packet_receive_event::PacketReceiveEvent;
use protocol::serverbound_packet::ServerboundPacket;
use protocol::structs::dimension_codec::{
    Biome, BiomeElement, Dimension, DimensionElement, DimensionType, Effects, MoodSound,
    WorldgenBiome,
};
use protocol::structs::{DimensionCodec, GameMode, HandshakeState, Identifier};

pub struct Plugin;

#[allow(dead_code)]
pub struct Player {
    name: String,
    handle: ConnectionHandle,
}

impl kazyol_lib::plugin::Plugin for Plugin {
    fn init() -> Box<Self>
    where
        Self: Sized,
    {
        Box::new(Plugin)
    }

    fn on_enable(&self, server: &mut Server) {
        with_states!(|states: &mut States| states.set::<Vec<Player>>(Vec::new()));
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
                            states.get_mut::<Vec<_>>().unwrap().push(Player {
                                name: name.to_string(),
                                handle: event.handle.clone(),
                            })
                        });
                        // TODO encryption
                        event.send_packet(ClientboundPacket::LoginSuccess {
                            uuid: event.handle.get_uuid(), // TODO change this to player's UUID
                            username: name.to_string(),
                        });
                        event.set_state(State::Play);
                        // TODO (hardcoded values)
                        event.send_packet(ClientboundPacket::JoinGame {
                            entity_id: 1,
                            is_hardcore: false,
                            game_mode: GameMode::Creative,
                            previous_game_mode: -1,
                            worlds: vec![
                                Identifier::new("minecraft", "world"),
                                Identifier::new("minecraft", "the_nether"),
                                Identifier::new("minecraft", "the_end"),
                            ],
                            dimension_codec: DimensionCodec {
                                dimension_type: DimensionType {
                                    r#type: "minecraft:dimension_type".to_string(),
                                    value: vec![Dimension {
                                        name: "minecraft:overworld".to_string(),
                                        id: 0,
                                        element: DimensionElement {
                                            piglin_safe: false,
                                            natural: true,
                                            ambient_light: 0.0,
                                            infiniburn: "minecraft:infiniburn_overworld"
                                                .to_string(),
                                            respawn_anchor_works: false,
                                            has_skylight: true,
                                            bed_works: true,
                                            effects: "minecraft:overworld".to_string(),
                                            has_raids: true,
                                            height: 256,
                                            logical_height: 256,
                                            min_y: 0,
                                            coordinate_scale: 1.0,
                                            ultrawarm: false,
                                            has_ceiling: false,
                                            fixed_time: false,
                                        },
                                    }],
                                },
                                biome: WorldgenBiome {
                                    r#type: "minecraft:worldgen/biome".to_string(),
                                    value: vec![Biome {
                                        name: "minecraft:plains".to_string(),
                                        id: 1,
                                        element: BiomeElement {
                                            precipitation: "rain".to_string(),
                                            effects: Effects {
                                                sky_color: 7907327,
                                                water_fog_color: 329011,
                                                fog_color: 12638463,
                                                water_color: 4159204,
                                                mood_sound: MoodSound {
                                                    tick_delay: 6000,
                                                    offset: 2.0,
                                                    sound: "minecraft:ambient.cave".to_string(),
                                                    block_search_extent: 8,
                                                },
                                            },
                                            depth: 0.125,
                                            temperature: 0.8,
                                            scale: 0.05,
                                            downfall: 0.4,
                                            category: "plains".to_string(),
                                        },
                                    }],
                                },
                            },
                            dimension: DimensionElement {
                                piglin_safe: false,
                                natural: true,
                                ambient_light: 0.0,
                                infiniburn: "minecraft:infiniburn_overworld".to_string(),
                                respawn_anchor_works: false,
                                has_skylight: true,
                                bed_works: true,
                                effects: "minecraft:overworld".to_string(),
                                has_raids: true,
                                height: 256,
                                logical_height: 256,
                                min_y: 0,
                                coordinate_scale: 1.0,
                                ultrawarm: false,
                                has_ceiling: false,
                                fixed_time: false,
                            },
                            world_name: Identifier::new("minecraft", "world"),
                            hashed_seed: 1,
                            max_players: 5,
                            view_distance: 4,
                            reduced_debug_info: false,
                            enable_respawn_screen: true,
                            is_debug: false,
                            is_flat: false,
                        });
                        event.send_packet(ClientboundPacket::PlayerAbilities {
                            invulnerable: true,
                            flying: true,
                            allow_flying: true,
                            instant_break: true,
                            flying_speed: 0.05,
                            field_of_view: 0.1,
                        })
                    }
                    ServerboundPacket::ClientSettings { locale, .. } => {
                        // Just to check if it works
                        println!("Player's language: {}", locale);
                        event.send_packet(ClientboundPacket::PlayerPositionAndLook {
                            x: 0.0,
                            y: 0.0,
                            z: 0.0,
                            yaw: 0.0,
                            pitch: 0.0,
                            x_is_relative: false,
                            y_is_relative: false,
                            z_is_relative: false,
                            yaw_is_relative: false,
                            pitch_is_relative: false,
                            teleport_id: 42,
                        })
                    }
                    ServerboundPacket::PlayerPosition { x, y, z, .. } => {
                        println!("Player moved to {} {} {}", x, y, z);
                    }
                    ServerboundPacket::PlayerPositionAndRotation {
                        x,
                        y,
                        z,
                        yaw,
                        pitch,
                        ..
                    } => {
                        println!(
                            "Player moved to {} {} {}, rotation: {}deg, {}deg",
                            x, y, z, yaw, pitch
                        );
                    }
                    ServerboundPacket::PlayerRotation { yaw, pitch, .. } => {
                        println!("Player rotation: {}deg {}deg", yaw, pitch);
                    }
                    ServerboundPacket::Animation { hand } => {
                        println!("Player swung {:?} handd", hand);
                    }
                    ServerboundPacket::TeleportConfirm { teleport_id } => {
                        println!("Teleport confirmed: {}", teleport_id);
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
