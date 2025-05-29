// Structs!

struct User {
    active: bool,
    sign_in_count: u64,

    // ? why do these have to be of type String and not &str?
    // - to deliberately make instances of this own all of their data.
    // - structs can store references to values they don't own, but it gets complicated.
    username: String,
    email: String,
}

// tuple struct declataration:
// - no attribute names, just types.
// - even if all the attribute types are the same, Color and Point are separate types.

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);


// * unit-like struct with no field:
// - essentially a custom type with no inherent data
// ? struct AlwaysEqual;


fn main() {

    let user1 = User {
        active: true,
        sign_in_count: 11,
        username: String::from("my_user123"),
        email: String::from("my_email@example.com"),
    };

    // * This would only work if the entire user is mutable;
    // ? user1.username = String::from("new_name321");
    // - a struct can't mix and match mutable and immutable attributes.

    let user2 = build_user(
        String::from("user-2"),
        String::from("email_two@example.com")
    );

    let user3 = User {
        email: String::from("new_name@email.com"),
        ..user1  // fill unspecified attributes with values from another instance
    };

    print_user(&user2);
    print_user(&user3);

    // * because user1 contains a String, when user3 is created with values from user1,
    // * the username String is *moved*, not copied, so it can't be used anymore:
    // ! println!("{}", user1.username);  // borrows a moved value!

    // - this would still be valid, since the int and bool attributes were *copied* to user2
    // ? println!("{}", user1.sign_in_count);
    // - if part of user1 is now invalidated, should the whole instance be invalidated?

    let black = Color(0, 0, 0);
    let origin = Point(black.0, black.1, black.2);

    println!("Test Point: {}, {}, {}", origin.0, origin.1, origin.2);
}


fn build_user(username: String, email: String) -> User {
    // because the username and email parameters have the same
    // name as the struct attributes, they don't have to be repeated
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}


fn print_user(user: &User) {
    println!("User: {}", user.username);
    println!("- Email: {}", user.email);
    println!("- Active: {}", match user.active { true => "Yes", false => "No"});
    println!("- Sign-in Count: {}", user.sign_in_count);
    println!();
}
