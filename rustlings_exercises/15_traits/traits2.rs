trait AppendBar {
    fn append_bar(self) -> Self;
}

// TODO: Implement the trait `AppendBar` for a vector of strings.
// `append_bar` should push the string "Bar" into the vector.

impl AppendBar for Vec<String> {
    fn append_bar(mut self) -> Self {
        // let mut new_vec = self; // INFO: for immutable self
        // new_vec.push("Bar".to_string());
        // new_vec
        self.push(String::from("Bar")); // INFO: for mutable self
        self
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), "Bar");
        assert_eq!(foo.pop().unwrap(), "Foo");
    }
}
