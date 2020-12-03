use std::sync::mpsc::{Sender, Receiver, channel};
use std::thread;
use std::net::TcpListener;
use crate::connection::{Connection, State};
use std::sync::{Mutex, Arc};
use crate::packet_send_event::PacketSendEvent;
use crate::packet_receive_event::PacketReceiveEvent;

pub enum ListenerReceiveAction {
//    SendPacket(Uuid, ClientboundPacket)
}

pub enum ListenerSendAction {
    SendEvent(Arc<Mutex<PacketSendEvent>>),
    ReceiveEvent(Arc<Mutex<PacketReceiveEvent>>),
}

pub(crate) fn start(tx: Sender<ListenerSendAction>, _rx: Receiver<ListenerReceiveAction>) {
    let connections = Arc::new(Mutex::new(Vec::new()));
    let connections2 = connections.clone();
    thread::spawn(move || {
        let listener = TcpListener::bind("0.0.0.0:25565").unwrap();
        while let Ok((stream, address)) = listener.accept() {
            println!("Connection from {}", address);
            let (send_tx, send_rx) = channel();
            let (receive_tx, receive_rx) = channel();
            let (send_confirm_tx, send_confirm_rx) = channel();
            let (receive_set_state_tx, receive_set_state_rx) = channel();
            let mut connection = Connection {
                send: send_tx,
                send_confirm: send_confirm_rx,
                receive: receive_tx,
                receive_set_state: receive_set_state_rx,
                state: State::Handshake,
                stream,
            };
            connections2.lock().unwrap().push((send_rx, send_confirm_tx, receive_rx, receive_set_state_tx));
            thread::spawn(move || {
                loop {
                    connection.receive();
                }
            });
        }
    });
    thread::spawn(move || {
        loop {
            for (send, send_confirm, receive, receive_set_state) in connections.lock().unwrap().iter() {
                if let Ok(packet) = send.try_recv() {
                    let event = Arc::new(Mutex::new(PacketSendEvent::new(packet)));
                    tx.send(ListenerSendAction::SendEvent(event.clone())).expect("Cannot send PacketSendEvent to main thread");
                    while !event.lock().expect("Unable to lock PacketSendEvent").handled {};
                    send_confirm.send(event.lock().unwrap().get_packet().clone()).expect("Cannot send packet between threads");
                }
                if let Ok(packet) = receive.try_recv() {
                    let event = Arc::new(Mutex::new(PacketReceiveEvent::new(packet)));
                    tx.send(ListenerSendAction::ReceiveEvent(event.clone())).expect("Cannot send PacketReceiveEvent to main thread");
                    while !event.lock().expect("Unable to lock PacketSendEvent").handled {};
                    receive_set_state.send(event.lock().unwrap().get_state_change()).expect("Cannot send packet between threads");
                }
            }
        }
    });
}