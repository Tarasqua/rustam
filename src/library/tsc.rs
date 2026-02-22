use std::collections::*;
use std::io::{self, Write};

pub mod tsc_utils {
    use crate::garden::fruits::Orange;
    use crate::garden::vegetables::Asparagus as Veg;
    use crate::host;

    pub fn test() {
        let orange = Orange {};
        println!("Testing tsc {orange:?}");

        let asparagus = Veg {};

        host::add_to_waitlist();
    }
}
