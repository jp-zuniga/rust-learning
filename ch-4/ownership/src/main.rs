// Ownership!

// da rulez:
// - every value in rust has an owner
// - every value can only have one owner at a time
// - when the owner goes out of scope, the value is dropped and its memory freed


fn main() {
    // s isn't valid here, it hasn't been declared

    {
        // s is valid from here on
        let mut s = String::from("hello");

        // ::from() automatically requests memory from the heap.
        // unlike string literals, which are stored on the stack,
        // Strings must be allocated on the heap, since they're mutable

        s.push_str(", world!");
        println!("{s}");

        // * the value of 5 is copied and both variables are pushed to the stack
        // * ---------------------------------------------------------------------
        // ? let x = 5;
        // ? let y = x;

        // * s2 receives a copy of s1's 'metadata' (pointer, length, capacity),
        // * but it still points to the same address on the heap that holds s1's value
        // * ---------------------------------------------------------------------
        // ? let s1 = String::from("hello");
        // ? let s2 = s1;
        // * ---------------------------------------------------------------------

        // however, in this case, s1 is invalidated when s2 is created.
        // if it wasn't, s1 and s2 would both go out of scope at the same time,
        // and try to free the same memory at the same time (double free error).

        // the invalidation of s1 means s2 isn't a copy at all:
        // the data bound to s1 is *moved* to s2, who becomes its owner
    }

    // the scope is over, and s is no longer valid
    // * drop() is called automatically to free the memory s was using

    // ! ---------------------------------------------------------------------

    // ? let mut s = String::from("hello");
    // ? s = String::from("ahoy");

    // here, s is immediately assigned a new value,
    // which means no one owns the original "hello" value anymore,
    // so rust considers it to have gone out of scope and frees its memory

    // ? println!("{s}, world!");
    // * ahoy, world!

    // ! ---------------------------------------------------------------------

    // since an int's size is known at compile time, it's pushed to the stack.
    // copying its value and pushing it again is cheap, so rust doesn't bother
    // invalidating x after it gets copied into y; both own separate items
    // on the stack, that happen to have the same value copied across.
    let x = 5;
    let y = x;

    // under the hood, simple types that don't require allocations implement Copy.

    // anything that can change at runtime and is allocated on the heap *can't*
    // implement Copy, and must instead implement Drop, to handle memory
    // collection when the value goes out of scope.

    // Copy and Drop are 'traits', which sound like dunder methods but who knows

    println!("x = {x}, y = {y}");

    // ! ---------------------------------------------------------------------

    // s comes into scope
    let s = String::from("hello");

    // but then gets moved to the function
    takes_ownership(s);
    // and is dropped after the function's scope ends
    // so s is no longer valid here

    // x comes into scope
    let x = 5;

    // since i32's have Copy, x is copied, not moved
    makes_copy(x);
    // and the function's copy is dropped

    // but the value of x in this scope can still be used
    println!("{x}");

    // this would cause a compile error:
    // println!("{s}");

    // ! ---------------------------------------------------------------------

    {
        // the value returned by gives_ownership() is moved into
        // the current scope, and the new variable takes ownership of it
        let visitor = gives_ownership();

        // a new string comes into scope
        let new_string = String::from("i'm new!");

        // and is then moved to the scope of the called function,
        // who then returns it back here so it may live on
        let still_kicking  = takes_and_gives_back(new_string);

        // new_string no longer owns a value, so accessing it would cause an error
        println!("{visitor} + {still_kicking}!");
    }

    // after the scope ends, visitor and still_kicking are dropped,
    // but since new_string was *moved* after being given to the function,
    // still_kicking took on ownership, so nothing happens to new_string
}


fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{some_string}");
    // then leaves and is dropped
}


fn makes_copy(some_int: i32) {
    // some_int comes into scope
    println!("{some_int}");
    // then leaves and is dropped
}


fn gives_ownership() -> String {
    // some_string is created
    let some_string = String::from("yours!");

    // and is sent off back to whoever called the function,
    // out of this scope so it may live on in another
    return some_string;
}


fn takes_and_gives_back(some_string: String) -> String {
    // some_string comes into scope
    // stuff is done with it

    // it's sent back to whence it came
    return some_string;
}
