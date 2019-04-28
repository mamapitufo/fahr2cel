use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let nums = &args[1..];

    println!("the numbers: {:?}", nums);
}
