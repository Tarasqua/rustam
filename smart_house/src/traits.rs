use std::fmt::Debug;

trait Task {
    fn new() -> Self
    where
        Self: Sized; // INFO: only implementable on a concrete type. Marker trait.

    fn execute(&self) -> String {
        "Executing task".to_string()
    }
}

#[derive(Debug)]
struct NetworkTask;

impl Task for NetworkTask {
    fn new() -> Self {
        Self
    }
}

struct DbTask;

impl Task for DbTask {
    fn new() -> Self {
        Self
    }
}

// fn get_task(condition: bool) -> impl Task { // INFO: RPIT Return Position Impl Trait
fn get_task(condition: bool) -> Box<dyn Task> {
    if condition {
        Box::new(DbTask::new())
    } else {
        Box::new(NetworkTask::new())
    }
}

// INFO: ?Sized: Maybe Unsized
// NOTE: fn foo implements Sized by default: fn foo<T>(t: T) {} -> fn foo <T + Sized>(t: T) {}
// NOTE: traits not implement Sized by default.
fn foo<T: ?Sized>(t: &T) {}

// INFO: after + ?Sized, T can be of any size
fn debug<T: Debug + ?Sized>(t: &T) {
    println!("{:?}", t);
}

fn main_() {
    let network = NetworkTask::new();
    let db = DbTask::new();

    // INFO: trait objects defined as `Box<dyn Task>` or `&dyn Task` can be used to hold any type that implements the `Task` trait
    let tasks: Vec<&dyn Task> = vec![&network, &db];

    for task in tasks {
        println!("{}", task.execute());
    }

    debug("my str");
    debug(&NetworkTask);
}
