use crate::serverbound_packet::ServerboundPacket;
use crate::connection::State;

#[derive(Debug, Clone)]
pub struct PacketReceiveEvent {
    packet: ServerboundPacket,
    set_state: Option<State>,
    pub(crate) handled: bool
}

impl PacketReceiveEvent {
    pub fn new(packet: ServerboundPacket) -> PacketReceiveEvent {
        PacketReceiveEvent { packet, set_state: None, handled: false }
    }
    pub fn set_state(&mut self, state: State) {
        self.set_state = Some(state);
    }
    pub fn get_state_change(&self) -> Option<State> {
        self.set_state.clone()
    }
    pub fn get_packet(&self) -> &ServerboundPacket {
        &self.packet
    }
    pub fn get_packet_mut(&mut self) -> &ServerboundPacket {
        &mut self.packet
    }
}