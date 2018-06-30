extern crate rand;

#[test]
fn result() {
    fn might_error() -> Result<i32, String> {
        if rand::random() {
            return Ok(1);
        }
        Err("it borked".to_string())
    }

    let did_it_work = might_error();

    if let Ok(num) = did_it_work {
        assert_eq!(num, 1);
    } else {
        assert_eq!(did_it_work.unwrap_err(), "it borked");
    }
}
