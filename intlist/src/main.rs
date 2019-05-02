use std::env;

fn main() {
    let numbers: Vec<i32> = env::args()
        .skip(1)
        .filter_map(|arg| arg.trim().parse().ok())
        .collect();

    println!("The numbers: {:?}", numbers);
}
