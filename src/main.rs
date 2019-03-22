fn main() {
    let fahr = 32;
    let celsius = fahrenheit_to_celsius(fahr);

    println!("{} degrees Fahrenheit is {} degrees Celsius", fahr, celsius);
}

fn fahrenheit_to_celsius(deg: i32) -> i32 {
    (deg - 32) * 5 / 9
}
