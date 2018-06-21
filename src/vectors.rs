#[test]
fn vector_macro() {
    let x = vec![0; 10];
    assert_eq!(x, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
}

#[test]
fn vector_constructor() {
    let x: Vec<i32> = Vec::new();
    assert_eq!(x, vec![]);
}

#[test]
fn index_access() {
    let x = vec![1, 2, 3];
    assert_eq!(x[1], 2);
}

#[test]
fn maybe_out_of_bounds_access() {
    let x = vec![1, 2, 3];
    let maybe_index = x.get(42);
    if let Some(_val) = maybe_index {
        assert!(false);
    } else {
        assert!(true);
    }
}

#[test]
fn push() {
    let mut x: Vec<i32> = Vec::new();
    assert_eq!(x, vec![]);
    x.push(1);
    assert_eq!(x, vec![1]);
}

#[test]
fn pop() {
    let mut x = vec![1, 2, 3];
    let maybe_three = x.pop();
    if let Some(three) = maybe_three {
        assert_eq!(three, 3);
    }
    assert_eq!(x, vec![1, 2]);
}

#[test]
fn iterating() {
    let x = vec![1, 2, 3];
    let mut count = 0;
    for _i in &x {
        count += 1
    }
    assert_eq!(count, 3);
}

#[test]
fn mutably_iterating() {
    let mut x = vec![1, 2, 3];
    for i in &mut x {
        // Need to dereference the pointer to an integer
        *i *= 10
    }
    assert_eq!(x, vec![10, 20, 30]);
}
