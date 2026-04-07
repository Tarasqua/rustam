use std::{
    io::stdin,
    mem,
    sync::mpsc::{self, Sender},
    thread::{self, JoinHandle},
};

fn main() {
    let mut system = System::default();

    // Each actor runs on its own thread and returns a Sender for its mailbox.
    let pong_tx = system.run(PongActor::new(String::from("Alice")));
    let ping_tx = system.run(PingActor::new(String::from("Bob"), pong_tx));
    let input_tx = system.run(InputActor::new(ping_tx));

    // InputActor starts reading from stdin only after it receives Start.
    input_tx.send(InputMessage::Start).unwrap();
}

/// Reads user input and forwards it to the PingActor.
///
/// This actor is a bit unusual because one message (`Start`) begins a long loop.
/// That is fine for a demo, but in a more "pure" actor system you usually avoid
/// blocking on stdin inside `process_message` because the actor cannot process any
/// other messages while it is waiting for input.
struct InputActor {
    ping_tx: Sender<PingMessage>,
}

impl InputActor {
    pub fn new(ping_tx: Sender<PingMessage>) -> Self {
        Self { ping_tx }
    }
}

enum InputMessage {
    Start,
}

impl Actor for InputActor {
    type Message = InputMessage;

    fn process_message(self, msg: Self::Message) -> Option<Self> {
        match msg {
            InputMessage::Start => loop {
                println!();
                println!("Enter message for ping-pong (`exit` to stop):");

                let mut input = String::new();
                stdin().read_line(&mut input).ok()?;

                let input = input.trim().to_string();
                if input == "exit" {
                    return None;
                }

                // Send the text into PingActor's mailbox.
                self.ping_tx.send(PingMessage::new(input)).ok()?;
            },
        }
    }
}

// PING

/// Receives a PingMessage, prints it, and forwards the payload to PongActor.
struct PingActor {
    name: String,
    pong_tx: Sender<PongMessage>,
}

impl PingActor {
    pub fn new(name: String, pong_tx: Sender<PongMessage>) -> Self {
        Self { name, pong_tx }
    }
}

struct PingMessage(String);

impl PingMessage {
    pub fn new(text: String) -> Self {
        Self(text)
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

impl Actor for PingActor {
    type Message = PingMessage;

    fn process_message(self, msg: Self::Message) -> Option<Self> {
        let text = msg.into_string();
        println!("{} received ping: {}", self.name, text);

        // The actor keeps ownership of itself by returning `Some(self)`.
        // Returning `None` would stop the actor thread.
        self.pong_tx.send(PongMessage::new(text)).ok()?;
        Some(self)
    }
}

// PONG

/// Receives the forwarded message and prints the final "pong" response.
struct PongActor {
    name: String,
}

impl PongActor {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

struct PongMessage(String);

impl PongMessage {
    pub fn new(text: String) -> Self {
        Self(text)
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

impl Actor for PongActor {
    type Message = PongMessage;

    fn process_message(self, msg: Self::Message) -> Option<Self> {
        let text = msg.into_string();
        println!("{} received pong: {}", self.name, text);
        Some(self)
    }
}

/// Minimal actor abstraction:
/// 1. the actor owns its internal state (`self`)
/// 2. the actor receives messages of one type
/// 3. each message either returns the updated actor back or stops it
pub trait Actor: Sized + Send + 'static {
    type Message: Send + 'static;

    fn process_message(self, msg: Self::Message) -> Option<Self>;

    fn name() -> &'static str {
        std::any::type_name::<Self>()
    }
}

#[derive(Debug, Default)]
pub struct System {
    handles: Vec<JoinHandle<()>>,
}

impl System {
    pub fn run<A: Actor>(&mut self, mut actor: A) -> Sender<A::Message> {
        let (tx, rx) = mpsc::channel();

        let handle = thread::spawn(move || {
            println!("actor {} started", A::name());

            // The mailbox loop lives outside the actor.
            // `recv()` blocks until a new message arrives or all senders are dropped.
            while let Ok(msg) = rx.recv() {
                actor = match actor.process_message(msg) {
                    Some(next_state) => next_state,
                    None => break,
                };
            }

            println!("actor {} finished", A::name());
        });

        self.handles.push(handle);
        tx
    }
}

impl Drop for System {
    fn drop(&mut self) {
        let handles = mem::take(&mut self.handles);
        for handle in handles {
            handle.join().unwrap();
        }
    }
}
