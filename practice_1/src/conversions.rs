use core::fmt;

fn main() {
    let Ok(ltt) = convert(5) else {
        return;
    };
    println!("{:?}", ltt);

    match convert(42) {
        Ok(value) => println!("Less than ten: {:?}", value),
        Err(e) => println!("Error: {}", e),
    }

    let ltt_msg = match LessTnanTen::new(42) {
        Some(ltt) => Some(format!("Less than 10: {}", ltt)),
        None => None,
    };
    println!("{:?}", ltt_msg);

    let ltt_message = LessTnanTen::new(42).map(|ltt| format!("Less than 10: {}", ltt));
    println!("{:?}", ltt_message);

    let other_err = convert(5).map_err(|e| e.to_string());
    println!("{:?}", other_err);
    // can be converted to...
    let other = convert(5)
        .map(|e| format!("Less than 10: {:?}", e))
        .map_err(|e| MyErr(e));
}

struct MyErr(&'static str);

#[derive(Debug)]
struct LessTnanTen(u8);

fn convert(value: u8) -> Result<LessTnanTen, &'static str> {
    value.try_into()
}

impl LessTnanTen {
    fn new(value: u8) -> Option<LessTnanTen> {
        if value < 10 {
            Some(LessTnanTen(value))
        } else {
            None
        }
    }
}

impl fmt::Display for LessTnanTen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// ERROR: impossible to describe the error
// impl From<u8> for LessTnanTen {
//     fn from(value: u8) -> Self {
//         LessTnanTen::new(value).unwrap()
//     }
// }

impl TryFrom<u8> for LessTnanTen {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let mb_ltt = LessTnanTen::new(value);

        // NOTE: Option 1 with MATCH
        // match mb_ltt {
        //     Some(ltt) => Ok(ltt),
        //     None => Err("Value must be less than 10"),
        // }

        // NOTE: Option 2 with IF LET
        // if let Some(ltt) = mb_ltt {
        //     Ok(ltt)
        // } else {
        //     Err("Value must be less than 10")
        // }

        // NOTE: Option 3 with LET ELSE
        // let Some(ltt) = mb_ltt else {
        //     return Err("Value must be less than 10"); // WARNING: must diverge
        // };
        // Ok(ltt)

        // NOTE: Option 4 with OK OR ERR
        mb_ltt.ok_or("Value must be less than 10")
    }
}
