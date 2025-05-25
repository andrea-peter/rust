fn main() {
    // #### Moving
    let name = String::from("Mike");
    println!("Hello {name}!");

    let name2 = name;
    println!("Hello {name2}!");

    // This is invalid! Ownership of the string has been *moved* to name2
    // println!("Hello {name}!");

    // #### Cloning ##########################################################
    let clone = name2.clone();
    println!("Hello {name2} and its clone {clone}");

    // #### The *Copy* trait #################################################
    // TODO

    // #### Function calling #################################################
    let my_string = String::from("The string");

    println!("I'm the owner of {my_string}");
    takes_ownership(my_string);

    // The string has been moved to the function's scope
    // println!("I'm no longer the owner of {my_string}");

    let my_integer = 5;
    println!("I'm the owner of {my_integer}");
    makes_copy(my_integer);
    println!("I'm still the owner of {my_integer}");

    let my_string = gives_ownership();
    println!("Got string {my_string}");

    let my_other_string = takes_and_gives_back(my_string);
    println!("Owner of {my_other_string}");

    // We lost ownershup of my_string when calling takes_and_gives_back()
    // println!("Still owner of {my_string}");

    // #### References and borrowing
}

fn takes_ownership(some_string: String) {
    println!("Got ownership of the string {some_string}");
}

// Calling this method makes a copy of the integer because i32 implements the *Copy trait*
fn makes_copy(some_integer: i32) {
    println!("Got ownership of a copy of the integer {some_integer}");
}

fn gives_ownership() -> String {
    String::from("yours")
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
