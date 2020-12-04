use crate::clientbound_packet::ClientboundPacket;

#[derive(Clone)]
pub struct PacketSendEvent {
    // TODO make events mutable and cancel if `packet` is None
    packet: Option<ClientboundPacket>,
    pub(crate) handled: bool,
}

impl PacketSendEvent {
    pub fn new(packet: ClientboundPacket) -> PacketSendEvent {
        PacketSendEvent {
            packet: Some(packet),
            handled: false,
        }
    }
    pub fn get_packet(&self) -> &Option<ClientboundPacket> {
        &self.packet
    }
    pub fn get_packet_mut(&mut self) -> &mut Option<ClientboundPacket> {
        &mut self.packet
    }
}
