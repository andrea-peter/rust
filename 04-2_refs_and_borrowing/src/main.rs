fn main() {
    // References can be "borrowed" without having to relinquish ownership
    let s1 = String::from("Hello");
    let len = calculate_length(&s1);
    println!("The length of '{s1}' is {len}");

    let mut s2 = String::from("Hello");
    // Notice the `mut` keyword
    change(&mut s2);
    println!("String has been mutated to '{s2}'");

    // Either many immutable references or one mutable reference
    let mut s3 = String::from("Hello");
    let r31 = &mut s3;
    println!("{r31}");
    let r32 = &mut s3;
    println!("{r32}");
    // This will not work
    // println!("{r1}");

    // Cannot make an immutable reference if a mutable reference already exists
    let r33 = &s3;
    // This will not work
    // println!("{r31}, {r33}");

    // Artificially restricting reference scope
    let mut s4 = String::from("Hello");
    {
        let r41 = &mut s4;
    }
    let r42 = &mut s4;
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // Here, s goes out of scope, but because s does not have ownership
// of what it refers to, the value is not dropped

// We must specify the String reference as mutable
// in order to be able to change it
fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}

// This does not work, we return a reference to an object which goes out of scope
// fn dangle() -> &String {
//     let s = String::from("Hello");
//     &s // Here we return a reference to s
// } // Here, s goes out of scope

// This works, ownership is given to the caller of the function
fn no_dangle() -> String {
    String::from("Hello")
}
