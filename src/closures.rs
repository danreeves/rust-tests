#[test]
fn environment_capturing() {
    let a = 1;

    // eq_a has access to a because it was defined while a was in scope
    let eq_a = |x| x == a;

    let b = 1;
    let c = 2;

    assert_eq!(eq_a(b), true);
    assert_eq!(eq_a(c), false);
}

#[test]
fn mutable_capturing() {
    let mut a = 1;
    {
        let mut incr_a = || a += 1;
        incr_a();
        incr_a();
        incr_a();
    }
    assert_eq!(a, 4);
}
