use adder::add_two_pub;

#[test]
fn it_adds_two() {
    let result = add_two_pub(2);
    assert_eq!(result, 4);
}
