use std::sync::mpsc::{Sender, Receiver};
use std::thread;
use crate::packet_receive_event::PacketReceiveEvent;
use std::net::TcpListener;
use crate::serverbound_packet::ServerboundPacket;
use crate::connection::State;

pub enum ListenerAction {
//    SendPacket(String, ClientboundPacket)
}

pub(crate) fn start(_tx: Sender<PacketReceiveEvent>, _rx: Receiver<ListenerAction>) {
    thread::spawn(|| {
        let listener = TcpListener::bind("0.0.0.0:25565").unwrap();
        while let Ok((stream, address)) = listener.accept() {
            println!("Connection from {}", address);
            let packet = ServerboundPacket::deserialize(State::Handshake /* TODO Connection struct */, Box::new(stream));
            if let Ok(packet) = packet {
                println!("Packet: {:?}", packet)
            } else {
                println!("Warn: {}", packet.unwrap_err())
            }
        }
    });
}