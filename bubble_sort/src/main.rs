// O(n^2) method for sorting arrays
// Remove () return suggested from cargo clippy: https://rust-lang.github.io/rust-clippy/master/index.html#unused_unit

pub fn bubble_sort(array: &mut [usize]) {
    for i in 0..array.len() {
        for j in 0..(array.len() - 1 - i) {
            // if current element > next element, swap positions
            if array[j] > array[j + 1] {
                let larger_element = array[j]; // store larger element in variable for swap
                let smaller_element = array[j + 1]; // store smaller element in variable for swap
                array[j] = smaller_element; // smaller element -> j position
                array[j + 1] = larger_element; // larger element -> j+1 position
            }
        }
    }
}

pub fn bubble_sort_clone(array: &mut [usize]) -> &mut [usize] {
    //    bubble_sort(array);
    let new_array = array;
    for i in 0..new_array.len() {
        for j in 0..(new_array.len() - 1 - i) {
            // if current element > next element, swap positions
            if new_array[j] > new_array[j + 1] {
                let larger_element = new_array[j]; // store larger element in variable for swap
                let smaller_element = new_array[j + 1]; // store smaller element in variable for swap
                new_array[j] = smaller_element; // smaller element -> j position
                new_array[j + 1] = larger_element; // larger element -> j+1 position
            }
        }
    }
    new_array
}

fn main() {
    let mut test_array = [4, 5, 1, 2, 3];
    bubble_sort(&mut test_array);
    println!("{:?}", test_array); // "{:?}" to display arrays in println! (Debug trait)
    let mut test_array_clone = [4, 5, 1, 2, 3];
    let sorted_array = bubble_sort_clone(&mut test_array_clone);
    println!("{:?}", sorted_array);
}
