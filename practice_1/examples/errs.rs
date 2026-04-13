fn main() {
    let result = foo();
    let Ok(_value) = result.inspect_err(|error| println!("{error}")) else {
        return;
    };

    let res2 = foo();
    let _value = match res2 {
        Ok(v) => v,
        Err(e) => {
            println!("{e}");
            return;
        }
    };
}

fn foo() -> Result<i32, std::io::Error> {
    std::fs::read_to_string("foobar")?;
    Ok(42)
}

#[allow(dead_code)]
enum MyError {
    SomeError,
    // IoError(std::io::Error),
    IoError(String),
}

// impl From<std::io::Error> for MyError {
//     fn from(error: std::io::Error) -> Self {
//         MyError::IoError(error)
//     }
// }

#[allow(dead_code)]
fn bar() -> Result<i32, MyError> {
    let context = "bar call";
    let val =
        foo().map_err(|e| MyError::IoError(format!("io error {e} in context of {context}")))?;
    Ok(val)
}
