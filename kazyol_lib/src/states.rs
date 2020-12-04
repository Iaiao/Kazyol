use crate::tracking;
use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::HashMap;

pub struct States {
    states: HashMap<String, HashMap<TypeId, Box<dyn Any>>>,
}

thread_local! {
    pub static STATES: RefCell<States> = RefCell::new(States::new());
}

impl States {
    pub fn new() -> States {
        States {
            states: HashMap::new(),
        }
    }
    pub fn set<T: 'static>(&mut self, state: T) {
        let mut plugin = String::new();
        tracking::PLUGINS.with(|stack| {
            plugin = stack.borrow().last().unwrap().clone();
        });
        if !self.states.contains_key(&plugin) {
            self.states.insert(plugin.clone(), HashMap::new());
        }
        let plugin_states = self.states.get_mut(&plugin).unwrap();
        plugin_states.insert(TypeId::of::<T>(), Box::new(state));
    }
    pub fn get<T: 'static>(&self) -> Option<&T> {
        let mut plugin = String::new();
        tracking::PLUGINS.with(|stack| {
            plugin = stack.borrow().last().unwrap().clone();
        });
        match self.states.get(&plugin) {
            None => None,
            Some(map) => match map.get(&TypeId::of::<T>()) {
                None => None,
                Some(e) => match e.downcast_ref() {
                    None => None,
                    Some(e) => Some(e),
                },
            },
        }
    }
    pub fn get_mut<T: 'static>(&mut self) -> Option<&mut T> {
        let mut plugin = String::new();
        tracking::PLUGINS.with(|stack| {
            plugin = stack.borrow().last().unwrap().clone();
        });
        println!("Plugin: {}", plugin);
        match self.states.get_mut(&plugin) {
            None => None,
            Some(map) => match map.get_mut(&TypeId::of::<T>()) {
                None => None,
                Some(e) => match e.downcast_mut() {
                    None => None,
                    Some(e) => Some(e),
                },
            },
        }
    }
}
