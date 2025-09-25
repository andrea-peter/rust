struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someemail@example.com"),
        sign_in_count: 1,
    };
    user1.email = String::from("new_email@example.com");
    println!("The email of '{}' is '{}'", user1.username, user1.email);

    // Struct update syntax,
    // username has been moved from user1 to user2,
    // active and sign_in_count have been copied (they implement the Copy trait)
    let user2 = User {
        email: String::from("another_emaill.example.com"),
        ..user1
    };
    println!("User1's email is {}", user1.email);
    println!("User2's email is {}", user2.email);
    println!("User1 has signed in {} time(s)", user1.sign_in_count);
    println!("User2 has signed in {} time(s)", user2.sign_in_count);
    // This will not work as user1's username has been moved to user2
    // (because String does not implement the Copy trait)
    // println!("User1's username is {}", user1.username);

    // Tuple-structs
    struct Color(i32, i32, i32);
    let black = Color(0, 0, 0);
    println!("{}, {}, {}", black.0, black.1, black.2);
    // Destructuring
    let Color(r, g, b) = black;
    println!("{r}, {g}, {b}");
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        // We can write this in short form beause
        // the variable and the struct field have the same name
        // (it does not depends on the position)
        username,
        email,
        sign_in_count: 1,
    }
}
