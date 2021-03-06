use std::any::TypeId;
use std::cell::RefCell;
use std::cmp::max;
use std::collections::HashMap;
use std::error::Error;
use std::time::{Duration, SystemTime};

use kazyol_api_method_macro::api_method;
use kazyol_lib::consts::TPS;
use kazyol_lib::event::EventResult::Handled;
use kazyol_lib::event::EventType;
use kazyol_lib::events::disable_event::DisableEvent;
use kazyol_lib::events::tick_event::TickEvent;
use kazyol_lib::plugin::Plugin;
use kazyol_lib::server::{Server, ENABLED};
use kazyol_lib::states::States;
use kazyol_lib::tracking;
use kazyol_lib::with_server;
use kazyol_lib::with_states;
use kazyol_plugin_loader::load_plugins;

fn main() {
    println!("Starting Kazyol");
    ctrlc::set_handler(move || {
        println!("Ctrl-C, shutting down");
        unsafe { ENABLED = false };
    })
    .expect("Error setting Ctrl-C handler");
    with_server!(|mut server: &mut Server| {
        let mut plugins: Vec<Box<dyn Plugin>> = Vec::new();
        tracking::name("kazyol".to_string());
        PluginManager::init().expect("Couldn't enable Plugin Manager.");
        tracking::pop();
        load_plugins!();

        server
            .events
            .register_event::<DisableEvent>(EventType::new());
        server.events.register_event::<TickEvent>(EventType::new());

        tracking::name("main".to_string());
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
                    panic!(
                        "Unresolved dependency for plugin {}: {}
  Help:
    - Download plugin `{}`
    - Remove {}
",
                        plugin.get_name(),
                        dep,
                        dep,
                        plugin.get_name()
                    )
                }
            }
            tracking::name(plugin.get_name());
            plugin.on_enable(&mut server);
            tracking::pop();
            println!(
                "Enabled plugin {} {}",
                plugin.get_name(),
                plugin.get_version()
            );
            server.plugins.push(plugin);
        }
    });

    run_tick_loop();

    with_server!(|server: &mut Server| {
        server
            .events
            .get::<DisableEvent>()
            .unwrap()
            .dispatch_event(&mut Box::new(DisableEvent));
    })
}

fn run_tick_loop() {
    let mut last_tick = SystemTime::now()
        .checked_sub(Duration::from_millis(1000 / TPS))
        .unwrap();
    let mut should_stop = false;
    loop {
        with_server!(|server: &mut Server| {
            if !unsafe { ENABLED } {
                should_stop = true;
            }
            server.events.get().unwrap().dispatch_event(&mut TickEvent);
        });
        if should_stop {
            break;
        }
        std::thread::sleep(Duration::from_millis(max(
            1000 / TPS as i64 - last_tick.elapsed().unwrap().as_millis() as i64,
            0,
        ) as u64));
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
    let n = plugins.get(&plugin).unwrap().len()
        + plugins.get(&plugin).unwrap().iter().fold(0, |val, plugin| {
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

struct PluginManager;

impl PluginManager {
    #[api_method("kazyol")]
    fn set_enabled<P>()
    where
        P: 'static,
    {
        with_states!(|states: &mut States| {
            states
                .get_mut::<Vec<TypeId>>()
                .unwrap()
                .push(TypeId::of::<P>())
        });
    }

    #[api_method("kazyol")]
    fn set_disabled<P>() -> Option<TypeId>
    where
        P: 'static,
    {
        with_states!(|states: &mut States| {
            let plugins = states.get_mut::<Vec<TypeId>>().unwrap();
            Some(plugins.remove(plugins.iter().position(|x| *x == TypeId::of::<P>())?))
        })
    }

    #[api_method("kazyol")]
    fn is_enabled<P>() -> bool
    where
        P: 'static,
    {
        with_states!(|states: &mut States| {
            states
                .get_mut::<Vec<TypeId>>()
                .unwrap()
                .contains(&TypeId::of::<P>())
        })
    }
}

impl Plugin for PluginManager {
    fn init() -> Result<Self, Box<dyn Error>>
    where
        Self: Sized,
    {
        with_states!(|states: &mut States| { states.set(Vec::<TypeId>::new()) });
        Ok(PluginManager)
    }

    fn get_name(&self) -> String {
        env!("CARGO_PKG_NAME").to_string()
    }

    fn get_description(&self) -> String {
        "Init plugin".to_string()
    }

    fn get_version(&self) -> String {
        env!("CARGO_PKG_VERSION").to_string()
    }

    fn get_authors(&self) -> Vec<String> {
        env!("CARGO_PKG_AUTHORS")
            .split(':')
            .map(ToString::to_string)
            .collect()
    }

    fn get_homepage(&self) -> Option<String> {
        None
    }

    fn get_repository(&self) -> String {
        "TO DO".to_string()
    }

    fn get_dependencies(&self) -> Vec<String> {
        Vec::new()
    }
}
