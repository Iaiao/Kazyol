use std::io::{Write, Cursor, Result};
use crate::bytebuf::ByteBufWrite;

#[derive(Clone)]
pub enum ClientboundPacket {
    Response {
        json: String
    }
}

impl ClientboundPacket {
    pub fn write<T>(&self, mut write: T) -> Result<()> where T: Write {
        let mut packet = Cursor::new(Vec::new());
        match self {
            ClientboundPacket::Response { json } => {
                packet.write_varint(0x00)?;
                packet.write_string(json)?;
            }
        }
        let packet = packet.into_inner();
        write.write_varint(packet.len() as i32)?;
        write.write(&packet)?;
        Ok(())
    }
}

// TODO tests for each packet