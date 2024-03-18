/*
* Strings:
* Challenge: Write a function reverse_string(text: &str) -> String that reverses the order of characters in a string.
*
* Example:
* let text = "hello";
* let reversed_text = reverse_string(text);
* assert_eq!(reversed_text, "olleh");
*/

fn reverse_string(text: &str) -> String {
    text.chars().rev().collect()
}

fn main() {
    let text = "hello";
    let reversed_text = reverse_string(text);
    println!("{}", reversed_text);
    assert_eq!(reversed_text, "olleh");
}
