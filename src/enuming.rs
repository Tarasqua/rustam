enum IpAddrKind {
    V4,
    V6,
}

enum IpAddr_ {
    V4(u8, u8, u8, u8),
    V6(String),
}

struct IpAddr {
    kind: IpAddrKind,
    ip: String,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quit"),
            Message::Move { x, y } => println!("Move to ({}, {})", x, y),
            Message::Write(text) => println!("Write: {}", text),
            Message::ChangeColor(r, g, b) => println!("Change color to ({}, {}, {})", r, g, b),
        }
    }
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
            // -- snip --
        }
    }
}

fn describe_state_quarter(coin: Coin) -> Option<String> {
    // if let Coin::Quarter(state) = coin {
    //     if state.existed_in(1900) {
    //         Some(format!("{state:?} is pretty old, for America!"))
    //     } else {
    //         Some(format!("{state:?} is relatively new."))
    //     }
    // } else {
    //     None
    // }

    // let state = if let Coin::Quarter(state) = coin {
    //     state
    // } else {
    //     return None;
    // };

    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            match state {
                UsState::Alabama => 25,
                UsState::Alaska => 25,
            }
            // println!("State quarter from {:?}!", state);
            // 25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    x.map(|i| i + 1)
    // match x {
    //     None => None,
    //     Some(i) => Some(i + 1),
    // }
}

pub fn enuming() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        ip: String::from("127.0.0.1"),
    };
    let loopback = IpAddr_::V6(String::from("::1"));
    let loopback = IpAddr_::V6("::1".parse().unwrap());
    let local = IpAddr_::V4(127, 0, 0, 1);

    let message = Message::Move { x: 1, y: 2 };
    message.call();

    let some_number = Some(5);
    let some_char = Some('a');
    let absent_number: Option<i32> = None;

    assert!(some_number.is_some_and(|some_number| some_number > 2));

    let five = Some(5);
    let six = plus_one(five);
    dbg!(six);
    let none = plus_one(None);
    dbg!(none);

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other), // some action with the value
    }
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(), // never mind what the value is
    }
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (), // do nothing
    }

    let num = 3u8; // type suffix instead of `let num: u8 = 3;`
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }
    let config_max: Option<u8> = None; // no println below
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}")
    }

    let coin = Coin::Quarter(UsState::Alabama);
    let mut count = 0;
    match &coin {
        // use coin reference to make the next match possible
        Coin::Quarter(state) => println!("State quarter from {state:?}!"),
        _ => count += 1,
    }
    if let Coin::Quarter(state) = &coin {
        println!("State quarter from {state:?}!")
    } else {
        count += 1;
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
fn reroll() {}
