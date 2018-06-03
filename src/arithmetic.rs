#[test]
fn addition() {
    assert_eq!(2 + 2, 4);
}

#[test]
fn subtraction() {
    assert_eq!(2 - 2, 0);
}

#[test]
fn multiplication() {
    assert_eq!(2 * 2, 4);
}

#[test]
fn division() {
    assert_eq!(2 / 2, 1);
}

#[test]
fn incremental_assignment() {
    let mut x = 2;
    x += 1;
    assert_eq!(x, 3);
}

#[test]
fn multiplicitive_assignment() {
    let mut x = 2;
    x *= 2;
    assert_eq!(x, 4)
}

#[test]
fn subtractive_assignment() {
    let mut x = 2;
    x -= 1;
    assert_eq!(x, 1);
}

#[test]
fn divisive_assignment() {
    let mut x = 2;
    x /= 2;
    assert_eq!(x, 1);
}

#[test]
fn negation() {
    let mut x = 2;
    x = -x;
    assert_eq!(x, -2);
}

#[test]
fn equality() {
    assert_eq!(2 == 2, true);
    assert_eq!(2 == 3, false);
}

#[test]
fn casting_types() {
    let x: i32 = 32;
    let y: u64 = 64;

    let z = x + y as i32;

    assert_eq!(z, 96);
}
