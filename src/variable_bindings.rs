#[test]
fn assignment() {
    let x = 2;
    assert_eq!(x, 2);
}

#[test]
fn lhs_is_a_pattern() {
    let (x, y) = (1, 2);
    assert_eq!(x, 1);
    assert_eq!(y, 2);
}

#[test]
fn any_case_binding() {
    let x = 2;
    match x {
        y => assert_eq!(y, 2),
    }
}

#[test]
fn it_allows_shadowing() {
    let x = 1;
    let y = 2;

    assert_eq!(y, 2);

    match x {
        y => assert_eq!(y, 1),
    }

    assert_eq!(y, 2);
}

#[test]
fn mutability() {
    let mut x = 1;
    assert_eq!(x, 1);
    x = 2;
    assert_eq!(x, 2);
}

#[test]
fn scope_and_shadowing() {
    let x = 1;
    assert_eq!(x, 1);
    {
        assert_eq!(x, 1);
        let x = 2;
        assert_eq!(x, 2);
    }
    assert_eq!(x, 1);
}

#[test]
fn rebinding() {
    let x = 2;
    assert_eq!(x, 2);

    let x = "string";
    assert_eq!(x, "string");
}
