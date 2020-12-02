use kazyol_plugin_loader::load_plugins;
use kazyol_lib::server::{Server, Kazyol};
use kazyol_lib::event::EventType;
use kazyol_lib::plugin::Plugin;
use std::collections::HashMap;
use std::cell::{RefCell, RefMut};
use kazyol_lib::events::disable_event::DisableEvent;
use kazyol_lib::event::EventResult::Handled;
use kazyol_lib::events::tick_event::TickEvent;
use std::time::{SystemTime, Duration};
use kazyol_lib::consts::TPS;
use std::cmp::max;
use kazyol_lib::with_server;

fn main() {
    println!("Starting Kazyol");
    with_server! (|mut server: Kazyol| {
        let mut plugins: Vec<Box<dyn Plugin>> = Vec::new();
        load_plugins!();

        server
            .events
            .register_event::<DisableEvent>(EventType::new());
        server
            .events
            .register_event::<TickEvent>(EventType::new());

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

    });

    run_tick_loop();

    with_server!(|mut server: Kazyol| {
        server
            .events
            .get::<DisableEvent>()
            .unwrap()
            .dispatch_event(&mut Box::new(DisableEvent));
    })
}

fn run_tick_loop() {
    let mut last_tick = SystemTime::now().checked_sub(Duration::from_millis(1000 / TPS)).unwrap();
    loop {
        with_server! (|mut server: Kazyol|{
            server.events.get().unwrap().dispatch_event(&Box::new(TickEvent));
        });
        std::thread::sleep(Duration::from_millis(max(1000 / TPS as i64 - last_tick.elapsed().unwrap().as_millis() as i64, 0) as u64));
        last_tick = SystemTime::now();
    }
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