#[test]
fn vectors() {
    let x = vec![0; 10];
    assert_eq!(x, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
}

#[test]
fn index_access() {
    let x = vec![1, 2, 3];
    assert_eq!(x[1], 2);
}

#[test]
fn out_of_bounds_access() {
    let x = vec![1, 2, 3];
    let maybe_index = x.get(42);
    if let Some(_val) = maybe_index {
        assert!(false);
    } else {
        assert!(true);
    }
}
