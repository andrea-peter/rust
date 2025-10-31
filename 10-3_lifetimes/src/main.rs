// This prevents cargo and rust-analyzer from warning about unused stuff
#![allow(unused)]

use std::fmt::Display;

fn main() {
    // Example of lifetime problem, reference to dead variable ################
    let r;
    {
        let x = 5;
        r = &x;
    }
    // x is not alive anymore
    // println!("r: {r}");

    // This works #############################################################
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");

    // This will not work ####################################################
    let result;
    {
        let string3 = String::from("Another string");
        result = longest(string1.as_str(), string3.as_str());
    }
    // Trying to use result after string3 has gone out of scope,
    // this does not work because the lifetime of the return value
    // is the same as the two parameters
    // println!("The longest string is {result}");

    // Static lifetime ########################################################
    // Static lifetime corresponds to the whole live of the program,
    // all string literals have 'static lifetime
    let s: &'static str = "I have a static lifetime";
}

// Lifetime annotation in function signature ##################################
/// This does not compile
/// ```rust
/// fn longest(x: &str, y: &str) -> &str {
///     if x.len() > y.len() { x } else { y }
/// }
/// ```
/// we must define the lifetimes between the different references
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// No need to annotate y since x is always returned
fn return_first<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

// Lifetime annotations in struct defs ########################################
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
    // Lifetime elsion rules make lifetime assignment
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {announcement}");
        self.part
    }

    // Here we need to assign the same lifetime to announcement
    // and the return value explicitly,
    // otherwise the 1st and 3s elsion rules would make the compilation fail
    fn announce_and_return_announcement<'b>(&self, announcement: &'b str) -> &'b str {
        println!("Attention please: {announcement}");
        announcement
    }
}

// Generic type parameters, trait bounds and lifetimes together
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
// This bound could be short cut as "<'a, T: Display>" above
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() { x } else { y }
}
