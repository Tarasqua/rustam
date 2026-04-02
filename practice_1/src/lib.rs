use std::cell::Cell;

mod closures;
mod fn_traits;

pub trait Expr {
    fn eval(&self) -> i32;
}

impl Expr for i32 {
    fn eval(&self) -> i32 {
        *self
    }
}

pub struct BinOp<L, R> {
    left: L,
    right: R,
    op: Op,
}

impl<L, R> BinOp<L, R> {
    pub fn plus(left: L, right: R) -> Self {
        BinOp {
            left,
            right,
            op: Op::Add,
        }
    }

    pub fn minus(left: L, right: R) -> Self {
        BinOp {
            left,
            right,
            op: Op::Sub,
        }
    }
}

#[allow(dead_code)]
enum Op {
    Add,
    Sub,
}

impl<L: Expr, R: Expr> Expr for BinOp<L, R> {
    fn eval(&self) -> i32 {
        match self.op {
            Op::Add => self.left.eval() + self.right.eval(),
            Op::Sub => self.left.eval() - self.right.eval(),
        }
    }
}

pub struct Cache<I> {
    inner: I,
    cache: Cell<Option<i32>>,
}

impl<I> Cache<I> {
    pub fn new(inner: I) -> Cache<I> {
        Cache {
            inner,
            cache: Cell::new(None),
        }
    }
}

impl<I: Expr> Expr for Cache<I> {
    fn eval(&self) -> i32 {
        match self.cache.get() {
            Some(v) => v,
            None => {
                let v = self.inner.eval();
                self.cache.replace(Some(v));
                v
            }
        }
    }
}
