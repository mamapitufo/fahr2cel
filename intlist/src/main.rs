use std::env;

fn main() {
    let mut numbers: Vec<i32> = Vec::new();

    for arg in env::args().skip(1) {
        match arg.trim().parse() {
            Ok(n) => numbers.push(n),
            Err(_) => eprintln!("'{}' is not a number, ignoring it!", arg),
        }
    }

    println!("The numbers: {:?}", numbers);
}
