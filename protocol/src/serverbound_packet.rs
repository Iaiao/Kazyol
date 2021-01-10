use crate::bytebuf::{ByteBufRead, VarInt};
use crate::connection::State;
use crate::structs::{Hand, HandSide, HandshakeState};
use std::io::{Cursor, Error, ErrorKind, Read};

#[derive(Debug, Clone)]
pub enum ServerboundPacket {
    Handshake {
        protocol: VarInt,
        server_address: String,
        port: u16,
        state: HandshakeState,
    },
    Request {},
    Ping {
        payload: i64,
    },
    LoginStart {
        name: String,
    },
    EncryptionResponse {
        shared_secret: Vec<u8>,
        verify_token: Vec<u8>,
    },
    LoginPluginResponse {
        message_id: VarInt,
        successful: bool,
        data: Option<Vec<u8>>,
    },
    ClientSettings {
        locale: String,
        view_distance: i8,
        chat_mode: VarInt,
        chat_colors: bool,
        skin_parts: u8, // TODO maybe make a struct?
        main_hand: HandSide,
    },
    PlayerPosition {
        x: f64,
        y: f64,
        z: f64,
        on_ground: bool,
    },
    PlayerPositionAndRotation {
        x: f64,
        y: f64,
        z: f64,
        yaw: f32,
        pitch: f32,
        on_ground: bool,
    },
    PlayerRotation {
        yaw: f32,
        pitch: f32,
        on_ground: bool,
    },
    Animation {
        hand: Hand,
    },
    TeleportConfirm {
        teleport_id: VarInt,
    },
    KeepAlive {
        id: i64,
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
                            ));
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
            (State::Login, 0x00) => {
                let packet = ServerboundPacket::LoginStart {
                    name: buf.read_string()?,
                };
                Ok(packet)
            }
            (State::Play, 0x05) => {
                let packet = ServerboundPacket::ClientSettings {
                    locale: buf.read_string()?,
                    view_distance: buf.read_i8()?,
                    chat_mode: buf.read_varint()?,
                    chat_colors: buf.read_bool()?,
                    skin_parts: buf.read_u8()?,
                    main_hand: if buf.read_varint()? == 0 {
                        HandSide::Left
                    } else {
                        HandSide::Right
                    },
                };
                Ok(packet)
            }
            (State::Play, 0x12) => {
                let packet = ServerboundPacket::PlayerPosition {
                    x: buf.read_f64()?,
                    y: buf.read_f64()?,
                    z: buf.read_f64()?,
                    on_ground: buf.read_bool()?,
                };
                Ok(packet)
            }
            (State::Play, 0x13) => {
                let packet = ServerboundPacket::PlayerPositionAndRotation {
                    x: buf.read_f64()?,
                    y: buf.read_f64()?,
                    z: buf.read_f64()?,
                    yaw: buf.read_f32()?,
                    pitch: buf.read_f32()?,
                    on_ground: buf.read_bool()?,
                };
                Ok(packet)
            }
            (State::Play, 0x14) => {
                let packet = ServerboundPacket::PlayerRotation {
                    yaw: buf.read_f32()?,
                    pitch: buf.read_f32()?,
                    on_ground: buf.read_bool()?,
                };
                Ok(packet)
            }
            (State::Play, 0x2C) => {
                let packet = ServerboundPacket::Animation {
                    hand: if buf.read_varint()? == 0 {
                        Hand::Main
                    } else {
                        Hand::Off
                    },
                };
                Ok(packet)
            }
            (State::Play, 0x00) => {
                let packet = ServerboundPacket::TeleportConfirm {
                    teleport_id: buf.read_varint()?,
                };
                Ok(packet)
            }
            (State::Play, 0x0B) => {
                println!("Got plugin message. Ignoring this for now");
                Err(Error::new(ErrorKind::Other, "Ignoring plugin message.")) // TODO PluginMessageEvent
            }
            (State::Play, 0x10) => {
                let packet = ServerboundPacket::KeepAlive {
                    id: buf.read_i64()?,
                };
                Ok(packet)
            }
            _ => {
                #[cfg(debug_assertions)]
                println!("Unknown packet: {}", packet_id);
                Err(Error::new(ErrorKind::InvalidData, "Unknown Packet ID"))
            }
        }
    }
}
