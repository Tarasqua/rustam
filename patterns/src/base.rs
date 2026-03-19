struct Point {
    x: i32,
    y: i32,
    z: i32,
}

fn unpack_tuple() {
    let (first, .., last) = (1, 2, 3, 4, 5); // .. ignores all the elements in the middle
    let (first, _, _, _, last) = (1, 2, 3, 4, 5); // _ ignores one element

    // =================
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value"); // NOTE: this one will be printed
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {setting_value:?}"); // NOTE: setting is Some(5)

    // =================

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}");
        }
    }

    // =================
    let origin = Point { x: 0, y: 0, z: 0 };

    match origin {
        Point { x, .. } => println!("x is {x}"), // INFO: ignoring the rest
    }
}

fn iflet() {
    // WARNING: if let expressions is that the compiler doesn’t check for exhaustiveness, whereas with match expressions it does: If we omitted the last else block and therefore missed handling some cases, the compiler would not alert us to the possible logic bug.
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        // age is shadowed by the Ok variant of the Result and now is a u8
        // so now we can do like: if let Ok(age) = age && age > 30
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}

fn whilelet() {
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        for val in [1, 2, 3] {
            tx.send(val).unwrap();
        }
    });

    // INFO: hecking a Result instead of an Option
    while let Ok(value) = rx.recv() {
        println!("{value}");
    }
}

fn forloop() {
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{value} is at index {index}");
    }
}

fn coordinates() {
    // INFO: this function waits for a tuple of two i32 values
    fn print_coordinates(&(x, y): &(i32, i32)) {
        println!("Current location: ({x}, {y})");
    }

    let point = (3, 5);
    print_coordinates(&point);
}
