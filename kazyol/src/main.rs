use kazyol_plugin_loader::load_plugins;
use kazyol_lib::server::Server;
use kazyol_lib::event::{Events, EventType};
use kazyol_lib::plugin::Plugin;
use std::collections::HashMap;
use std::cell::RefCell;
use kazyol_lib::events::disable_event::DisableEvent;
use kazyol_lib::event::EventResult::Handled;

fn main() {
    println!("Starting Kazyol");
    let mut server = Server {
        events: Events::new(),
        plugins: Vec::new(),
    };
    let mut plugins: Vec<Box<dyn Plugin>> = Vec::new();
    load_plugins!();

    server
        .events
        .register_event::<DisableEvent>(EventType::new());

    kazyol_lib::tracking::name("main".to_string());
    server
        .events
        .get::<DisableEvent>()
        .unwrap()
        .add_handler(|_| {
            println!("Disabling kazyol");
            Handled
        });

    let names: Vec<String> = plugins.iter().map(|p| p.get_name()).collect();
    let mut deps = HashMap::new();
    plugins.iter().for_each(|plugin| {
        deps.insert(plugin.get_name(), plugin.get_dependencies());
    });
    plugins.sort_by_key(|a| dependencies(a.get_name(), &deps));
    for plugin in plugins {
        for dep in plugin.get_dependencies() {
            if !names.contains(&dep) {
                panic!("Unresolved dependency for plugin {}: {}
  Help:
    - Download plugin `{}`
    - Remove {}
", plugin.get_name(), dep, dep, plugin.get_name())
            }
        }
        plugin.on_enable(&mut server);
        println!(
            "Enabled plugin {} {}",
            plugin.get_name(),
            plugin.get_version()
        );
        server.plugins.push(plugin);
    }

    server
        .events
        .get::<DisableEvent>()
        .unwrap()
        .dispatch_event(&mut Box::new(DisableEvent));
}

thread_local! {
    static DEPENDENCY_STACK: RefCell<Vec<String>> = RefCell::new(Vec::new());
}

fn dependencies(plugin: String, plugins: &HashMap<String, Vec<String>>) -> usize {
    DEPENDENCY_STACK.with(|stack| {
        stack.borrow_mut().push(plugin.clone());
    });
    let n = plugins.get(&plugin.clone()).unwrap().len()
        + plugins.get(&plugin.clone()).unwrap().iter().fold(0, |val, plugin| {
        DEPENDENCY_STACK.with(|stack| {
            if stack.borrow().contains(plugin) {
                panic!(
                    "Stack overflow: Infinite dependency recursion with plugins {:?}",
                    stack
                );
            }
        });
        val + dependencies(plugin.clone(), plugins)
    });
    DEPENDENCY_STACK.with(|stack| {
        stack.borrow_mut().pop();
    });
    n
}