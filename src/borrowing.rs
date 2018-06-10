#[test]
fn immutable_references() {
    let x = vec![1, 2, 3];
    let y = &x; //            -+ `y` comes into scope.
    assert!(y.len() == 3); // -+ `y` goes out of scope.
    assert!(x.len() == 3);
}

#[test]
fn mutable_references() {
    let mut x = vec![1, 2, 3];
    {
        // This extra scope won't be necessary with NLL (Non lexical lifetimes)
        let y = &mut x; // -+ `y` comes into scope.
        y.push(4); //       |
    } //                   -+ `y` goes out of scope.
    assert_eq!(x, [1, 2, 3, 4]);
}

#[test]
fn borrowing_with_function_arguments() {
    fn sum_lens(a: &Vec<i32>, b: &Vec<i32>) -> i32 {
        a.len() as i32 + b.len() as i32
    }
    let x = vec![1, 2];
    let y = vec![3, 4];
    let z = sum_lens(&x, &y);
    assert_eq!(x.len(), 2);
    assert_eq!(y.len(), 2);
    assert_eq!(z, 4);
}

#[test]
fn lifetimes() {
    // Assign arg1 and return value to lifetime a so that v and str1 share
    // a lifetime and v does't disappear when str2 does
    fn some_func<'a, 'b>(str1: &'a str, str2: &'b str) -> &'a str {
        str2.to_string(); // Do a thing so we don't get warnings about unused
        str1
    }

    let str1 = "string 1";

    let v;
    {
        let str2 = "string 2"; //    -+ `str2` comes into scope.
        v = some_func(str1, str2); // |
    } //                             -+ `str2` goes out of scope.
    assert_eq!(v, "string 1")
}
