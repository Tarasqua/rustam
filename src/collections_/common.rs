#[derive(Debug)]
enum SpreadsheetCell {
    Int32(i32),
    Float64(f64),
    Text(String),
}

pub fn vectors() {
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(1);
    v.push(1);
    // v.push('a');

    let third: &i32 = &v[2];
    dbg!(third);

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third value ir {third}"),
        None => println!("There is no third element."),
    }

    let mut v = vec![1, 2, 3, 4, 5];
    let first = v[0]; // adding a reference lead to a borrow error
    v[0] = 10; // and this line is invalid
    v.push(1); // and this one too
    println!("The first element is: {first}");

    let mut v = vec![100, 52, 23];
    for i in &mut v {
        *i += 50; // dereference
    }
    dbg!(v);

    let mut row = vec![
        SpreadsheetCell::Int32(32),
        SpreadsheetCell::Float64(32.),
        SpreadsheetCell::Text(String::from("Hello")),
        SpreadsheetCell::Text("Hello".to_string()),
    ];
    let last = row.last().unwrap(); // without deleting the last element
    dbg!(last);
    let last = row.pop(); // removes and gives the Some statement
    dbg!(last);
}

pub fn strings() {}
