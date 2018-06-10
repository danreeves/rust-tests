#[test]
fn while_loop() {
    let mut x = 0;

    while x != 10 {
        x += 1;
    }

    assert_eq!(x, 10);
}

#[test]
fn for_in_loops() {
    let mut x = 0;

    for i in 0..10 {
        // Start is inclusive, end is exclusive
        x += i;
    }

    assert_eq!(x, 0 + 0 + 1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9);
}

#[test]
fn enumerate() {
    // .enumerate provides an (index, value) tuple for
    // each value in an iterator
    for (i, v) in (5..11).enumerate() {
        if i == 0 {
            assert_eq!(v, 5);
        }
        if i == 5 {
            assert_eq!(v, 10);
        }
    }
}

#[test]
fn loop_and_break() {
    // An infinite loop
    loop {
        assert!(true);
        break;
    }
    assert!(true);
}

#[test]
fn loop_and_continue() {
    let mut x = 0;

    for i in 1..11 {
        if i % 2 == 0 {
            continue;
        }
        x += i;
    }

    assert_eq!(x, 1 + 3 + 5 + 7 + 9);
}

#[test]
fn loop_labels() {
    'outer: for x in 0..10 {
        'inner: for y in 0..10 {
            if x != 5 {
                continue 'outer;
            }
            if y != 5 {
                continue 'inner;
            }
            assert_eq!(x, 5);
            assert_eq!(y, 5);
            break;
        }
    }
}
