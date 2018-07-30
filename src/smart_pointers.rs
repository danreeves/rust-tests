use std::ops::{Deref, DerefMut};

#[test]
fn dereferencing() {
    let a = 5;
    let b = &a;
    assert_eq!(a, 5);
    assert_eq!(*b, 5);
    assert_eq!(*b, a);
}

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

#[test]
fn custom_smart_pointer() {
    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &T {
            &self.0
        }
    }

    impl<T> DerefMut for MyBox<T> {
        fn deref_mut(&mut self) -> &mut T {
            &mut self.0
        }
    }

    // Immutable deref
    let a = 5;
    let b = MyBox::new(a);

    assert_eq!(a, 5);
    assert_eq!(*b, 5);

    // Mutable deref
    let mut c = vec![1];
    c.push(2);
    let mut d = MyBox::new(c);
    (*d).push(3);

    assert_eq!(*d, vec![1, 2, 3]);
}
