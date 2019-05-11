use std::collections::HashMap;
use std::io;

fn main() {
    println!("Welcome to our People Inventory System!\n");
    let mut departments: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!("Please input your command (or type 'help')");
        let cmd = read_command();

        if cmd == "quit" {
            std::process::exit(0);
        } else if cmd.starts_with("Add ") {
            match parse_add(&cmd[4..]) {
                Some((employee_name, department_name)) => {
                    let department = departments.entry(department_name).or_insert(Vec::new());
                    department.push(employee_name);
                }
                None => help(),
            }
        } else if cmd == "List" {
            list_everyone(&departments);
        } else if cmd.starts_with("List ") {
            list_department(&departments, &cmd[5..]);
        } else {
            help();
        }
    }
}

fn list_everyone(departments: &HashMap<String, Vec<String>>) {
    for department in departments.keys() {
        list_department(&departments, department);
    }
}

fn list_department(departments: &HashMap<String, Vec<String>>, department_name: &str) {
    match departments.get(department_name) {
        Some(department) => {
            println!("  Employees in {}:", department_name);
            for employee in department.iter() {
                println!("  - {}", employee);
            }
        },
        None => println!("Err: The {} department does not exist.", department_name),
    }
}

fn read_command() -> String {
    let mut cmd = String::new();
    io::stdin()
        .read_line(&mut cmd)
        .expect("Failed to read input (x_x)");

    cmd.trim().to_string()
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
