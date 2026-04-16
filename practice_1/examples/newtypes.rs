use std::{fmt, hash::Hash};

fn main() {
    let buf = Command([3u8; 4]);
    println!("Command is '{buf}'");
}

struct Command([u8; 4]);

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Command([1, 1, 1, 1]) => write!(f, "Start"),
            Command([2, 2, 2, 2]) => write!(f, "Finish"),
            _ => write!(f, "Unknown"),
        }
    }
}

impl PartialEq for Command {
    fn eq(&self, other: &Self) -> bool {
        self.0[0] == other.0[0]
    }
}

impl Eq for Command {}

// INFO: key in a hash-based data structure is now based on the first byte
impl Hash for Command {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0[0].hash(state);
    }
}
