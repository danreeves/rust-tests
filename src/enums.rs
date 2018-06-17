extern crate rand;

#[test]
fn enums_are_cool() {
    enum Animal {
        Cat,
        Dog,
        Bird(String),
        Robot,
    }

    let dobby = Animal::Dog;
    let gizmo = Animal::Cat;
    let polly = Animal::Bird(String::from("Parrot"));
    let artoo = Animal::Robot;

    fn noise(animal: Animal) -> String {
        match animal {
            Animal::Dog => String::from("Woof!"),
            Animal::Cat => String::from("Meow!"),
            Animal::Bird(name) => format!("{}s go squawk!", name),
            _ => String::from("beep"),
        }
    }

    assert_eq!(noise(dobby), "Woof!");
    assert_eq!(noise(gizmo), "Meow!");
    assert_eq!(noise(polly), "Parrots go squawk!");
    assert_eq!(noise(artoo), "beep");
}

#[test]
fn if_let() {
    fn maybe_a_number() -> Option<u8> {
        if rand::random() {
            return Some(rand::random());
        }
        None
    }

    let idk_is_it = maybe_a_number();

    if let Some(yes_a_number) = idk_is_it {
        // This is cool! We can do a thing if there is a value and the
        // if let construct will unwrap that value for you!
        assert!(yes_a_number > 0);
    } else {
        assert_eq!(idk_is_it, None);
    }
}
