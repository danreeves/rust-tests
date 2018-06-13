use std::any::{Any, TypeId};
use std::f32::consts::PI;

#[test]
fn creating_structs() {
    struct User {
        name: String,
        age: u32,
    }
    let mut dan = User {
        name: String::from("Dan"),
        age: 25,
    };
    assert_eq!(dan.name, "Dan");
    assert_eq!(dan.age, 25);

    dan.name = String::from("Dan Reeves");
    assert_eq!(dan.name, "Dan Reeves");
}

#[test]
fn field_init_shorthand() {
    struct User {
        name: String,
        age: u32,
    }
    let name = String::from("Dan");
    let age = 25;
    let dan = User { name, age };
    assert_eq!(dan.name, "Dan");
    assert_eq!(dan.age, 25);
}

#[test]
fn constructor_functions() {
    struct Cat {
        name: String,
        legs: u8,
        is_a_cat: bool,
    }
    fn new_cat(name: String, legs: u8) -> Cat {
        Cat {
            name,
            legs,
            is_a_cat: true,
        }
    }
    let legolas = new_cat(String::from("Legolas"), 0);
    assert_eq!(legolas.name, "Legolas");
    assert_eq!(legolas.legs, 0);
    assert_eq!(legolas.is_a_cat, true);
}

#[test]
fn struct_update_syntax_basically_destructuring() {
    struct Cat {
        name: String,
        legs: u8,
        is_a_cat: bool,
    }
    let legolas = Cat {
        name: String::from("Legolas"),
        legs: 0,
        is_a_cat: true,
    };
    assert_eq!(legolas.name, "Legolas");
    assert_eq!(legolas.legs, 0);
    assert_eq!(legolas.is_a_cat, true);

    let doggolas = Cat {
        is_a_cat: false,
        ..legolas
    };
    assert_eq!(doggolas.name, "Legolas");
    assert_eq!(doggolas.legs, 0);
    assert_eq!(doggolas.is_a_cat, false);
}

#[test]
fn tuple_structs() {
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    // Ooh fance I wrote this
    fn same_type<T1: ?Sized + Any, T2: ?Sized + Any>(_a: &T1, _b: &T2) -> bool {
        TypeId::of::<T1>() == TypeId::of::<T2>()
    }

    let black = Color(0, 0, 0);
    let white = Color(255, 255, 255);
    let origin = Point(0, 0, 0);
    assert!(same_type(&black, &white));
    assert!(!same_type(&black, &origin));
}

#[test]
fn methods() {
    struct Rect {
        height: u32,
        width: u32,
    }
    struct Circle {
        radius: f32,
        circumference: f32,
    }

    impl Rect {
        // An associated function.
        // Called with :: rather than . and doesn't have access to an instance
        fn square(size: u32) -> Rect {
            Rect {
                height: size,
                width: size,
            }
        }

        // Uses self reference
        fn area(&self) -> u32 {
            self.width * self.height
        }

        // Uses self reference and takes another argument
        fn can_fit(&self, other: &Rect) -> bool {
            self.width >= other.width && self.height >= other.height
        }

        // Uses self mutably
        fn double(&mut self) {
            self.height *= 2;
            self.width *= 2;
        }

        // Takes ownership and returns a new type
        fn to_circle(self) -> Circle {
            let diameter = self.height as f32;
            let radius = diameter / 2.0;
            let circumference = PI * diameter;
            Circle {
                radius,
                circumference,
            }
        }
    }

    let mut square = Rect::square(42);
    let rectangle = Rect {
        height: 42,
        width: 84,
    };

    assert_eq!(square.area(), 1764);

    assert_eq!(square.can_fit(&rectangle), false);
    assert_eq!(rectangle.can_fit(&square), true);

    square.double();
    assert_eq!(square.height, 84);
    assert_eq!(square.width, 84);

    // We can no longer use square because the to_circle method
    // took ownership and converted it into a circle
    let circle = square.to_circle();
    assert_eq!(circle.radius, 42.0);
    assert_eq!(circle.circumference, 263.8938);
}
