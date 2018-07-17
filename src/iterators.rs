#[test]
fn iter() {
    let a = vec![1, 2, 3];
    let mut it = a.iter();
    assert_eq!(it.next(), Some(&1));
    assert_eq!(it.next(), Some(&2));
    assert_eq!(it.next(), Some(&3));
    assert_eq!(it.next(), None);
}

#[test]
fn into_iter() {
    let a = vec![1, 2, 3];
    let mut it = a.into_iter();
    // Now we can't access a because into_iter takes ownership
    // but we have ownership of the items now, instead of references
    assert_eq!(it.next(), Some(1));
    assert_eq!(it.next(), Some(2));
    assert_eq!(it.next(), Some(3));
    assert_eq!(it.next(), None);
}
