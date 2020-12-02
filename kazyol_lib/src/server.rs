use crate::event::Events;
use crate::plugin::Plugin;
use std::cell::{RefCell, RefMut};
use crate::events::disable_event::DisableEvent;
use std::time::SystemTime;

pub struct Server {
    pub events: Events,
    pub plugins: Vec<Box<dyn Plugin>>,
    pub started: SystemTime,
}

// For internal usage (handling ctrl-c) only
pub static mut ENABLED: bool = true;

impl Server {
    pub fn shutdown(&mut self) {
        self.events.get::<DisableEvent>().unwrap().dispatch_event(&Box::new(DisableEvent));
    }
}

thread_local! {
    pub static SERVER: RefCell<Server> = RefCell::new(Server {
        events: Events::new(),
        plugins: Vec::new(),
        started: SystemTime::now()
    })
}

#[macro_export]
macro_rules! with_server {
    ($block: expr) => {
        {
            kazyol_lib::server::SERVER.with(|server| {
                let server: RefMut<Server> = server.borrow_mut();
                $block(server)
            });
        }
    };
}

pub type Kazyol<'a> = RefMut<'a, Server>;