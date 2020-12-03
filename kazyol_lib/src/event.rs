use std::collections::HashMap;
use std::any::{TypeId, Any};
use crate::tracking;
use crate::event::EventResult::Handled;

// TODO documentation
// Example usage:
// events.register_event(EventType::<Box<Event>>::new());
pub struct EventType<E> {
    handlers: HashMap<String, Vec<fn(&mut E) -> EventResult>>,
}

impl<E> EventType<E> {
    // Creates a new event type
    pub fn new() -> EventType<E> {
        EventType {
            handlers: HashMap::new()
        }
    }

    // Registers an event handler
    pub fn add_handler(&mut self, handler: fn(&mut E) -> EventResult) {
        tracking::PLUGINS.with(|stack| {
            self.get_plugin_handlers(stack.borrow().last().unwrap().clone()).push(handler);
        });
    }

    pub fn dispatch_event(&self, event: &mut E) -> EventDispatchResult {
        let mut results = Vec::new();
        for (plugin, handlers) in self.handlers.iter() {
            tracking::name(plugin.clone());
            for handler in handlers.iter() {
                results.push(handler(event));
            }
            tracking::pop();
        }
        EventDispatchResult::from(results)
    }

    fn get_plugin_handlers(&mut self, plugin: String) -> &mut Vec<fn(&mut E) -> EventResult> {
        if self.handlers.contains_key(&plugin) {
            self.handlers.get_mut(&plugin).unwrap()
        } else {
            let handlers = Vec::new();
            self.handlers.insert(plugin.clone(), handlers);
            self.handlers.get_mut(&plugin).unwrap()
        }
    }
}

pub struct Events {
    events: HashMap<TypeId, Box<dyn Any>>
}

impl Events {
    pub fn new() -> Events {
        Events { events: HashMap::new() }
    }
    pub fn get<E: 'static>(&mut self) -> Option<&mut EventType<E>> {
        match self.events.get_mut(&TypeId::of::<E>()) {
            None => None,
            Some(e) => {
                match e.downcast_mut() {
                    None => None,
                    Some(e) => Some(e)
                }
            }
        }
    }
    pub fn register_event<E: 'static>(&mut self, event_type: EventType<E>) {
        self.events.insert(TypeId::of::<E>(), Box::new(event_type));
    }
}

#[derive(Eq, PartialEq)]
pub enum EventResult {
    Handled,
    Cancelled,
}

pub struct EventDispatchResult {
    cancelled: bool
}

impl EventDispatchResult {
    pub fn is_cancelled(&self) -> bool {
        self.cancelled
    }
    pub fn set_cancelled(&mut self, cancelled: bool) {
        self.cancelled = cancelled;
    }
}

impl From<Vec<EventResult>> for EventDispatchResult {
    fn from(results: Vec<EventResult>) -> Self {
        EventDispatchResult {
            cancelled: results.contains(&Handled)
        }
    }
}