use std::io;

fn main() {
    println!("Welcome to our People Inventory System!\n");

    loop {
        println!("Please input your command (or type 'help')");
        let mut cmd = String::new();
        io::stdin()
            .read_line(&mut cmd)
            .expect("Failed to read input (x_x)");

        let cmd = cmd.trim();

        if cmd == "quit" {
            std::process::exit(0);
        } else {
            help();
        }
    }
}

fn help() {
    println!("  Available Commands:\n  ===================");
    println!("  Add Sally to Engineering: Adds 'Sally' to the 'Engineering' department.");
    println!("  List Engineering:         Lists all employees in Engineering.");
    println!("  List:                     List all employees, grouped by department.");
}
