// Errors!
// ----------------------------
// Recoverable:   Result<T, E>
// Unrecoverable: panic!()
// ----------------------------
// ? which one, and when?
// ? https://doc.rust-lang.org/stable/book/ch09-03-to-panic-or-not-to-panic.html

// import the module and specific items from the module:
use std::io::{self, ErrorKind, Read};
use std::fs::File;


fn main() {
    // rust doesn't have exceptions; instead, it has recoverable and unrecoverable erorrs:
    // ? recoverable:   managed with the Result<T, E> enum.
    // ? unrecoverable: caused by the panic! macro and immediately abort execution.

    // * calling panic! deliberately:
    // ! panic!("crash and burn!");
    // ? this will tell rust to stop executing, walk up the call stack, and clean up memory

    // * code that panics:
    // ? let v = vec![1, 2, 3];
    // ! let x = &v[99];  // out of bounds!

    // * Result enum:
    // ? enum Result<T, E> {
    // ?     Ok(T),
    // ?     Err(E),
    // ? }
    // * if the operation works: there will be an Ok variant holding a value of type T
    // * if it fails:            there will be an Err variant holding an error of type E

    let filename = String::from("hello.txt");

    // handle outcomes of attempting to open a file with a match:
    let _ = match File::open(&filename) {
        Ok(file) => file,

        // if there's an error, handle different possibilities:
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(&filename) {
                Ok(new_file) => new_file,
                Err(e) => panic!("Problem creating file: {e:?}."),
            },
            _ => panic!("Problem opening file: {error:?}."),
        },
    };

    // that's a fucking mess holy shit ^^
    // matches are cool but that's very confusing
    // smn smn, closures? .unwrap_or_else()??? that sounds like a threat but ok

    // * will return the inner value of the Ok() case, and panic otherwise:
    // ? Result<T, E>.unwrap();

    // * will panic with the custom message provided:
    // ? Result<T, E>.expect("error message describing expected behavior");

    // even with the ? operator, the Result enum is received here,
    // and must be handle appropriately.
    let _ = match read_from_file(&filename) {
        Ok(username) => username,
        Err(_)       => panic!("shit!"),
    };

    // main() doesn't have to return (), it can also return integers when it exits,
    // like C: non-zero returns means an error ocurred.
}


fn read_from_file(file: &String) -> Result<String, io::Error> {
    let mut username = String::new();

    // * the ? operator:
    // * can be used on Results and Options!
    // - if the value of the Result returned is an Ok(T),
    //   the entire expression will return Ok(t).
    // - if there's an error, the operator exits the function and returns Err(E).
    //   it propagates the error up to the calling function, which must handle it.

    // essentially simplifies error propagation so match statements aren't necessary.
    // however, the specific Err(e) type received must be
    // converted to the function's specified return type.

    // this tries to open the file, and if it works, reads its contents to username.
    // if either operation returns an error, the function exits.
    File::open(file)?.read_to_string(&mut username)?;

    // if everything works, return an Ok with the username String.
    Ok(username)

    // ? this whole function could just be a std::fs::read_to_string() call!
}
