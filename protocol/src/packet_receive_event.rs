use crate::serverbound_packet::ServerboundPacket;

pub struct PacketReceiveEvent {
    packet: ServerboundPacket
}

impl PacketReceiveEvent {
    pub fn get_packet(&self) -> &ServerboundPacket {
        &self.packet
    }
}