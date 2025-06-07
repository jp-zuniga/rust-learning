// Enums!

#![allow(unused)]  // stops the compiler from complaning :D

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// * the above is equivalent to the following structs:
// ? struct QuitMessage;
// ? struct MoveMessage {
// ?     x: i32,
// ?     y: i32,
// ? }
// ? struct WriteMessage(String);
// ? struct ChangeColorMessage(i32, i32, i32);
// ------------------------------------------------------
// instead of all that, enums allow the grouping of related types under one banner.

// enums can have implementation blocks that apply to all of its variants:
impl Message {
    fn call(&self) {
        // handle each variant of the enum separately:
        match self {
            Message::Quit                 => println!("Quitting!"),
            Message::Write(some_str)      => println!("{some_str}"),
            Message::Move { .. }          => println!("Moving somewhere else!"),
            Message::ChangeColor(_, _, _) => println!("Changing color!")
        }
    }
}

fn main() {
    let m = Message::Write(String::from("Enum!"));
    m.call();

    // using Option<T> explicitly states that the value *could* be null.
    // having to opt into the possibility of null values makes it clear what could go wrong.
    // it clearly tells the compiler to complain when one of the possibilities isn't handled:
    // either the variable has a value, or it doesn't, and they need to be treated separately.
    let possible_num: Option<i32> = Some(3);

    // Option<T> is just an enum.
    // it can be Some or it can be None:
    // either there's a value, or there isn't.
    // by silo-ing the possibility of null into an enum variant,
    // it becomes a different type: T and Option<T> are *not* the same.

    // "if let" syntax simplifies a match statement that only cares about one path,
    // all other variants of the value given don't matter. this trades away exhaustiveness!

    // to not deal with the compiler's warnings, match's is simplified into an if statement:
    // 1) create pattern
    // 2) if it matches the value, execute this code

    if let Some(3) = possible_num {
        println!("Non-null value found!")
    }

    // * used instead of:
    // ? match value {
    // ?     Some(x) => println!(),
    // ?     _       => (),
    // ? }

    // there's also let..else syntax for some weird shit that seems uninituitive
    // ? let Pattern(binding) = value else {
    // ?     // do stuff
    // ? }
    // * binding is available in the current scope now, i guess???
}
