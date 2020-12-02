# Kazyol
Kazyol is an open-source event-driven Rust Minecraft server implementation where everything is a plugin.

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
        Handled
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
    .dispatch_event(&mut Box::new(CustomEvent {
        data: "".to_string(),
    }));
```

# TO DO
- Generic plugin storage [memory]
`server.storage.get<MyDataType>()`
  
- Generic plugin storage [saved in binary file]
  `server.save_storage.get<MyDataType>()` give it better name than `save_storage`
  
- Scheduler - schedule a task to run on next tick (for async stuff), after 10 ticks or every 20 ticks

## Contributing
Currently, Kazyol is not even a working prototype, so any contribution will be appreciated.
If you want to help but don't know what to do, here are few ideas:
- Core plugins: packets, nbt, world, player, entities, chat, redstone, world generator
- Additional plugins: Proxy support (BungeeCord, Velocity, etc), BossBar API, Commands API, Queue
- Plugin manager and repository (something like Cargo/npm/pip, but without 95% of programming stuff, just for users)
- Any issue in [TO DO](https://github.com/Iaiao/Kazyol/projects/1)
