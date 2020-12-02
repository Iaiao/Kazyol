use std::sync::mpsc::{Sender, Receiver};
use std::thread;
use crate::packet_receive_event::PacketReceiveEvent;
use std::net::TcpListener;

pub enum ListenerAction {
//    SendPacket(String, ClientboundPacket)
}

pub(crate) fn start(tx: Sender<PacketReceiveEvent>, rx: Receiver<ListenerAction>) {
    thread::spawn(|| {
        let listener = TcpListener::bind("0.0.0.0:25565").unwrap();
        while let Ok((stream, address)) = listener.accept() {
            println!("Connection from {}", address)
        }
    });
}