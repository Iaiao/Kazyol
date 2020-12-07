use crate::bytebuf::{ByteBufWrite, VarInt};
use crate::structs::{Chat, DimensionCodec, GameMode, Identifier};
use std::io::{Cursor, Result, Write};
use uuid::Uuid;
use crate::structs::dimension_codec::DimensionElement;

#[derive(Clone, Debug)]
pub enum ClientboundPacket {
    Response {
        json: String,
    },
    Pong {
        payload: i64,
    },
    DisconnectLogin {
        reason: Chat,
    },
    EncryptionRequest {
        server_id: String,
        public_key: Vec<u8>,
        verify_token: Vec<u8>,
    },
    LoginSuccess {
        uuid: Uuid,
        username: String,
    },
    SetCompression {
        threshold: VarInt,
    },
    LoginPluginRequest {
        message_id: VarInt,
        channel: Identifier,
        data: Vec<u8>,
    },
    JoinGame {
        entity_id: i32,
        is_hardcore: bool,
        game_mode: GameMode,
        // -1 if none
        previous_game_mode: i8,
        worlds: Vec<Identifier>,
        dimension_codec: DimensionCodec, // TODO
        dimension: DimensionElement,              // TODO
        world_name: Identifier,
        hashed_seed: i64,
        max_players: VarInt,
        view_distance: VarInt,
        reduced_debug_info: bool,
        enable_respawn_screen: bool,
        is_debug: bool,
        is_flat: bool,
    },
}

impl ClientboundPacket {
    pub fn write<T>(&self, mut write: T) -> Result<()>
    where
        T: Write,
    {
        let mut packet = Cursor::new(Vec::new());
        match self {
            ClientboundPacket::Response { json } => {
                packet.write_varint(0x00)?;
                packet.write_string(json)?;
            }
            ClientboundPacket::Pong { payload } => {
                packet.write_varint(0x01)?;
                packet.write_i64(*payload)?;
            }
            ClientboundPacket::LoginSuccess { username, uuid } => {
                packet.write_varint(0x02)?;
                packet.write_uuid(uuid)?;
                packet.write_string(username)?;
            }
            ClientboundPacket::JoinGame {
                entity_id,
                is_hardcore,
                game_mode,
                previous_game_mode,
                worlds,
                dimension_codec,
                dimension,
                world_name,
                hashed_seed,
                max_players,
                view_distance,
                reduced_debug_info,
                enable_respawn_screen,
                is_debug,
                is_flat,
            } => {
                packet.write_varint(0x25)?;
                packet.write_i32(*entity_id)?;
                packet.write_bool(*is_hardcore)?;
                packet.write_u8(game_mode.clone() as u8)?;
                packet.write_i8(*previous_game_mode)?;
                packet.write_varint(worlds.len() as VarInt)?;
                for world in worlds {
                    packet.write_identifier(world)?;
                }
                nbt::to_writer(&mut packet, dimension_codec, None).unwrap();
                nbt::to_writer(&mut packet, dimension, None).unwrap();
                packet.write_identifier(world_name)?;
                packet.write_i64(*hashed_seed)?;
                packet.write_varint(*max_players)?;
                packet.write_varint(*view_distance)?;
                packet.write_bool(*reduced_debug_info)?;
                packet.write_bool(*enable_respawn_screen)?;
                packet.write_bool(*is_debug)?;
                packet.write_bool(*is_flat)?;
            }
            _ => {
                println!("Unknown packet: {:?}", self);
                unimplemented!()
            }
        }
        let packet = packet.into_inner();
        write.write_varint(packet.len() as i32)?;
        write.write(&packet)?;
        Ok(())
    }
}

// TODO tests for each packet
