/**
* Challenge: Write a function get_first_character(text: &str) -> Option<char> that returns the first character of the string as a char value (if it's not empty), or None if the string is empty.

* Example:
* let text = "Rust";
* let first_char = get_first_character(text);
* assert_eq!(first_char, Some('R'));

* let text = "";
* let first_char = get_first_character(text);
* assert_eq!(first_char, None);
**/

fn get_first_character(text: &str) -> Option<char> {
    text.chars().next()
}
fn main() {
    let text = "Rust";
    let first_char = get_first_character(text);
    assert_eq!(first_char, Some('R'));

    let text = "";
    let first_char = get_first_character(text);
    assert_eq!(first_char, None);
}
