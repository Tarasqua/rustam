trait One {
    fn get(&self) -> i32;
}

trait Two {
    fn get(&self) -> String;
}

impl One for () {
    fn get(&self) -> i32 {
        1
    }
}

impl Two for () {
    fn get(&self) -> String {
        String::from("2")
    }
}

fn try_it() {
    // ERROR: метод `get` не найден для типа `()`
    // ().get();

    // INFO: используем turbofish для указания типа, который реализует метод `get`
    let n = <() as One>::get(&());
    let s = <() as Two>::get(&());
}
