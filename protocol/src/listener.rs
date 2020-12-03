use std::sync::mpsc::{Sender, Receiver, channel};
use std::thread;
use crate::packet_receive_event::PacketReceiveEvent;
use std::net::TcpListener;
use crate::connection::Connection;
use std::sync::{Mutex, Arc};

pub enum ListenerAction {
//    SendPacket(String, ClientboundPacket)
}

pub(crate) fn start(_tx: Sender<PacketReceiveEvent>, _rx: Receiver<ListenerAction>) {
    let connections = Arc::new(Mutex::new(Vec::new()));
    let connections2 = connections.clone();
    thread::spawn(move || {
        let listener = TcpListener::bind("0.0.0.0:25565").unwrap();
        while let Ok((stream, address)) = listener.accept() {
            println!("Connection from {}", address);
            let (send_tx, send_rx) = channel();
            let (receive_tx, receive_rx) = channel();
            let (send_confirm_tx, send_confirm_rx) = channel();
            let mut connection = Connection {
                send: send_tx,
                send_confirm: send_confirm_rx,
                receive: receive_tx,
                stream
            };
            connections2.lock().unwrap().push((send_rx, send_confirm_tx, receive_rx));
            thread::spawn(move || {
                loop {
                    connection.receive();
                }
            });
        }
    });
    thread::spawn(move || {
        loop {
            {
                for (send, send_confirm, receive) in connections.lock().unwrap().iter() {
                    if let Ok(packet) = send.try_recv() {
                        // TODO event
                        send_confirm.send(Some(packet)).expect("Cannot send packet between threads")
                    }
                    if let Ok(packet) = receive.try_recv() {
                        // TODO event
                        dbg!(packet);
                    }
                }
            }
        }
    });
}