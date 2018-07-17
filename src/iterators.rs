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

#[test]
fn map() {
    let a: Vec<i32> = vec![1, 2, 3];
    let b: Vec<i32> = a.iter().map(|x| x + 1).collect();
    assert_eq!(b, vec![2, 3, 4]);
}

#[test]
fn implementing_iterator() {
    struct Counter {
        count: u32,
        max_num: u32,
    };

    impl Counter {
        fn new(max_num: u32) -> Counter {
            Counter { max_num, count: 0 }
        }
    };

    // Here's the good stuff
    impl Iterator for Counter {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            self.count += 1;

            if self.count <= self.max_num {
                Some(self.count)
            } else {
                None
            }
        }
    }

    let counter = Counter::new(5);
    let mut list: Vec<u32> = Vec::new();
    for i in counter {
        list.push(i);
    }
    assert_eq!(list, vec![1, 2, 3, 4, 5]);
}
