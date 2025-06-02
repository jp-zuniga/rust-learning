// Practice Exercises:
// --------------------------------------------------------------------------------------
// 1. Given a list of integers, use a vector and return the median and mode of the list.
// --------------------------------------------------------------------------------------
// 2. Convert strings to Pig Latin. The first consonant of each word is moved to the end,
//    and -ay is added ("first" => "irst-fay"). Words that start with a vowel have -hay
//    added to the end ("apple" => "apple-hay").
// --------------------------------------------------------------------------------------
// 3. Using a hash map and vectors, create a text interface to allow a user to add
//    employee names to a department in a company; for example, "Add Amir to Sales."
//    Then let the user retrieve a list of all people in a department or all people
//    in the company by department, sorted alphabetically.

use::std::collections::HashMap;


fn main() {
    let (median, mode) = exercise1(vec![3, 4, 1, 7, 9, 3, 2, 7, 5, 3, 8, 6]);

    println!("Exercise 1:");
    println!("Median: {median}.");
    println!("Mode: {mode}.");

    exercise3::ex3_main();
}


fn exercise1(mut numbers: Vec<i32>) -> (i32, i32) {
    numbers.sort();
    let size = numbers.len();

    let median = {
        if size % 2 == 0 {
            (numbers[size / 2] + numbers[(size / 2) + 1]) / 2
        } else {
            numbers[size / 2]
        }
    };

    let mut num_count: HashMap<i32, i32> = HashMap::new();

    for i in &numbers {
        let key = num_count.entry(*i).or_insert(0);
        *key += 1;
    }

    let mode = num_count.values().max().unwrap();
    (median, *mode)
}


mod exercise3 {
    use std::io::{self, Write};
    use std::collections::HashMap;
    use std::num::ParseIntError;

    pub fn ex3_main() {
        let mut database: HashMap<String, Vec<String>> = HashMap::new();
        let mut option: u8 = 0;

        while option != 4 {
            option = match menu() {
                Ok(num) => {
                    match num {
                        1 => add_employee(&mut database),
                        2 => see_department(&database),
                        3 => see_all(&database),
                        4 => (),
                        _ => println!("Please enter a valid option."),
                    };
                    num
                },
                _ => {
                    println!("Please enter a number that corresponds to a valid option.");
                    option
                },
            }
        }
    }

    fn menu() -> Result<u8, ParseIntError> {
        let mut option = String::new();

        println!();
        println!("Menu:");
        println!("1. Add employee.");
        println!("2. See a specific department's employees.");
        println!("3. See all employees.");
        println!("4. Exit.");
        print!("=> ");
        let _ = io::stdout().flush();

        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read line!");

        option.trim_end().parse::<u8>()
    }

    fn add_employee(database: &mut HashMap<String, Vec<String>>) {
        print!("\nEmployee Name: ");
        let _ = io::stdout().flush();

        let mut name = String::new();
        io::stdin()
            .read_line(&mut name)
            .expect("Failed to read line!");

        print!("Department Name: ");
        let _ = io::stdout().flush();

        let mut dept = String::new();
        io::stdin()
            .read_line(&mut dept)
            .expect("Failed to read line!");

        dept = dept.trim().to_string();
        database.entry(dept).or_default().push(name.trim().to_string());
    }

    fn see_department(database: &HashMap<String, Vec<String>>) {
        print!("\nName of department to inspect: ");
        let _ = io::stdout().flush();

        let mut dept = String::new();
        io::stdin()
            .read_line(&mut dept)
            .expect("Failed to read line!");

        dept = dept.trim().to_string();

        let dept_employees = match database.get(&dept) {
            Some(vec) => vec,
            None => return println!("Error: department name doesn't exist."),
        };

        println!("{dept} Employees:");
        for (num, employee) in dept_employees.iter().enumerate() {
            println!("{}. {employee}.", num + 1);
        }
    }

    fn see_all(database: &HashMap<String, Vec<String>>) {
        for (dept, employees) in database.iter() {
            println!("\n{dept} Employees:");
            for (num, employee) in employees.iter().enumerate() {
                println!("{}. {employee}.", num + 1);
            }
        }
    }
}
