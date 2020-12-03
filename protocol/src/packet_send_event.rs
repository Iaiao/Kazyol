use crate::clientbound_packet::ClientboundPacket;

#[derive(Clone)]
pub struct PacketSendEvent {
    // TODO make events mutable and cancel if `packet` is None
    packet: Option<ClientboundPacket>,
    id: u64,
    pub(crate) handled: bool,
}

impl PacketSendEvent {
    pub fn new(packet: ClientboundPacket) -> PacketSendEvent {
        PacketSendEvent { packet: Some(packet), id: rand::random(), handled: false }
    }
    pub fn get_packet(&self) -> &Option<ClientboundPacket> {
        &self.packet
    }
    pub fn get_packet_mut(&mut self) -> &mut Option<ClientboundPacket> {
        &mut self.packet
    }
}