#[test]
fn return_statement() {
    fn func(x: u8, y: u8) -> u8 {
        return x + y;
    }
    assert_eq!(func(1, 1), 2)
}

#[test]
fn return_expression() {
    fn func(x: u8, y: u8) -> u8 {
        x + y
    }
    assert_eq!(func(1, 1), 2)
}

#[test]
fn function_pointers() {
    fn func(i: u8) -> u8 {
        i
    }

    // with type inference
    let f = func;

    // with explicit types
    // let f: fn(u8) -> u8 = func;

    assert_eq!(f(1), 1)
}

#[test]
fn passing_functions() {
    fn func(i: u8, f: fn(u8) -> u8) -> u8 {
        f(i)
    }

    fn triple(i: u8) -> u8 {
        i * 3
    }

    assert_eq!(func(1, triple), 3);
    assert_eq!(func(2, triple), 6);
}
