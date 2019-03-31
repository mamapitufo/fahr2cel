use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let n = &args[1];

    let n: i64 = match n.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("The argument must be a number!");
            std::process::exit(1);
        }
    };

    if n > 40 {
        eprintln!("This will take too long!");
        std::process::exit(2);
    };

    println!("Fibonacci({}) = {}", n, fibo(n));
}

fn fibo(n: i64) -> i64 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fibo(n - 1) + fibo(n - 2)
    }
}
