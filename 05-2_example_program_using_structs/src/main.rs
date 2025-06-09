#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        // dbg! takes ownership of the exèression and returns the result...
        width: dbg!(30 * scale),
        height: 50,
    };
    // Taking advantage of the debug trait:
    //  - Formatting with ':?' or ':#?'
    //  - Passing the object to the dbg! macro
    println!("rect1 is {rect1:?}");
    println!("rect1 is {rect1:#?}"); // Pritty print

    // ...or we can pass it a reference
    dbg!(&rect1);

    println!(
        "The are of the rectacngle is {} square pixels",
        area(&rect1)
    );
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
