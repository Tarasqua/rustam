use std::{cell::Cell, ops::Add, process::Output};

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

impl<L, R, Rhs> Add<Rhs> for BinOp<L, R> {
    type Output = BinOp<Self, Rhs>;

    fn add(self, rhs: Rhs) -> Self::Output {
        BinOp::plus(self, rhs)
    }
}

trait MyAdd<Rhs> {
    type Output;

    fn add(self, rhs: Rhs) -> Output;
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

pub struct Transform<I, F> {
    inner: I,
    transform: F,
}

impl<I, F> Transform<I, F> {
    pub fn new(inner: I, transform: F) -> Transform<I, F> {
        Transform { inner, transform }
    }
}

impl<I: Expr, F: Fn(i32) -> i32> Expr for Transform<I, F> {
    fn eval(&self) -> i32 {
        let inner_res = self.inner.eval();
        (self.transform)(inner_res)
    }
}

pub struct DynTransform<I> {
    inner: I,
    transform: Box<dyn Fn(i32) -> i32>,
}

impl<I: Expr> DynTransform<I> {
    // INFO: + 'static is needed to ensure that the closure outlives the lifetime of the transform and NOT DOES NOT CAPTURE THE INNER VALUE!
    // INFO: transform accepts any functor that implements Fn(i32) -> i32 and has the 'static lifetime
    pub fn new<F>(inner: I, transform: F) -> Self
    where
        F: Fn(i32) -> i32 + 'static,
    {
        DynTransform {
            inner,
            transform: Box::new(transform),
        }
    }
}

impl<I: Expr> Expr for DynTransform<I> {
    fn eval(&self) -> i32 {
        let inner_res = self.inner.eval();
        (self.transform)(inner_res)
    }
}
