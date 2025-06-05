// Generics!
// With no runtime overhead!

#![allow(unused)]


struct Point<T> {
    x: T,
    y: T,
}

// must specify that this impl block is with the T generic,
// so that it can be stated that it's for Points of type T.
impl<T> Point<T> {
    // method defined for all instances of Point, with any type T
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    // method defined *only* for instances of Point with f32 attributes
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}


fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);

    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);

    println!("The largest char is {result}");
}


fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    // generic function designed to take a list of elements of any type,
    // as long as that type implements std::cmp::PartialOrd,
    // since the body of the function compares elements with >.

    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
