use std::any::{Any, TypeId};
use std::cmp::PartialOrd;

fn same_type<T1: ?Sized + Any, T2: ?Sized + Any>(_a: &T1, _b: &T2) -> bool {
    TypeId::of::<T1>() == TypeId::of::<T2>()
}

#[test]
fn generics() {
    // it takes types that have the PartialOrd and Copy traits
    fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
        let mut largest = list[0];

        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    assert_eq!(largest(&vec![1, 2, 3]), 3);
    assert_eq!(largest(&vec!['a', 'b', 'c']), 'c');
}

#[test]
fn in_structs() {
    struct Point<T> {
        x: T,
        y: T,
    }

    let a = Point { x: 1, y: 2 };
    let b = Point { x: 1.5, y: 2.5 };
    let c = Point { x: 'x', y: 'y' };

    assert!(same_type(&a.x, &a.y));
    assert!(same_type(&b.x, &b.y));
    assert!(same_type(&c.x, &c.y));

    struct Pointy<A, B> {
        a: A,
        b: B,
    }

    let d = Pointy { a: 1, b: 1.1 };
    let e = Pointy { a: 'a', b: 2 };

    assert!(!same_type(&d.a, &d.b));
    assert!(!same_type(&e.a, &e.b));
}
