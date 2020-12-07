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

STATES.with(|states| {
    let states = states.borrow();
    // or
    let mut states = states.borrow_mut();

    states.set<State>(State { ... });
    let state: Option<&State> = states.get<State>(); // None if not set
    let state: Option<&mut State> = states.get_mut<State>(); // None if not set
    ...
})
```

## Contributing
Currently, Kazyol is not even a working prototype, so any contribution will be appreciated.
If you want to help but don't know what to do, here are few ideas:
- Core plugins: packets - [server](https://github.com/Iaiao/Kazyol/blob/main/protocol/src/serverbound_packet.rs), [client](https://github.com/Iaiao/Kazyol/blob/main/protocol/src/clientbound_packet.rs), [protocol](https://wiki.vg/Protocol) (Targeting [latest snapshot](https://wiki.vg/Pre-release_protocol)), nbt, world, player, entities, chat, redstone, world generator
- Additional plugins: Proxy support (BungeeCord, Velocity, etc), BossBar API, Commands API, Queue
- Plugin manager and repository (something like Cargo/npm/pip, but without 95% of programming stuff, just for users)
- State storage (saved in a binary file)
- Scheduler - schedule a task to run on next tick (for async stuff), after 10 ticks or every 20 ticks
- Any issue in [TO DO](https://github.com/Iaiao/Kazyol/projects/1)
