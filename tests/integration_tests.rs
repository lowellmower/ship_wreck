// Note from the book on testing binaries:
// This is one of the reasons Rust projects that provide a binary have a straight
// forward src/main.rs file that calls logic that lives in the src/lib.rs file.
// Using that structure, integration tests can test the library crate by using
// extern crate to exercise the important functionality. If the important
// functionality works, the small amount of code in the src/main.rs file will work
// as well, and that small amount of code doesn’t need to be tested.

extern crate ship_wreck;

#[test]
fn returns_string() {
    let t = ship_wreck::string_return();
    assert_eq!(t, "a mutable string");
}
