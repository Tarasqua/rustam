pub mod decl_macr;
pub mod figures;
pub mod plug;
pub mod proc_macr;
pub mod sorter;

fn compare<T: PartialOrd>(a: T, b: T) -> T {
    match a.partial_cmp(&b) {
        Some(std::cmp::Ordering::Greater) => a,
        _ => b,
    }
}

#[derive(Debug)]
struct Pair<T, U> {
    first: T,
    second: U,
}

impl<T, U> Pair<T, U>
where
    T: Clone,
    U: Clone,
{
    fn new(first: T, second: U) -> Self {
        Pair { first, second }
    }

    fn swap(self) -> Pair<U, T> {
        Pair {
            first: self.second,
            second: self.first,
        }
    }

    fn swap_clone(&self) -> Pair<T, U> {
        Pair {
            first: self.first.clone(),
            second: self.second.clone(),
        }
    }

    fn swap_refs(&self) -> Pair<&U, &T> {
        Pair {
            first: &self.second,
            second: &self.first,
        }
    }
}

trait Area<T> {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

struct Square {
    side: f64,
}

impl<T> Area<T> for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius.powi(2)
    }
}

impl<T> Area<T> for Square {
    fn area(&self) -> f64 {
        self.side.powi(2)
    }
}

fn print_areaA<T: Area<T>>(shape: T) {
    println!("The area is: {}", shape.area());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare() {
        assert_eq!(compare(1, 2), 2);
        assert_eq!(compare(2.2, 1.2), 2.2);
        assert_eq!(compare('a', 'z'), 'z');
    }

    fn test_swap() {
        let p = Pair::new("first", 3);
        let swapped = p.swap();

        assert_eq!(swapped.first, 3);
        assert_eq!(swapped.second, "first");
    }
}
