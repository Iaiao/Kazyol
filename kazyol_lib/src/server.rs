use crate::event::Events;
use crate::plugin::Plugin;
use std::cell::{RefCell, RefMut};

pub struct Server {
    pub events: Events,
    pub plugins: Vec<Box<dyn Plugin>>,
}

thread_local! {
    pub static SERVER: RefCell<Server> = RefCell::new(Server {
        events: Events::new(),
        plugins: Vec::new()
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