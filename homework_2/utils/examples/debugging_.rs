fn main() {
    let a = 42;
    let s = String::from("hello");
    let arr = [1, 2, 3];
    let cat = Cat {
        name: String::from("kitty"),
        age: 3,
    };

    println!("{}", a);
    println!("{}", s);
    println!("{:?}", arr);
    println!("{:?}", cat);
}

#[derive(Debug)]
struct Cat {
    name: String,
    age: u8,
}
