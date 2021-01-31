use crate::clientbound_packet::ClientboundPacket;
use crate::connection::{Connection, State};
use crate::packet_receive_event::PacketReceiveEvent;
use crate::packet_send_event::PacketSendEvent;
use kazyol_lib::server::Server;
use kazyol_lib::with_server;
use std::net::TcpListener;
use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use uuid::Uuid;

pub enum ListenerSendAction {
    #[allow(dead_code)]
    SendEvent(Arc<Mutex<PacketSendEvent>>),
    ReceiveEvent(Arc<Mutex<PacketReceiveEvent>>),
}

pub(crate) fn start(tx: Sender<ListenerSendAction>) {
    let connections = Arc::new(Mutex::new(Vec::new()));
    let connections2 = connections.clone();
    let uuids = Vec::new(); // TODO get them from Mojang API
    thread::spawn(move || {
        let listener = TcpListener::bind("0.0.0.0:25565").unwrap();
        while let Ok((stream, address)) = listener.accept() {
            println!("Connection from {}", address);
            let (send_tx, send_rx) = channel();
            let (receive_tx, receive_rx) = channel();
            let (receive_set_state_tx, receive_set_state_rx) = channel();
            let mut unique_id = Uuid::new_v4();
            while uuids.contains(&unique_id) {
                unique_id = Uuid::new_v4();
            }
            let mut connection = Connection {
                send: send_rx,
                receive: receive_tx,
                receive_set_state: receive_set_state_rx,
                state: State::Handshake,
                stream,
                unique_id,
            };
            connections2.lock().unwrap().push((
                ConnectionHandle(send_tx, receive_set_state_tx, unique_id),
                receive_rx,
            ));
            thread::spawn(move || while connection.tick() {});
        }
    });
    thread::spawn(move || loop {
        for (handle, receive) in connections.lock().unwrap().iter() {
            if let Ok(packet) = receive.try_recv() {
                let event = Arc::new(Mutex::new(PacketReceiveEvent::new(packet, handle.clone())));
                tx.send(ListenerSendAction::ReceiveEvent(event.clone()))
                    .expect("Cannot send PacketReceiveEvent to main thread");
                while !event
                    .lock()
                    .expect("Unable to lock PacketSendEvent")
                    .handled
                {}
                handle
                    .1
                    .send(event.lock().unwrap().get_state_change())
                    .expect("Cannot send packet between threads");
            }
        }
    });
}

#[derive(Debug, Clone)]
pub struct ConnectionHandle(Sender<ClientboundPacket>, Sender<Option<State>>, Uuid);

impl ConnectionHandle {
    pub fn get_send(&self) -> &Sender<ClientboundPacket> {
        &self.0
    }
    #[allow(dead_code)]
    pub(crate) fn get_state_send(&self) -> &Sender<Option<State>> {
        &self.1
    }
    pub fn get_uuid(&self) -> Uuid {
        self.2
    }
    pub fn send(&self, packet: ClientboundPacket, event: bool) {
        let mut packet = Some(packet);
        with_server!(|server: &mut Server| {
            if event {
                let mut event = PacketSendEvent::new(packet.unwrap());
                server.events.get().unwrap().dispatch_event(&mut event);
                packet = event.get_packet().clone();
            }
            if let Some(packet) = packet {
                self.0.send(packet).expect("Cannot send packet to player");
            }
        });
    }
}

unsafe impl Send for ConnectionHandle {}
