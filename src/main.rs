use std::io;

fn main() {
    println!("Enter the temperature in Fahrenheit:");

    let mut fahr = String::new();
    io::stdin()
        .read_line(&mut fahr)
        .expect("Failed to read temperature! (x_x)");

    let fahr: i32 = match fahr.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("You must enter a number!");
            std::process::exit(1);
        }
    };

    let celsius = fahrenheit_to_celsius(fahr);

    println!("{} degrees Fahrenheit is {} degrees Celsius", fahr, celsius);
}

fn fahrenheit_to_celsius(deg: i32) -> i32 {
    (deg - 32) * 5 / 9
}
