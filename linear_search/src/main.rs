// O(n) method to find an item from a list of items
// Changes implemented that clippy suggested https://rust-lang.github.io/rust-clippy/master/index.html#needless_range_loop

pub fn linear_search(haystack: &[usize], needle: usize) -> bool {
    for &i in haystack {
        if needle == haystack[i] {
            return true;
        }
    }
    false
}

fn main() {
    let test_array = [1, 2, 3, 4, 5];
    let test_number = 2;
    println!("{}", linear_search(&test_array, test_number));
}
