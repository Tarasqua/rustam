use super::*;

#[test]
fn test_roll() {
    let dice = Dice::new(6);
    assert_eq!(dice.roll(), 32);
}
