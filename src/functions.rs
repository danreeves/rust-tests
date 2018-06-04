#[test]
fn arguments_and_returns() {
    fn func(i: i8) -> i8 {
        return i;
    }
    assert_eq!(func(1), 1)
}
