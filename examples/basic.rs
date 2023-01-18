use std::str::FromStr;

use tinyid::TinyId;

fn main() {
    // Create a random ID
    let rand_id = TinyId::random();

    // Parse a string into a Result<TinyId, TinyIdError> for possibly unsafe / invalid ID strings
    let maybe = TinyId::from_str("AAAABBBB");
    assert!(maybe.is_ok());
    let bad = TinyId::from_str("AAAABBB");
    assert!(bad.is_err());

    // Parse a string you **KNOW** is safe into a TinyId
    let parsed = TinyId::from_str_unchecked("AAAABBBB");

    // All expected operations are available on TinyIds
    // Equality is a simple byte comparison so it should be fast and cheap!
    assert_eq!(maybe.unwrap(), parsed);

    // IDs may be printed using Display or Debug
    println!("Random ID: {rand_id}");
    println!("Parsed ID: {parsed}");
    println!(" Debug ID: {parsed:?}");

    // IDs are small!
    println!("TinyID Size: {}", std::mem::size_of::<TinyId>());

    // IDs are case-sensitive
    let parsed2 = TinyId::from_str_unchecked("aaaaBBBB");
    assert_ne!(parsed, parsed2);

    // Check whether an ID starts with a given string. Example use case would be providing a
    // list of IDs to a user, and asking for a partial string to match against so the user
    // doesn't have to type the entire thing.
    assert!(parsed.starts_with("AAAA"));
    assert!(parsed.ends_with("BBBB"));
    assert!(!parsed.starts_with("BBBB"));

    // IDs are copied when assigned.
    let mut switched = parsed;
    assert_eq!(switched, parsed);

    // Validity can be checked, and a "marker" exists for null / invalid IDs.
    assert!(switched.is_valid());
    assert!(!switched.is_null());
    assert_ne!(switched, TinyId::null());
    // Mutable IDs can be made null. This change has no effect on the `parsed` variable.
    switched.make_null();
    assert!(!switched.is_valid());
    assert!(switched.is_null());
    assert_eq!(switched, TinyId::null());
}
