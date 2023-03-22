// O(log(n)) method to find an item from a sorted list of items
// Note: low is inclusive, high is exclusive [low, high)

// Suggested from cargo clippy: https://rust-lang.github.io/rust-clippy/master/index.html#comparison_chain
use std::cmp::Ordering;

// Assumptions:
// (1) low_index, middle_index, and high_index are always >= 0 (usize)
// (2) Values such as needle and the elements within haystack[] are not guaranteed to be >= 0 (i32)
// (3) Values in array are sorted in ascending order

pub fn is_sorted_asc(array: &[i32]) -> bool {
    for i in 0..array.len() - 1 {
        if array[i] > array[i + 1] {
            return false;
        }
    }
    return true;
}

pub fn binary_search(haystack: &[i32], needle: i32) -> bool {
    // Edge case: array is empty
    if haystack.len() == 0 {
        return false;
    }

    // Edge case: array has only one element
    if haystack.len() == 1 {
        return haystack[0] == needle;
    }

    // Check if array is sorted in ascending order
    if is_sorted_asc(haystack) == false {
        panic!("Array is not sorted in ascending order!");
    }

    // Main case: array has 2 or more elements in ascending order
    let mut low_index = 0;
    let mut high_index = haystack.len() - 1;

    while low_index < high_index {
        let middle_index = low_index + (high_index - low_index) / 2;
        let middle_value = haystack[middle_index];

        match middle_value.cmp(&needle) {
            Ordering::Equal => {
                return true;
            }
            Ordering::Greater => {
                high_index = middle_index;
            }
            Ordering::Less => {
                low_index = middle_index + 1;
            }
        }
    }
    false
}

fn main() {
    // Array test case 1

    let test_array_one = [-1, 1, 2, 3, 4, 5];
    println!("Testing Case 1: {:?}", test_array_one);

    // True test case
    let test_number_true_one = 2;
    assert_eq!(binary_search(&test_array_one, test_number_true_one), true);
    println!(
        "{test_number_true_one} exists in array? {}",
        binary_search(&test_array_one, test_number_true_one)
    );

    // False test case
    let test_number_false_one = 0;
    assert_eq!(binary_search(&test_array_one, test_number_false_one), false);
    println!(
        "{test_number_false_one} exists in array? {}",
        binary_search(&test_array_one, test_number_false_one)
    );

    // Array test case 2

    let test_array_two = [-2, 3, 5, 6];
    println!("Testing Case 2: {:?}", test_array_two);

    // True test case
    let test_number_true_two = 5;
    assert_eq!(binary_search(&test_array_two, test_number_true_two), true);
    println!(
        "{test_number_true_two} exists in array? {}",
        binary_search(&test_array_two, test_number_true_two)
    );

    // False test case
    let test_number_false_two = -1;
    assert_eq!(binary_search(&test_array_two, test_number_false_two), false);
    println!(
        "{test_number_false_two} exists in array? {}",
        binary_search(&test_array_two, test_number_false_two)
    );
    
}
