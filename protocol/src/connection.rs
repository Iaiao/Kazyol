use crate::clientbound_packet::ClientboundPacket;
use crate::serverbound_packet::ServerboundPacket;
use std::net::TcpStream;
use std::sync::mpsc::{Receiver, Sender};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum State {
    Handshake,
    Status,
    Login,
    Play,
}

pub type PacketSender = Sender<ClientboundPacket>;

pub struct Connection {
    pub(crate) send: Receiver<ClientboundPacket>,
    pub(crate) receive: Sender<ServerboundPacket>,
    pub(crate) receive_set_state: Receiver<Option<State>>,
    pub state: State,
    pub stream: TcpStream,
    pub unique_id: Uuid,
}

impl Connection {
    pub(crate) fn receive(&mut self) -> bool {
        if let Ok(packet) = self.send.try_recv() {
            packet
                .write(&mut self.stream)
                .expect("Cannot send packet to player");
        }

        if let Ok(packet_size) = ServerboundPacket::get_size(&mut self.stream) {
            let packet = ServerboundPacket::read(self.state.clone(), &mut self.stream, packet_size);
            if let Ok(packet) = packet {
                self.received(packet);
            }
            true
        } else {
            println!("Connection closed");
            false
        }
    }
    pub(crate) fn received(&mut self, packet: ServerboundPacket) {
        self.receive
            .send(packet)
            .expect("Cannot send receive (?) packet");
        if let Some(state) = self
            .receive_set_state
            .recv()
            .expect("Cannot receive state change")
        {
            self.state = state;
        }
    }
}
