// Loops and Conditionals!

fn main() {
    let number = 3;

    // conditions must evaluate to bools
    if number < 5 {
        println!("true!");
    } else if number > 10 {
        println!("else if!");
    } else {
        println!("false!");
    }

    let condition = true;

    // since an if is an expression, its return value can be assigned

    // in this case, the return values of both arms must be the same type
    let n = if condition { 5 } else { 6 };

    let calculation = loop_fn(n);

    println!("n = {n}");
    println!("loop test: {}", calculation);
    println!("--------------------------------------------");

    // doesn't change the value of the variable!
    countdown(calculation);
    println!("calculation = {calculation}");
    println!("--------------------------------------------");

    print_array([10, 20, 30, 40, 50]);
}


fn loop_fn(limit: i32) -> i32 {
    let mut counter = 0;

    // the function returns what the loop returns
    loop {
        counter += 1;
        if counter == limit {
            // the loop returns counter
            break counter;
        }
    }
}


// ? will this change the value of the variable passed to it?
fn countdown(counter: i32) {
    // (1..x) generates a range from 1 (inclusive) to x (exclusive)
    // .rev() then reverses that range
    for number in (1..counter + 1).rev() {
        println!("{number}!");
    }

    // same as range-based for loop above:
    // ---------------------------------------
    // while counter != 0 {
    //     println!("{counter}!");
    //     counter -= 1;
    // }

    println!("LIFTOFF!!!");
}


// must get an array of 5 ints
fn print_array(arr: [i32; 5]) {
    println!("Array elements:");

    for num in arr {
        println!("{num}");
    }
}
