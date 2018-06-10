#[test]
fn classic_if() {
    if true {
        assert!(true);
    }
}

#[test]
fn classic_if_else() {
    if true {
        assert!(true);
    } else if true {
        assert!(true)
    } else {
        assert!(false);
    }
}

#[test]
fn if_expression() {
    let x = if true { 5 } else { 10 };
    assert_eq!(x, 5);
}
