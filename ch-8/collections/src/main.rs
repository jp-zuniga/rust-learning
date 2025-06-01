// Collections!
// Looking closely at vectors, strings, and hash maps.

use std::collections::HashMap;


fn main() {
    vectors();
    // all vectors will be dropped, including their contents

    strings();
    hash_maps();
}


fn vectors() {
    // * macro for easily creating vectors:
    let v = vec![1, 2, 3, 4, 5];

    // index accessing works like any array,
    // but causes the program to panic if the index is out of bounds
    let second = &v[2];

    // .get() allows safely accessing any possible index of the vector,
    // since it returns an Option<T>. if the index is out of bounds,
    // it just returns None, which must be handled with a match statement.
    let third  = v.get(3);

    println!("Second element: {second}.");

    match third {
        Some(third) => println!("Third element: {third}."),
        None        => println!("There is no third element."),
    }

    // * this line would cause an error:
    // ? v.push(4);
    // ? println!("Second element: {second}.");

    // since an immutable reference to part of the vector is still being used;
    // pushing would require borrowing the vector as mutable.

    // iterating through an immutable reference of a vector:
    for i in &v {
        println!("{i}");
    }
}


fn strings() {
    // * ways to create Strings:
    // ? String::new()
    // ? String::new(&str)
    // ? &str.to_string()

    // * Strings are UTF-8 encoded:
    // - let hello = String::from("السلام عليكم");
    // - let hello = String::from("שלום");
    // - let hello = String::from("नमस्ते");
    // - let hello = String::from("こんにちは");
    // - let hello = String::from("안녕하세요");
    // - let hello = String::from("你好");
    // - let hello = String::from("Здравствуйте");

    let s1 = String::from("Hello,");
    let s2 = String::from("world!");

    //  s1 has been moved!
    let s3 = s1 + &s2;
    //  both strings can't be immutable references;
    //  the second one has to be *appended* to the first one -- s1 has to be moved

    println!("{s3}");

    // these variables are taking ownership of new values;
    // the old ones (e.g: "world!", "Hello, world!") are dropped.
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s  = format!("{s1}-{s2}-{s3}");

    println!("{s}");

    // * Rust Strings don't support indexing:
    // ! let letter = &s[0];  // this doesn't work!

    // the short explanation is:
    // Rust Strings are wrappers around Vec<u8>, but they're UTF-8 encoded.
    // this means that not every "letter" will be exactly 1 byte long.

    // the expected behavior of indexing into a String to get a letter is impossible:
    // not every element of the vector under the hood maps to an actual Unicode character.

    // д takes 2 bytes to store, so it'll be spread across 2 elements of the vector.
    // ते is actually 2 Unicode scalar values: त and 'े -- the character and its diacritic.

    // Strings are stored as bytes, and chars are stored as Unicode scalar values;
    // however, neither of those are what people think of as "letters".

    // trying to index a UTF-8 String stored as a vector of bytes would never be O(1),
    // since it would require piecing together which bytes go together to make a letter.

    // attempting to index one of its Strings could lead to unexpected bugs,
    // so Rust doesn't even let you compile the code that would cause them.
}


fn hash_maps() {
    let mut scores = HashMap::new();

    // types that implement Copy are copied.
    // types that implement Drop are moved, and the hash maps becomes their owner.
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // .get():       tries to get the value of the key given - returns an Option<&T>
    // .copied():    copies the immutable reference given    - returns an Option <T>
    // .unwrap_or(): takes the Option<T> and does a match    - returns Some(T) or 0
    let _ = scores.get("Blue").copied().unwrap_or(0);

    // iteration over a hash map:
    for (key, value) in &scores {
        // the pairs are accessed in an arbitrary/random order
        println!("{key}: {value}");
    }

    // * overwriting the value of a key:
    // ? scores.insert(String::from("Blue"), 40)

    // * adding a key/value pair only if the key doesn't exist:
    // ? scores.entry(String::from("Yellow")).or_insert(50);
    // - .entry() will return an Entry enum, which is a key that *could* exist.
    // - .or_insert() is a method on an Entry that will essentially do a match:
    //    if the key exists, return a mutable reference to its value;
    //    if it doesn't, insert the given value and return a mutable reference to it.
}
