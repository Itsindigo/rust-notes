use std::collections::HashMap;
use std::io;

type CompanyData<'a> = HashMap<&'static str, Vec<String>>;

fn main() {
    // Open application

    // prompt user with three options.
    // 1. All people by department
    // 2. All people in company
    // 3. Add new user to department
    // 4. Exit program

    let mut company_data = seed_company_data();

    show_menu(&mut company_data);
}

fn show_menu(company_data: &mut CompanyData) -> () {
    println!("\nWhat would you like to do?\nTo make your choice enter 1, 2, 3 or 4.\n\t1. List employees by department\n\t2. List all employees \n\t3. Add new employee to department\n\t4. Exit program\n");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    match input.as_str().trim() {
        "1" => employees_by_department(&company_data),
        "2" => list_all_employees(&company_data),
        "3" => join_department(company_data),
        "4" => exit(),
        input => {
            println!("Invalid input: \"{}\".\n", input);
        }
    };

    show_menu(company_data);
}

fn seed_company_data() -> CompanyData<'static> {
    let mut map = HashMap::new();

    map.entry("Finance")
        .or_insert_with(Vec::new)
        .push("Tom Smith".to_string());

    map.entry("Finance")
        .or_insert_with(Vec::new)
        .push("Angela Delaney".to_string());

    map.entry("Technology")
        .or_insert_with(Vec::new)
        .push("Bob Jenkins".to_string());

    map.entry("Human Resources")
        .or_insert_with(Vec::new)
        .push("Sally Cinammon".to_string());

    map
}

fn employees_by_department(company_data: &CompanyData) {
    let mut departments: Vec<&str> = vec![];
    println!("Choose a department:");

    // how can i remove this clone()?
    for (i, (key, _)) in company_data.clone().into_iter().enumerate() {
        departments.push(key);
        println!("\t{}. {:?}", i + 1, key);
    }

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input_index = input.trim().parse::<i32>();

    match input_index {
        Ok(input_index) => {
            if input_index <= 0 || input_index > departments.len() as i32 {
                println!("Your choice was invalid.");
                return employees_by_department(&company_data);
            }
            let department = departments.get((input_index - 1) as usize).unwrap();
            let department_employees = company_data.get(department).unwrap();
            println!(
                "The employees in {} are {:?}",
                department, department_employees
            );
        }
        Err(_err) => {
            eprintln!("Your choice was invalid. You must make a numeric choice.");
            return employees_by_department(&company_data);
        }
    }
}

fn list_all_employees(company_data: &CompanyData) {
    let mut all_employees: Vec<&String> = vec![];

    for (_, value) in company_data.into_iter() {
        all_employees.extend(&*value);
    }

    all_employees.sort_unstable();

    println!("All employees: {:?}", all_employees);
}

fn join_department(company_data: &mut CompanyData) {
    let mut departments: Vec<&str> = vec![];
    println!("Which department are we adding to?");

    // how can i remove this clone()?
    for (i, (key, _)) in company_data.clone().into_iter().enumerate() {
        departments.push(key);
        println!("\t{}. {:?}", i + 1, key);
    }

    let mut department_choice = String::new();

    io::stdin()
        .read_line(&mut department_choice)
        .expect("Failed to read line");

    let input_index = department_choice.trim().parse::<i32>();

    let mut department = None;

    match input_index {
        Ok(input_index) => {
            if input_index <= 0 || input_index > departments.len() as i32 {
                println!("Your choice was invalid.");
                return employees_by_department(&company_data);
            }
            department = Some(departments.get((input_index - 1) as usize).unwrap());
        }
        Err(_err) => {
            eprintln!("Your choice was invalid. You must make a numeric choice.");
            return employees_by_department(&company_data);
        }
    }

    
    match department {
        Some(d) => {
            let mut user_name = String::new();
            println!("What is the user's full name?");

            io::stdin()
                .read_line(&mut user_name)
                .expect("Failed to read line");

            company_data
                .entry(d)
                .or_insert_with(Vec::new)
                .push(user_name.trim().to_string());
        }
        None => println!("Didn't have department"),
    }
}

fn exit() {
    let exit_code = 0;

    println!("Exiting");
    std::process::exit(exit_code);
}
