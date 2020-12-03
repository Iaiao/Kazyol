use crate::serverbound_packet::ServerboundPacket;

#[derive(Debug, Clone)]
pub struct PacketReceiveEvent {
    packet: ServerboundPacket
}

impl PacketReceiveEvent {
    pub fn new(packet: ServerboundPacket) -> PacketReceiveEvent {
        PacketReceiveEvent { packet }
    }
    pub fn get_packet(&self) -> &ServerboundPacket {
        &self.packet
    }
}