/**
* Match Expressions:
* Challenge: Write a function get_day_type(day: u8) -> &str that uses a match expression to return a string representing the day type based on the day number (1-7):
*
* 1: "Weekend"
* 2-6: "Weekday"
* 7: "Weekend"
* Invalid input: "Invalid Day"
*
* Example:
* let day = 3;
* let day_type = get_day_type(day);
* assert_eq!(day_type, "Weekday");

* let day = 8;
* let day_type = get_day_type(day);
* assert_eq!(day_type, "Invalid Day");
*/

fn get_day_type(day: &u8) -> &str {
    match day {
        1|7 => "Weekend",
        2..=6 => "Weekday",
        _=> "Invalid Day",
    }
}

fn main() {
    let day = 3;
    let day_type = get_day_type(&day);
    println!("{}", day_type);
    assert_eq!(day_type, "Weekday");

    let day = 8;
    let day_type = get_day_type(&day);
    println!("{}", day_type);
    assert_eq!(day_type, "Invalid Day");
}
