use crate::bytebuf::{ByteBufWrite, VarInt};
use crate::structs::{Chat, Identifier};
use std::io::{Cursor, Result, Write};
use uuid::Uuid;

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
            _ => unimplemented!(),
        }
        let packet = packet.into_inner();
        write.write_varint(packet.len() as i32)?;
        write.write(&packet)?;
        Ok(())
    }
}

// TODO tests for each packet
