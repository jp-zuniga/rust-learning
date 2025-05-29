// Slices!


fn main() {
    let s = String::from("Hello, world!");

    // slices are references to a range of indices:
    let hello = &s[0..5];
    let world = &s[7..12];

    println!("{hello}, {world}!");

    // * range syntax:
    // - [0..2]   = [..2]
    // - [3..len] = [3..]
    // - [0..len] = [..]

    // string literals are just slices.
    // since they're stored directly in the binary,
    // they're a slice pointing to their position in it:
    // ? let s: &str = "Literal!"

    println!("First word: {}", first_word(&s));

    // slices of an array:
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}

// the function can take a &String *OR* a &str, to maximize usability
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item.is_ascii_whitespace() || !item.is_ascii_alphabetic() {
            return &s[0..i];
        }
    }

    // return the whole string if no spaces were found:
    s
}
