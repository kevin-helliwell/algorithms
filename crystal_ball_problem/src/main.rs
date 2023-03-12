// Given two crystal balls that will break if dropped from high enough distance, determine the exact spot in which it will break in the most optimized way.
// Function returns index of first occurence of true in O(sqrt(n))

pub fn two_crystal_balls(breaks: &[bool]) -> usize {
    // Converts to f32, does math, then converts to usize
    let jump_amount = (breaks.len() as f32).sqrt().floor() as usize;

    let mut i = jump_amount;

    // Loops through array by jump_amount in each leap
    while i < breaks.len() {
        if breaks[i] == true {
            break;
        }
        i += jump_amount;
    }

    // Goes back by one leap to start walking using linear search
    i -= jump_amount;
    let mut j = 0;
    while j <= jump_amount && i < breaks.len() {
        if breaks[i] == true {
            return i;
        }
        i += 1;
        j += 1;
    }

    // sentinel value: No occurence of true within the array
    return usize::MAX;
}

fn main() {
    let test_array = [
        false, false, false, false, false, false, false, true, true, true,
    ];
    println!("{}", two_crystal_balls(&test_array));
}
