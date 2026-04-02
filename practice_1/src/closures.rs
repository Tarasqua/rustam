fn main() {
    let y = String::from("world");

    checked_print(true, |x| format!("The answer is: {x}, {y}"));
    checked_print(false, |x| format!("The answer is: {x}, {y}"));
    checked_print(true, get_result);

    checked_print_fn(true, get_result);
    // checked_print_fn(true, |x| format!("The answer is: {x}, {y}")); // ERROR HERE

    // ===========================================

    let res = checked_call(
        true,
        |(a1, a2, a3)| format!("{a1}, {a2}, {a3}"),
        (true, 42, "hello"),
    );
    println!("{:?}", res);

    let res = checked_call(
        true,
        |(a1, a2)| format!("{a1}, {a2}").into_bytes(),
        (42.0, "hello"),
    );
    println!("{:?}", res);

    // ===========================================

    let res = compose(|x| x + 1, 42, |x| format!("Result {x}"));
    println!("{:?}", res);
}

fn get_result(x: i32) -> String {
    format!("The answer is: {x}")
}

fn checked_print<F>(condition: bool, f: F)
where
    F: FnOnce(i32) -> String,
{
    if condition {
        let result = f(42);
        println!("result: {}", result);
    } else {
        println!("Condition is false");
    }
}

fn checked_print_fn(condition: bool, f: fn(i32) -> String) {
    if condition {
        let result = f(42);
        println!("result: {}", result);
    } else {
        println!("Condition is false");
    }
}

fn checked_call<F, Args, Res>(condition: bool, f: F, args: Args) -> Option<Res>
where
    Res: Default,
    F: FnOnce(Args) -> Res,
{
    if condition { Some(f(args)) } else { None }
}

fn compose<F1, Args, Res1, F2, Res2>(f1: F1, args: Args, f2: F2) -> Res2
where
    F1: FnOnce(Args) -> Res1,
    F2: FnOnce(Res1) -> Res2,
{
    f2(f1(args))
}
