#[test]
fn first_test() {
    let left = 1;
    let right = 2;
    assert_eq!(left + right, otus_2::add(left, right));
}

#[test]
#[should_panic]
fn second_test() {
    assert_eq!(5, otus_2::add(2, 1));
}
