fn main() {
    // The problem: Reference contiguous sequence
    // of elements on a collection
    let phrase = String::from("The phrase");
    println!(
        "The first word in '{phrase}' is at index {}",
        first_word(&phrase)
    );

    // String slices
    let s = String::from("Hello world");
    // [inclusive..exclusive]
    let hello = &s[0..5]; // or [..5]
    let world = &s[6..11]; // or [6..]
    // This would compile, but would fail on runtime
    // let world = &s[6..12];  // Index out of bounds
    println!("{hello} {world}");

    let whole_string = &s[..];
    println!("{}", whole_string);

    // This will fail as slices use char-boundaries
    // let _fancy = String::from("Fäncy string");
    // let _ = &_fancy[0..2]; // Cannot slice through char boundaries

    println!(
        "First word in '{phrase}' is '{}'",
        first_word_slice(&phrase)
    );

    // Thanks to the fact that slices are references,
    // bus like these are caught during compile time
    // (cannot have a mutable and an immutable reference at the same time)
    // let mut some_phrase = String::from("Hello world!");
    // let word = first_word_slice(&some_phrase);
    // some_phrase.clear();
    // println!("The first word in '{some_phrase}' is {}", word);
}

/// Return index of first word in space separated phrase
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

/// Return first word in given space separated phrase
/// `phrase` being `&str` instead of `&String` allows
/// for the function to be called with `&str` and `&String`
/// thanks to "deref coercion"
fn first_word_slice(phrase: &str) -> &str {
    let bytes = phrase.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &phrase[0..i];
        }
    }
    phrase
}
