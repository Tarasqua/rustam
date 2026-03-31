macro_rules! four {
    () => {
        1 + 3
    };
}

macro_rules! add {
    ($a: expr, $b: expr) => {
        $a + $b
    };
}

// NOTE: repetitions
macro_rules! vector {
    // * means repeat zero or more times
    ( $($elem:expr),* ) => {
        {
            let mut vec = Vec::new();
            $( vec.push($elem); )*
            vec
        }
    };
}

macro_rules! repeat_two {
    ( $($i:ident)*, $($j:ident)* ) => {
        $( let $i: (); let $j: (); )*;
    };
}

macro_rules! hashmap {
    // NOTE: $(,)? is optional trailing comma
    ( $( $key:expr => $val:expr ),* $(,)?) => {
        {
            let mut map = std::collections::HashMap::new();
            $( map.insert($key, $val); )*
            map
        }
    };
}

macro_rules! timeit {
    ($name:expr, $code:block) => {{
        let start = std::time::Instant::now();
        let result = $code;
        println!("{}: {:?}", $name, start.elapsed());
        result
    }};
}

pub fn timeit_() {
    let sum: i32 = timeit!("Summing loop", { (0..100).sum() });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_four() {
        // INFO: four!(), four![], four!{}
        assert_eq!(four!(), 4);
    }

    #[test]
    fn test_add() {
        assert_eq!(add!(1, 3 + 2), 6);
    }

    #[test]
    fn test_vector() {
        let v = vector![1, 2, 3];
        assert_eq!(v, vec![1, 2, 3]);
    }

    #[test]
    fn test_map() {
        let map = hashmap!(1 => 2, 3 => 4, );
        let real_map = std::collections::HashMap::from([(1, 2), (3, 4)]);
        assert_eq!(map, real_map);
    }
}
