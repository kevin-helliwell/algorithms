pub fn linear_search(haystack: &[i32], needle: i32) -> bool {
    for i in 0..haystack.len() {
        if needle == haystack[i] {
            return true;
        }
    }
    return false;
}

fn main() {
    let test_array = [1, 2, 3, 4, 5];
    let test_number = 2;
    println!("{}", linear_search(&test_array, test_number));
}
