mod another;

use self::another::thing;

#[test]
fn using_modules() {
    assert!(another::public_fn());
    assert_eq!(thing::public_thing(), false);
}
