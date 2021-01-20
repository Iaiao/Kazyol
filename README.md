[![Build Status](https://travis-ci.org/Iaiao/Kazyol.svg?branch=main)](https://travis-ci.org/Iaiao/Kazyol)
# Kazyol
Kazyol is an open-source event-driven Minecraft server implementation where everything is a plugin.

# For server owners
It's not ready for production yet. If you want to help, you can translate it to your language [not yet],
become a patron [not yet] or test and report issues [not much to test yet].

# For developers
This server has two basic concepts: Events and Plugins
## Plugins
Create a Plugin struct that implements the kazyol_lib::plugin::Plugin trait
```rust
pub struct Plugin;
impl kazyol_lib::plugin::Plugin for Plugin { ... }
```
Add it to kazyol's Cargo.toml and plugins.txt [TODO auto-scan them in Cargo.toml]

Note: everything below must be called on main thread. You can use `mpsc` and try_recv on TickEvent to call it
from other threads.
## Events
Plugin can register its own event and listen to others. For example, TickEvent, DisableEvent, EntityMoveEvent
### Listening to events
```rust
server
    .events
    .get::<DisableEvent>()
    .unwrap()
    .add_handler(|_| {
        println!("Plugin disabled");
        Handled // can also be Cancelled for cancellable events
    });
```
### Registering a custom event
```rust
struct CustomEvent {
    data: String
}

server
    .events
    .register_event::<CustomEvent>(EventType::new());
```
### Dispatching an event
```rust
server
    .events
    .get::<DisableEvent>()
    .unwrap()
    .dispatch_event(&CustomEvent {
        data: "".to_string(),
    });
```
### How to get the `server`?
```rust
with_server!(|server: &mut Server| { /* ... */ });
```
### States
Your plugin can have multiple singleton states.
It can be HashMap, Vec or even your custom Struct/Enum/whatever
```rust
struct State { ... }

with_states!(|states: &mut States| {
    states.set<State>(State { ... });
    let state: Option<&State> = states.get<State>(); // None if not set
    let state: Option<&mut State> = states.get_mut<State>(); // None if not set
    ...
});
```

## Contributing
Currently, Kazyol is a poorly working prototype, the core features
are working, but there is a lack of plugins. The current TO DO list is:
- Packets in [protocol](protocol) ([Project #2](https://github.com/Iaiao/Kazyol/projects/2))
- World
- Block
- ItemStack
- Entities
- Redstone
- World Generator
- Inventories, chests
After completing these core features, it would be great to make a high level API:
- Minimize boilerplate
- BossBar API
- NBT Api
- Block/Entity/ItemStack storage, saving data, persistent states, etc
- Custom entity API / AI

This list is too long and maybe everything will be done in ~2042.

Before trying to get into the code, you may want to read [Navigation](Navigation.md),
which contains description and implementation details of each plugin and feature.