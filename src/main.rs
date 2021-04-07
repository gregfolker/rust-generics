// Project: rust-generics
// Author: Greg Folker

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
}
