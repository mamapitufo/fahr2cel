use std::collections::HashMap;
use std::env;
use std::iter::Iterator;

fn main() {
    let arg_count = env::args().len() - 1;

    if arg_count == 0 {
        usage_and_exit("Error: You did not provide any integers.");
    }

    let mut args: Vec<i32> = env::args()
        .skip(1)
        .filter_map(|arg| arg.trim().parse().ok())
        .collect();

    if arg_count > args.len() {
        usage_and_exit("Error: at least one of the arguments provided is not an integer.");
    }

    args.sort_unstable();
    let numbers = args.clone();

    println!("For the integers {:?},", numbers);
    println!("  mean: {:.*}", 2, mean(&numbers));
    println!("  median: {}", median(&numbers));
    println!("  mode: {}", mode(&numbers));
}

fn usage_and_exit(msg: &str) {
    eprintln!("{}\nUsage: intlist [INTEGER ...]", msg);
    std::process::exit(2);
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

fn mode(numbers: &[i32]) -> i32 {
    let mut freqs = HashMap::new();

    for n in numbers {
        let count = freqs.entry(n).or_insert(0);
        *count += 1;
    }

    let mut max = 0;
    let mut mode: i32 = -1;

    for (n, count) in freqs.iter() {
        if *count >= max {
            max = *count;
            mode = **n;
        }
    }

    mode
}
