use crate::bytebuf::ByteBufRead;
use crate::connection::State;
use std::io::{Cursor, Error, ErrorKind, Read};

#[derive(Debug, Clone)]
pub enum ServerboundPacket {
    Handshake {
        protocol: i32,
        server_address: String,
        port: u16,
        state: HandshakeState,
    },
    Request {},
    Ping {
        payload: i64,
    },
}

impl ServerboundPacket {
    pub fn read_with_size<R>(state: State, mut read: R) -> Result<ServerboundPacket, std::io::Error>
    where
        R: Read,
    {
        let packet_size = Self::get_size(&mut read)?;
        Self::read(state, read, packet_size)
    }
    pub fn get_size<R>(mut read: R) -> Result<usize, std::io::Error>
    where
        R: Read,
    {
        let packet_size = read.read_varint()?;
        if packet_size < 0 {
            return Err(Error::new(ErrorKind::InvalidData, "Negative packet size"));
        }
        Ok(packet_size as usize)
    }
    pub fn read<R>(
        state: State,
        mut read: R,
        packet_size: usize,
    ) -> Result<ServerboundPacket, std::io::Error>
    where
        R: Read,
    {
        let mut buf = vec![0; packet_size];
        let read_bytes = read.read(&mut buf)?;
        if read_bytes != buf.len() {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Cannot read, stream ended",
            ));
        }
        let mut buf = Cursor::new(buf);

        // TODO compression

        let packet_id = buf.read_varint()?;
        match (state, packet_id) {
            (State::Handshake, 0x00) => {
                let packet = ServerboundPacket::Handshake {
                    protocol: buf.read_varint()?,
                    server_address: buf.read_string()?,
                    port: buf.read_u16()?,
                    state: match buf.read_varint()? {
                        1 => HandshakeState::Status,
                        2 => HandshakeState::Login,
                        _ => {
                            return Err(Error::new(
                                ErrorKind::InvalidData,
                                "Unknown handshake state",
                            ))
                        }
                    },
                };
                Ok(packet)
            }
            (State::Status, 0x00) => {
                let packet = ServerboundPacket::Request {};
                Ok(packet)
            }
            (State::Status, 0x01) => {
                let packet = ServerboundPacket::Ping {
                    payload: buf.read_i64()?,
                };
                Ok(packet)
            }
            _ => Err(Error::new(ErrorKind::InvalidData, "Unknown Packet ID")),
        }
    }
}

#[repr(i32)]
#[derive(Debug, Clone, PartialEq)]
pub enum HandshakeState {
    Status = 1,
    Login = 2,
}
