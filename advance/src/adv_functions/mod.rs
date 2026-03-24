// NOTE: It’s best to write functions using a generic type and one of the closure traits so that your functions can accept either functions or closures.
// WARNING: one example of where you would want to only accept fn and not closures is when interfacing with external code that doesn’t have closures: C functions can accept functions as arguments, but C doesn’t have closures.

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn stringify() {
    let list_of_numbers = vec![1, 2, 3];

    let list_of_strings: Vec<String> = list_of_numbers. // INFO: using closure
        iter()
        .map(|i| i.to_string())
        .collect();
    let list_of_strings: Vec<String> = list_of_numbers // INFO: using trait method
        .iter()
        .map(ToString::to_string)
        .collect();
}

enum Status {
    Value(u32),
    Stop,
}

fn statusify() {
    let list_of_statuses: Vec<Status> = (0u32..20) // INFO: makes vector of Status::Value variants
        .map(Status::Value)
        .collect();

    let list_of_stops: Vec<Status> = (0u32..5) // INFO: makes vector of Status::Stop variants
        .map(|_| Status::Stop)
        .collect();
}

// NOTE: we can't return closure because they don't have a concrete type that is returnable; we're not allowed to use the function pointer fn as a return type if the closure captures any values from its scope, for example.
// NOTE: Instead, we can return a trait object, which is a pointer to a value that implements a particular trait.
// NOTE: impl Fn(i32) -> i32 is an opaque type
fn returns_closure() -> impl Fn(i32) -> i32 {
    |x| x + 1
}

fn handlers() {
    fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
        // INFO: Box is needed due to the unknown size of the closure type
        Box::new(|x| x + 1)
    }

    fn returns_initialized_closure(init: i32) -> Box<dyn Fn(i32) -> i32> {
        // INFO: move keyword forces the closure to take ownership of init and move it into the closure's own internal storage (which is inside the Box on the heap), so init doesn't disappear when the function returns
        Box::new(move |x| x + init)
    }

    let handlers = vec![returns_closure(), returns_initialized_closure(123)];
    for handler in handlers {
        let output = handler(5);
        println!("{output}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_do_twice() {
        assert_eq!(do_twice(add_one, 5), 12);
    }
}
