use std::result;

use practice_1::{BinOp, Cache, DynTransform, Expr, Transform};

fn main() {
    let a = 2;
    let b = 3;
    let sum = BinOp::plus(a, b);
    let cached_sum = Cache::new(sum);
    let sub = BinOp::minus(cached_sum, 4);
    let inverted = Transform::new(sub, |x| x ^ i32::MAX);

    print_expr(inverted);

    let transforms = [
        DynTransform::new(42, |x| x ^ i32::MAX),
        DynTransform::new(42, |x| x % i32::MAX),
    ];

    let expr = 42;
    let other = BinOp::plus(expr, 1);
    let bin_op = other + 25;

    let inverted = Transform::new(100, |x| x ^ i32::MAX);
    let result = bin_op + inverted;
    print_expr(result);
}

fn print_expr(expr: impl Expr) {
    println!("{}", expr.eval());
}
