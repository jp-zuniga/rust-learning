// Functions!

// expressions vs. statements:
// ---------------------------------------------------
// - expressions return their values and don't end with semicolons
// - opposite is true for statements; they don't return anything and
//   must end with a semicolon

fn main() -> () {
    // -> () means 'return None'

    println!("Hello, world!");
    print_param(five());
    print_measurement(five(), 'x');
}

// can be defined anywhere, as long as it's in scope for the caller
fn print_param(x: i32) {
    println!("x = {x}");
}

fn print_measurement(value: i32, unit_label: char) {
    println!("Measurement received: {value}{unit_label}");
}

fn five() -> i32 {
    // simply returns the value of the last line
    // equivalent to:
    // return 5;
    5
}
