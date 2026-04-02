use practice_1::{BinOp, Cache, Expr};

fn main() {
    let a = 2;
    let b = 3;
    let sum = BinOp::plus(a, b);
    let cached_sum = Cache::new(sum);
    let sub = BinOp::minus(cached_sum, 4);

    print_expr(sub);
}

fn print_expr(expr: impl Expr) {
    println!("{}", expr.eval());
}
