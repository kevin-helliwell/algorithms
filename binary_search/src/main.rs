// O(log(n)) method to find an item from a sorted list of items
// Note: low is inclusive, high is exclusive [low, high)

use std::cmp::Ordering; // Suggested from cargo clippy: https://rust-lang.github.io/rust-clippy/master/index.html#comparison_chain

pub fn binary_search(haystack: &[usize], needle: usize) -> bool {
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
    let test_array = [1, 2, 3, 4, 5];
    let test_number = 2;
    println!("{}", binary_search(&test_array, test_number));
}
