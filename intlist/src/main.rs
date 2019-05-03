use std::env;
use std::iter::Iterator;

fn main() {
    let numbers: Vec<i32> = env::args()
        .skip(1)
        .filter_map(|arg| arg.trim().parse().ok())
        .collect();

    println!("Non-numbers are filtered out!");
    println!("For the list {:?}:", numbers);
    println!("the mean is: {}", mean(&numbers));
}

fn mean(numbers: &[i32]) -> f64 {
    let sum = numbers.iter().fold(0, |acc, n| acc + n);

    f64::from(sum) / numbers.len() as f64
}
