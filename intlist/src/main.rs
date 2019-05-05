use std::env;
use std::iter::Iterator;

fn main() {
    let mut args: Vec<i32> = env::args()
        .skip(1)
        .filter_map(|arg| arg.trim().parse().ok())
        .collect();
    args.sort_unstable();

    let numbers = args.clone();

    println!("Anything that is not an integer is silently discarded!\n");
    println!("For the list {:?}:", numbers);
    println!("the mean is: {}", mean(&numbers));
    println!("the median is: {}", median(&numbers));
}

fn mean(numbers: &[i32]) -> f64 {
    let sum: i32 = numbers.iter().sum();

    f64::from(sum) / numbers.len() as f64
}

fn median(sorted: &[i32]) -> f64 {
    let count = sorted.len();
    let mid = count / 2 - 1;

    if count % 2 == 0 {
        (sorted[mid] + sorted[mid + 1]) as f64 / 2.0
    } else {
        sorted[mid + 1] as f64
    }
}
