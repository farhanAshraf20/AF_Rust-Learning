/*
* Vectors:
* Challenge: Write a function find_max(data: &Vec<i32>) -> Option<i32> that finds the maximum value in a vector of integers, or returns None if the vector is empty.
* Example:
*
* let data = vec![5, 2, 8, 1];
* let max_value = find_max(&data);
* assert_eq!(max_value, Some(8));

* let data: Vec<i32> = vec![];
* let max_value = find_max(&data);
* assert_eq!(max_value, None);
*/

fn find_max(data: &Vec<i32>) -> Option<i32> {
    data.iter().max().copied()
}
fn main() {
    let data = vec![5, 2, 8, 1];
    let max_value = find_max(&data);
    assert_eq!(max_value, Some(8));

    let data: Vec<i32> = vec![];
    let max_value = find_max(&data);
    assert_eq!(max_value, None);
}
