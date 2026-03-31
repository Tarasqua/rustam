pub mod dices;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(not(feature = "real_random"))]
pub fn roll_dice() -> u32 {
    4u32
}

#[cfg(feature = "real_random")]
pub fn roll_dice() -> u32 {
    rand::random::<u32>() % 6 + 1
}

fn foo() {
    let mut v = vec![1, 2, 3];
    let x = v.get(0);
    v.push(4);
    println!("{x:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
