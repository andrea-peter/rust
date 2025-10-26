// Bad design, we don't want to create a function for every type
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// Better design with generic type T,
// we restrict T to those types that implement the `PartialOrd` trait,
// to be able to compare them
//
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// Struct with generic type,
// this point can have coordinates of any type
struct Point<T> {
    x: T,
    y: T,
}

// We can specify several generic types
struct MixedPoint<T, U> {
    x: T,
    y: U,
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&number_list);
    println!("The largest number is {result}");

    let result = largest(&char_list);
    println!("The largest char is {result}");

    let point_int = Point { x: 2, y: 3 };
    let point_float = Point { x: 2.0, y: 3.0 };

    // This will not compile, x and y must be of same type
    // let mixed_point = Point { x: 2, y: 3.0 };

    let mixed_point = MixedPoint { x: 2, y: 3.0 };
}
