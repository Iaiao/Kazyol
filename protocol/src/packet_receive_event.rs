use crate::serverbound_packet::ServerboundPacket;
use crate::connection::State;
use crate::listener::ConnectionHandle;
use crate::clientbound_packet::ClientboundPacket;

#[derive(Debug, Clone)]
pub struct PacketReceiveEvent {
    packet: ServerboundPacket,
    set_state: Option<State>,
    /// ConnectionHandle is used to send packets
    /// You can also save it for further usage
    /// as it implements Clone and is thread-safe
    pub handle: ConnectionHandle,
    pub(crate) handled: bool,
}

impl PacketReceiveEvent {
    pub fn new(packet: ServerboundPacket, handle: ConnectionHandle) -> PacketReceiveEvent {
        PacketReceiveEvent { packet, set_state: None, handle, handled: false }
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
    // Send reply packet to player
    pub fn send_packet(&self, packet: ClientboundPacket) {
        self.handle.send(packet, true);
    }
}