#[test]
fn chars() {
    // chars are single UTF character and are denoted by the single quote
    let x: char = 'x';
    let cat: char = '😻';
    assert_eq!(x.len_utf8(), 1);
    assert_eq!(cat.len_utf8(), 4);
}

#[test]
fn array_shorthand() {
    // Shorthand for prefilling an array
    // [type or initial value; length]
    let x = [0; 3];
    let y = [0, 0, 0];
    assert_eq!(x, y);
}

#[test]
fn array_index_access() {
    let x = [1, 2, 3];
    let y = x[1];
    assert_eq!(y, 2);
}

#[test]
fn array_index_write() {
    let mut x = [1, 2, 3];
    x[1] = 0;
    assert_eq!(x, [1, 0, 3]);
}

#[test]
fn array_slicing() {
    let x = [1, 2, 3, 4];
    let y = &x[1..3]; // start is inclusive, end is exclusive...
    assert_eq!(y, [2, 3]);
}

#[test]
fn tuples_of_matching_type_and_arity() {
    let x = (1, 'a');
    let mut _y = (2, 'b');
    assert_ne!(x, _y);
    _y = x;
    assert_eq!(x, _y);
}

#[test]
fn destructuring_tuples() {
    let x = (1, 2, 3);
    let (a, b, c) = x;
    assert_eq!(a, 1);
    assert_eq!(b, 2);
    assert_eq!(c, 3);
    assert_eq!((a, b, c), x);
}

#[test]
fn tuple_index_access() {
    let x = (1, 2, 3);
    let y = x.1; // It's different from arrays for some reason?
    assert_eq!(y, 2);
}

#[test]
fn tuple_index_writing() {
    let mut x = (1, 2, 3);
    x.1 = 0;
    assert_eq!(x, (1, 0, 3));
}
