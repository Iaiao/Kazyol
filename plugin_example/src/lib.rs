use kazyol_api_method_macro::api_method;
use kazyol_lib::event::EventResult::Handled;
use kazyol_lib::event::EventType;
use kazyol_lib::events::disable_event::DisableEvent;
use kazyol_lib::server::Server;
use kazyol_lib::states::States;
use kazyol_lib::tracking::PLUGINS;
use kazyol_lib::with_states;

pub struct CustomEvent;

pub struct Plugin;

#[derive(Debug)]
pub struct ExampleData {
    s: String,
    n: i32,
}

impl kazyol_lib::plugin::Plugin for Plugin {
    fn init() -> Box<Self>
    where
        Self: Sized,
    {
        println!("Hello, World!");
        Box::new(Plugin)
    }

    fn on_enable(&self, server: &mut Server) {
        server
            .events
            .register_event::<CustomEvent>(EventType::new());
        server
            .events
            .get::<CustomEvent>()
            .unwrap()
            .add_handler(|_| {
                println!("Got a custom event");
                Handled
            });
        let custom_event = server.events.get::<CustomEvent>().unwrap();
        custom_event.dispatch_event(&mut CustomEvent);

        server
            .events
            .get::<DisableEvent>()
            .unwrap()
            .add_handler(|_| {
                println!("Plugin disabled");
                Handled
            });

        with_states!(|states: &mut States| {
            states.set(ExampleData {
                s: "yes".to_string(),
                n: 1,
            })
        });

        API::api_example();
    }

    fn get_name(&self) -> String {
        env!("CARGO_PKG_NAME").to_string()
    }

    fn get_description(&self) -> String {
        env!("CARGO_PKG_DESCRIPTION").to_string()
    }

    fn get_version(&self) -> String {
        env!("CARGO_PKG_VERSION").to_string()
    }

    fn get_authors(&self) -> Vec<String> {
        env!("CARGO_PKG_AUTHORS")
            .split(":")
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

struct API;
impl API {
    #[api_method(env!("CARGO_PKG_NAME"))]
    pub fn api_example() {
        with_states!(|states: &mut States| {
            let data: &ExampleData = states.get().unwrap();
            println!("{:?}", data)
        });
    }
}
