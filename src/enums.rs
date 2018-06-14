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
