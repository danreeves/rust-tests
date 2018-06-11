#[test]
fn string_slices() {
    fn first_word(string: &str) -> &str {
        let bytes = string.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &string[..i];
            }
        }

        &string[..]
    }

    let x = "hello world";
    let y = first_word(&x);
    assert_eq!(y, "hello");
}

#[test]
fn array_slices() {
    let a = [1, 2, 3];

    let b = &a[..];
    let c = &a[0..3];
    assert_eq!(a, b);
    assert_eq!(a, c);

    let d = &a[1..3];
    assert_eq!(d, [2, 3]);
}
