fn simple_match() {
    let x = 5;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

pub fn named_match() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"), // NOTE: y is shadowing the outer y so the "Matched, y = 5" will be printed
        _ => println!("Default case, x = {x:?}"),
    }

    println!("at the end: x = {x:?}, y = {y}"); // NOTE: at the end: x = Some(5), y = 10
}

fn or_match() {
    let x = 1;

    match x {
        1 | 2 => println!("one or two"), // NOTE: "one or two" will be printed
        3 => println!("three"),
        _ => println!("anything"),
    }
}

fn range_match() {
    let x = 5;

    match x {
        1..=5 => println!("one through five"), // NOTE: "one through five" will be printed
        _ => println!("anything"),
    }

    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"), // NOTE: "early ASCII letter" will be printed
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}

fn match_guard() {
    // WARNING: works only with `match` not with `if let` or `while let`
    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("even"),
        Some(x) => println!("odd"),
        _ => println!("nothing"),
    }

    // INFO: match guard to fix pattern-shadowing problem

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {x:?}"), // NOTE: "Default case, x = Some(5)" will be printed
    }

    println!("at the end: x = {x:?}, y = {y}");

    // INFO: or pattern with match guard

    let x = 4;
    let y = false;

    match x {
        // NOTE: x is equal to 4, 5, or 6 and if y is true
        // pattern works like this: (4 | 5 | 6) if y => ...
        // rather than this: 4 | 5 | (6 if y) => ...
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"), // NOTE: "no" will be printed
    }
}
