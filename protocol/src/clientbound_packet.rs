use crate::bytebuf::ByteBufWrite;
use std::io::{Cursor, Result, Write};

#[derive(Clone, Debug)]
pub enum ClientboundPacket {
    Response { json: String },
    Pong { payload: i64 },
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
        }
        let packet = packet.into_inner();
        write.write_varint(packet.len() as i32)?;
        write.write(&packet)?;
        Ok(())
    }
}

// TODO tests for each packet
