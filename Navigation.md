# Navigation
Here you can find what is plugin doing, how it is doing this and why.
I've never seen something like this in other repos, but I think it would
be very useful for contributors and random persons looking at the code.

## Plugins
### [Kazyol](kazyol)
Kazyol is a launcher. It handles some basic operations like ctrl-c,
loading plugins, TickEvent and other essential events. It's loading
plugins via [kazyol_plugin_loader](kazyol_plugin_loader/src/lib.rs)

### [Kazyol plugin loader](kazyol_plugin_loader)
A stupid simple procedural macro that reads a file named `plugins.txt`
and runs `<plugin_name>::Plugin::init()`

### [Kazyol lib](kazyol_lib)
Kazyol lib is the main thing every plugin is interacting with. It
handles event flow, tracking which plugin is currently running,
states and almost everything else.

### [Kazyol API method macro](kazyol_api_method_macro)
A simple boilerplate generator, which will run
`kazyol_lib::tracking::name(<name of the plugin>)` to make the method
access plugin's states, and will call `kazyol_lib::tracking::pop()`
after method returns. It uses pattern
[Finalisation in destructors](https://rust-unofficial.github.io/patterns/idioms/dtor-finally.html).

### [Protocol](protocol)
Protocol handles everything related to network and packets, it
runs a TCP listener, reads packets, dispatches
[PacketSendEvent](protocol/src/packet_send_event.rs) and
[PacketReceiveEvent](protocol/src/packet_receive_event.rs).
#### Understanding of Minecraft protocol
All packets are split to four [states](protocol/src/connection.rs#L8):
- Handshake (just one packet that client sends to the server)
- Status (server list status, icon, motd, online players, max players, ping)
- Login (server is verifying that client is not using a hacked client.
  Currently, it is skipped because the accounts will be migrated from Mojang
  to Microsoft and no one knows how it will be changed. If you like to, you can
  add those packets)
- Play (that's where the player is in a game world, everything interesting goes here)

### [Server list status](server_list_status)
This plugin handles Handshake and Status packets.

### [Player](player)
Handles Login and Play packets. In the future, it may split up to Login
and Player plugins.

### [Plugin Example](plugin_example)
Just an example plugin to show what you can do. It covers states, handling
events, creating, registering and dispatching custom events, API methods
and it's not very readable, maybe it would be better to make ~10 examples
and put them into examples/ directory, but that would be a bit hard to tell
newcomers to copy the example to the root directory, add it to Cargo.toml
and recompile. It will be changed once someone finds out better way to do
this or when the plugin manager will be done.

## Understanding
### States
States are like `HashMap<Plugin, HashMap<TypeId /*T*/, T>>`. Plugins
can access its states using `with_states!(|states: &mut States| { ... })`
macro. Under the hood, it's just like a thread_local UnsafeCell<HashMap>,
but the states are type of `dyn Any` and are downcasted to T.
Since everything is thread_local, there are no data races and UnsafeCell
is pretty safe. See the implementation at
[kazyol_lib/states.rs](kazyol_lib/src/states.rs)

### Tracking
Kazyol is tracking which plugin is currently running on the main thread.
Since everything with events and states is done on a main thread, main
thread is enough. [kazyol_lib/tracking.rs](kazyol_lib/src/tracking.rs)
contains thread_local Vec<String> which is used as a call stack, function
pop() to remove last element and name() to add an element to the end. The
last element on the stack is the currently running plugin's name.

### Events
Events are the main part of Kazyol, everything works because of events.
All events are dispatched and handled on the main thread. Everything
works like in States, but here the plugins are adding event handlers
(`fn(&mut E) -> EventResult` where E is an event) and when the event
arrives, every handler is invoked. Plugins can register and call their
own custom events Implementation: [kazyol_lib/event.rs](kazyol_lib/src/event.rs)

Here's an example event flow:

- `Kazyol` (launcher) enables plugin `Protocol`
- `Protocol` listens to incoming connections
- `Protocol` gets a packet and dispatches a `PacketReceiveEvent`
- `Server List Status` gets it and sends back to
- `Protocol` another packet
- `Protocol` dispatches `PacketSendEvent`
- Other plugins may do something else with the packet
- If the event isn't cancelled, `Protocol` sends the packet to a client.
- Once the client is logged in, some `Player` will send the player a
`KeepAlive` packet every X seconds, the client will respond and so on.