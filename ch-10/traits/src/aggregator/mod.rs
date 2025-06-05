// Example crate to display summaries of news articles and social media posts.

pub mod news;
pub mod social;

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        // default implementation for all types implementing Summary:
        format!("Read more from {}...", self.summarize_author())
    }
}


pub fn notify(item: &impl Summary) {
    // the item parameter can be *any* type that implements Summary
    // only methods defined by the trait can be used
    println!("Breaking news!\n{}", item.summarize());
}

// * the following function definitions are equivalent:
// ? pub fn notify(item: &impl Summary);
// ? pub fn notify<T: Summary>(item: &T);
// - "impl Trait" is syntactic sugar for the second definition:
//    instead of having to specify a generic type,
//    it can just be stated that the type must implement Trait,
//    which implies a generic type in shorter syntax.
//
// - multiple trait bounds can be given:
// ? pub fn notify(item: &(impl Summary + Display));
//
// - if multiple generic types and trait bounds are flying around,
//   the where keyword is more appropriate:
// ? fn some_function<T, U>(t: &T, u: &U) -> i32
// ? where
// ?     T: Display + Clone,
// ?     U: Clone + Debug,
// ? {
//
// - impl Trait can also be used to specify a general return type for a function:
// ? fn some_function<T, U>(t: &T, u: &U) -> impl Summary;
//
// * trait bounds can also be used to define impl blocks:
// ? impl<T: Display + PartialOrd> Pair<T> { }
// - the code within that impl block will only be accessible
//   to Pair instances of a type, T, that implements *both*
//   Display and PartialOrd.
//
// * trait bounds *can also be used* to implement traits on types that implement other traits:
// ? impl<T: Display> ToString for T { }
// - this impl block will implement ToString for any type that also implements Display.
// - "blanket implementations" like this are used across the standard library.
