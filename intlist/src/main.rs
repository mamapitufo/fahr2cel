use std::env;
use std::iter::Iterator;

fn main() {
    let mut args: Vec<i32> = env::args()
        .skip(1)
        .filter_map(|arg| arg.trim().parse().ok())
        .collect();
    args.sort();

    let numbers = args.clone();

    println!("Anything that is not an integer is silently discarded!\n");
    println!("For the list {:?}:", numbers);
    println!("the mean is: {}", mean(&numbers));
}

fn mean(numbers: &[i32]) -> f64 {
    let sum: i32 = numbers.iter().sum();

    f64::from(sum) / numbers.len() as f64
}
