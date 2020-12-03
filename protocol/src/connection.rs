use std::sync::mpsc::{Sender, Receiver, SendError};
use crate::clientbound_packet::ClientboundPacket;
use crate::serverbound_packet::ServerboundPacket;
use std::net::TcpStream;

pub enum State {
    Handshake,
    Status,
    Login,
    Play,
}

pub struct Connection {
    pub(crate) send: Sender<ClientboundPacket>,
    pub(crate) send_confirm: Receiver<Option<ClientboundPacket>>,
    pub(crate) receive: Sender<ServerboundPacket>,
    pub stream: TcpStream,
}

impl Connection {
    pub fn send_packet(&self, packet: ClientboundPacket) {
        self.send.send(packet).expect("Cannot send packet");
        if let Some(packet) = self.send_confirm.recv().expect("Cannot receive packet confirmation") {
            // TODO send packet
        }
    }
    pub(crate) fn receive(&mut self) {
        let packet = ServerboundPacket::deserialize(State::Handshake, &mut self.stream);
        if let Ok(packet) = packet {
            self.received(packet).unwrap();
        } else {
            //println!("Warn: {}", packet.unwrap_err())
        }
    }
    pub(crate) fn received(&self, packet: ServerboundPacket) -> Result<(), SendError<ServerboundPacket>> {
        self.receive.send(packet)
    }
}

