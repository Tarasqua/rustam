use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if (1..100).contains(&value) {
            panic!("Guess value must be between 1 and 100, got {value}.");
        };
        // if value < 1 || value > 100 {
        //     panic!("Guess value must be between 1 and 100, got {value}.");
        // };

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

pub fn game() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: Guess = match guess.trim().parse::<i32>() {
            Ok(num) => Guess::new(num),
            Err(_) => continue,
        };

        match guess.value().cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
