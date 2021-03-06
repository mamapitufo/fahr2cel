use std::io;
use std::io::Write;

fn main() {
    print!("Enter the temperature in Fahrenheit: ");
    io::stdout()
        .flush()
        .expect("Error flushing stdout buffer!");

    let mut fahr = String::new();
    io::stdin()
        .read_line(&mut fahr)
        .expect("Failed to read temperature!");

    let fahr: f64 = match fahr.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("The temperature must be a number!");
            std::process::exit(1);
        }
    };

    let celsius = fahrenheit_to_celsius(fahr);

    println!("{:.*}°F is: {:.*}°C", 1, fahr, 1, celsius);
}

fn fahrenheit_to_celsius(deg: f64) -> f64 {
    (deg - 32.0) * 5.0 / 9.0
}
