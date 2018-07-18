#[test]
fn assigning_to_the_heap_with_boxes() {
    let metal_bawkses = Box::new(5);
    assert_eq!(*metal_bawkses, 5);
}

#[test]
fn recursive_types() {
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    let list = List::Cons(
        1,
        Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
    );

    if let List::Cons(id, list) = list {
        assert_eq!(id, 1);
        if let List::Cons(id, _list) = *list {
            assert_eq!(id, 2);
        }
    }
}
