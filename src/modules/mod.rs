mod another;

// This is unnecessary for child modules like this
// but this is how you would pull specific things
// into scope. ୧(•̀ᴗ•́๑)ᵒᵏᵎᵎ
use self::another::thing;

#[test]
fn using_modules() {
    assert!(another::public_fn());
    assert_eq!(thing::public_thing(), false);
}
