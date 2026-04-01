use from_primitive::FromPrimitive;

#[derive(Debug, PartialEq, FromPrimitive)]
enum Example {
    A,
    B,
    C,
    D,
}

fn main() {
    let a = Example::try_from(0).unwrap();
    dbg!(a);
}
