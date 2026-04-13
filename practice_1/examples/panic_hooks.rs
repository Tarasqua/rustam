fn main() {
    // set_hook_();
    catch_unwind_();
}

#[allow(dead_code)]
fn set_hook_() {
    std::panic::set_hook(Box::new(|info| {
        if let Some(message) = info.payload().downcast_ref::<&str>() {
            println!("Hello, panic: {}", message);
        }
    }));

    panic!("Oh no")
}

fn catch_unwind_() {
    let no_panic = std::panic::catch_unwind(|| {
        println!("no panic here");
        42
    });
    assert!(no_panic.is_ok());

    let with_panic = std::panic::catch_unwind(|| {
        if 2 > 1 {
            panic!("panic here");
        }
    });
    assert!(with_panic.is_err());
    println!("continue execution..."); // NOTE: continue execution after panic
}
