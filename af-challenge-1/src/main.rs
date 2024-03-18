/*
* Functions:
* Challenge: Write a function greet(name: &str) -> String that returns a greeting message like "Hello, John!".
* Example:
* let name = "Alice";
* let greeting = greet(name);
* assert_eq!(greeting, "Hello, Alice!");
*/

fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

fn main() {
    let name = "Alice";
    let greeting = greet(name);
    println!("{}", greeting);
    assert_eq!(greeting, "Hello, Alice!");
}
