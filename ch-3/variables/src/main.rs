// Mutability!

// Variables are immutable by default,
// and must be declared with mut to be changed.

fn main() {
    // type *must* be specified beforehand for constants
    const SPEED_OF_LIGHT: i32 = 299_792;

    let x: i32 = 5;
    let x: i32 = SPEED_OF_LIGHT + x;

    {
        let x: i32 = x - SPEED_OF_LIGHT / 2;
        println!("x in inner scope: {x}");
    }

    println!("x in outer scope: {x}");

    // immutability and shadowing allows the reuse of
    // the same variable name to hold a different type:
    let spaces: &'static str = "   ";
    let spaces: usize = spaces.len();

    // a char can be any unicode value
    let int_symbol: char = 'ℤ';

    println!("{int_symbol}: {spaces}!");

    // a tuple's elements don't have to be the same type
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // unpacking and discarding:
    let (_, y, _) = tup;

    // index accessing:
    println!("x: {}; y: {y}; z: {}", tup.0, tup.2);

    // fixed-size array with type annotation
    // unlike vectors, all elements must have the same type
    let a: [i32; 3] = [1, 2, 3];

    // index accessing:
    // rust has runtime bounds-checking!
    println!("[{}, {}, {}]", a[0], a[1], a[2]);
}
