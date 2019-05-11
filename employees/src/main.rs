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
        } else if cmd.starts_with("Add ") {
            match parse_add(&cmd[4..]) {
                Some((employee, department)) => {
                    println!("add {} to {}", employee, department);
                }
                None => help(),
            }
        } else {
            help();
        }
    }
}

fn parse_add(cmd: &str) -> Option<(String, String)> {
    let parts: Vec<&str> = cmd.split(" to ").collect();
    if parts.len() == 2 {
        Some((String::from(parts[0]), String::from(parts[1])))
    } else {
        None
    }
}

fn help() {
    println!("  Available Commands:\n  ===================");
    println!("  Add Sally to Engineering: Adds 'Sally' to the 'Engineering' department.");
    println!("  List Engineering:         Lists all employees in Engineering.");
    println!("  List:                     List all employees, grouped by department.");
}
