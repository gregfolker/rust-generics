// Project: rust-generics
// Author: Greg Folker

use std::fmt::Display;

// A function that abstracts an operation
// without using generics that may need to
// be done more than once in a program
//
// The disadvantage of this approach is
// it will only work for one type, `i32`
fn largest_i32(list: &[i32]) -> &i32 {
    let mut n = &list[0];

    for i in list {
        if i > n {
            n = i;
        }
    }

    n
}

// Using a generic type, we can abstract
// the types of a `struct`, for example
struct Point<T> {
    _x: T,
    _y: T,
}

// Generics can be used to make it such
// that variables in a `struct` are
// different types as well
struct Point2<T, U> {
    _x: T,
    _y: U,
}

// Methods can be implemented on structs
// and enums that use generic typing as well
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self._x
    }
}

fn main() {
	println!("Hello, World!");

    let i32_list = vec![34, 50, 25, 100, 65];

    let largest_number = largest_i32(&i32_list);

    println!("largest_int is {}", largest_number);

    let i32_list = vec![102, 34, 6000, 89, 52, 2, 43, 8];

    let largest_number = largest_i32(&i32_list);

    println!("largest_int is {}", largest_number);

    let _integer_point = Point { _x: 5, _y: 10 };
    let _float_point = Point { _x: 1.1, _y: 5.2 };

    let _integer_and_float_point = Point2 { _x: 5, _y: 7.2 };

    let p = Point { _x: 5, _y: 10 };

    println!("p.x = {}", p.x());

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);

    println!("The longest string is {}", result);

    // `static` lifetimes indicate that a reference can
    // live for the entire lifespan of the program. These
    // are stored directly in the program binary
    let s: &'static str = "I will live forever!";

    println!("{}", s);

    let result = longest_with_an_announcement(
        string1.as_str(),
        string2,
        "Today is someone's birthday!",
    );

    println!("{}", result);
}

// Generic Lifetimes inside Functions
//
// This can be accomplished using 'lifetime annotation syntax'
//
// Note that lifetime annotations do not change how long a
// reference lives; they just describe the relationship of
// multiple references to each other without affecting any
// actual lifetimes for those references
//
// Example Syntax:
// &i32;            // A reference
// &'a i32;         // A reference with an explicit lifetime
// &'a mut i32;     // A mutable reference with an explicit lifetime
//
// Lifetime annotations in function signatures need to be used when
// you have no way of knowing the concrete lifetime of the references
// that are being passed as parameters. For example, in the
// function below, we don't know if the `if` or the `else` will
// be executed and therefore cannot guarentee the reference we actually
// return will be valid. This is why a 'lifetime' parameter is required
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest_with_an_announcement<'a, T>(
    x: &'a str,                         /* This block is still the function definition, */
    y: &'a str,                         /* it's just broken up into multiple lines */
    ann: T,                             /* Lines 123-125 are the parameters and Line 126 is the return type */
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

