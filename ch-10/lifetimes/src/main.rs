// Lifetimes!
// The compiler calculates "how long" every variable lives:
// If a value goes out of scope before a reference to it does, the code won't compile.
// Rust's borrow checker prevents dangling references --
// pointers to dropped data -- by comparing lifetimes.
//
// * Syntax:
// ? &        i32  // a reference
// ? &'a      i32  // a reference with an explicit lifetime
// ? &'a mut  i32  // a mutable reference with an explicit lifetime
//
// * &'static i32  // a static lifetime will last for the entire runtime

#![allow(dead_code)]

use std::fmt::Display;


fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");

    // * if the code instead was:
    // ? let string1 = String::from("long string is long");
    // ? let result;
    // ?
    // ? {
    // ?     let string2 = String::from("xyz");
    // ?     result = longest(string1.as_str(), string2.as_str());
    // ? }
    // ?
    // * // string2 has been dropped, but result is still in scope.
    // * // the reference longest() returned only lived as long as string2,
    // * // so after the inner scope, it's also been dropped.
    // * // trying to access the borrowed value after that is invalid:
    // ? println!("The longest string is {result}");
}


// here, a generic *lifetime* parameter is defined for the function.
// just like generic *type* parameters, they annotate how long a reference will live.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // since x, y, and the return value all have the same lifetime,
    // it means that the returned value will only live as long
    // as the smaller lifetime between x and y.
    if x.len() > y.len() {
        x
    } else {
        y
    }
}


// generic types, trait bounds, and lifetimes all at once:
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() { x } else { y }
}
