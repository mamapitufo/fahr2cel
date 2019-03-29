fn main() {
    let n = 3;

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
