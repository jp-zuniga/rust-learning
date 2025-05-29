fn main() {
    let s1 = String::from("test!");

    // &: reference operator
    // *: dereference operator

    // the function *borrows* our string with a reference
    // ownership isn't given, and values aren't moved
    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");

    // * this function throws an error:
    // ! it's trying to change a value it doesn't own
    // fn change(some_string: &String) {
    //     some_string.push_str(", world!");
    // }

    // * references, like variables, are immutable by default

    let mut s2 = String::from("Hello");

    // this *will* work, because both the variable and the reference are mutable
    change(&mut s2);

    // ! this isn't allowed !!!
    // ? let r1 = &mut s;
    // ? let r2 = &mut s;
    // ? println!("New reference: {r1}.")

    // only *one* mutable reference to a variable is allowed at any given time.
    // specifically, the error above occurs because a new mutable reference
    // is created before the use of the first one. by preventing mutation
    // from multiple sources, race conditions are avoided.

    // ! --------------------------------------------------------------------------------

    // can't mix and match mutable and immutable references:
    let r1 = &s2;
    let r2 = &s2;

    // this would cause an error:
    // ? let r3 = &mut s;

    // the creation of an immutable reference implies the original value won't change.
    // allowing the creation of a mutable reference would allow for that,
    // which would lead to RUIN!!!

    println!("Immutables: {r1}, {r2}");
    println!("All good!");

    // however, the lifetime of a reference ends with its last use.
    // that means after all immutable references have been used,
    // (and, therefore, dropped) new mutable references can be made:

    let r3 = &mut s2;
    println!("Now a mutable: {r3}");

    // * dangling references:
    // - if the memory a pointer is pointing to is freed while the pointer still exists,
    //   it's left "dangling": it can be used to access uncontrolled memory outside the program

    // rust prevents these by not allowing values to go out of scope
    // before references to them have already been dropped.

    // ! this would return a dangling pointer and a compile-time error:
    // fn dangle() -> &String {
    //     let s = String::from("hello");
    //     return &s;
    // }

    // the above function's scope would end, dropping the local variables,
    // while also returning a reference to one of those variables.
    // the compiler will yell at you if you do this.
}


fn calculate_length(s: &String) -> usize {
    // s, by virtue of being a reference, doesn't own the value it points to.
    return s.len();
    // when it goes out of scope, only *it* is dropped, not the string it points to.
}


fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}
