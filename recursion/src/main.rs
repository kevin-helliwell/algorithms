// Classic example (Gauss' adding numbers 1-100 problem)
pub fn foo(number: usize) -> usize {
    // Base case
    if number == 1 {
        return 1;
    }

    // We shall recurse!
    number + foo(number - 1)
}

// Classic example (Math factorial function)
pub fn factorial(number: usize) -> usize {
    // Base case
    if number == 0 {
        return 1;
    }

    // We shall recurse!
    number * factorial(number - 1)
}

fn main() {
    println!("{:?}", foo(1000)); // 500_500
    println!("{:?}", factorial(5)); // 120
}
