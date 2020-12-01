use crate::event::Events;
use crate::plugin::Plugin;

pub struct Server {
    pub events: Events,
    pub plugins: Vec<Box<dyn Plugin>>,
}