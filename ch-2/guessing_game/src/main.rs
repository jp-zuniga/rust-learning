// Guessing Game!

use std::cmp::Ordering;
use std::io;

use rand::Rng;


fn main() {
    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    // println!();
    println!("Guess the number!");
    // println!("(It's {secret_number}, but shhhhh!)");

    loop {
        // println!();
        println!("Please input your guess: ");

        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // println!();
        println!("You guessed {guess}...");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("That's too small!"),
            Ordering::Greater => println!("That's too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
