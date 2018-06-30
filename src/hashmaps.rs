use std::collections::HashMap;

#[test]
fn construction() {
    let mut h = HashMap::new();
    h.insert("Hello".to_string(), "World".to_string());
    let world = h.get("Hello");
    if let Some(w) = world {
        assert_eq!(w, "World");
    }
}

#[test]
fn from_vector() {
    // In this example the ownership of the values still belongs to
    // the two vectors, so I need to use references.
    let teams = vec!["Red".to_string(), "Blue".to_string()];
    let scores = vec![0, 100];
    let scoreboard: HashMap<&String, &i32> = teams.iter().zip(scores.iter()).collect();
    if let Some(blue_score) = scoreboard.get(&teams[1]) {
        assert_eq!(**blue_score, 100);
    }
}

#[test]
fn for_in() {
    let mut h = HashMap::new();
    h.insert('a', 1);
    h.insert('b', 2);
    let mut count = 0;
    for (key, value) in &h {
        count += 1;
        if key == &'a' {
            assert_eq!(value, &1);
        }
        if key == &'b' {
            assert_eq!(value, &2);
        }
    }
    assert_eq!(count, 2)
}

#[test]
fn overwriting_values() {
    let mut h = HashMap::new();
    h.insert('a', 1);
    h.insert('a', 2);
    assert_eq!(h.get(&'a'), Some(&2));
}

#[test]
fn only_setting_unset_values() {
    let mut h = HashMap::new();
    h.insert('a', 1);
    h.entry('a').or_insert(2);
    h.entry('b').or_insert(2);
    assert_eq!(h.get(&'a'), Some(&1));
    assert_eq!(h.get(&'b'), Some(&2));
}

#[test]
fn updating_based_on_old_value() {
    let mut h = HashMap::new();
    {
        let a = h.entry('a').or_insert(1);
        *a += 1;
    }
    assert_eq!(h.get(&'a'), Some(&2));
}
