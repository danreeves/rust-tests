#[test]
fn construction() {
    let a = "stringy".to_string();
    let b: String = "stringy".into();
    let c = String::from("stringy");
    assert_eq!(a, b);
    assert_eq!(a, c);
}

#[test]
fn push() {
    let mut lo = "lo".to_string();
    lo.push('l'); // Push takes a char
    assert_eq!(lo, "lol");
}

#[test]
fn push_str() {
    let mut lol = "lol".to_string();
    lol.push_str("cat"); // push_str takes a string slice
    assert_eq!(lol, "lolcat");
}

#[test]
fn concatenation() {
    let lol = "lol".to_string();
    let cat = "cat".to_string();
    let lolcat = lol + &cat;
    assert_eq!(lolcat, "lolcat");
}

#[test]
fn format_macro() {
    let lol = "lol".to_string();
    let cat = "cat".to_string();
    let lolcat = format!("{}{}", lol, cat);
    assert_eq!(lolcat, "lolcat");
}

#[test]
fn bytes() {
    let x = "lolcat".to_string();
    assert_eq!(x.len(), 6);

    let y = "Malmö".to_string();
    assert_eq!(y.len(), 6);

    let z = &y[0..1];
    assert_eq!(z, "M");

    let a = &y[4..6];
    assert_eq!(a, "ö");
}

#[test]
fn chars() {
    let x = "Malmö".to_string();
    let mut chars: Vec<char> = Vec::new();
    for c in x.chars() {
        chars.push(c)
    }
    assert_eq!(chars, vec!['M', 'a', 'l', 'm', 'ö']);
}
