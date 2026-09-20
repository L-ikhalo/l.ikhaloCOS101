use std::io::{self, Write};

fn main() {
    println!("--- Employee Incentive Calculator ---");

    
    let is_experienced = read_boolean("Is the employee experienced? (yes/no): ");

    let incentive = if is_experienced {
 
        let age = read_u32("Enter employee age: ");

        if age >= 40 {
            1_560_000 // Experienced, age 40 or more
        } else if age >= 30 && age <= 39 {
            1_480_000 // Experienced, age 30 to 39
        } else if age < 28 {
            1_300_000 // Experienced, below 28
        } else {
           
            1_300_000 
        }
    } else {
        100_000 
    };

    println!("\n------------------------------------");
    println!("Annual Incentive: N{}", incentive);
    println!("------------------------------------");
}

// Helper function to handle reading boolean input
fn read_boolean(prompt: &str) -> bool {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim().to_lowercase().as_str() {
            "yes" | "y" | "true" | "1" => return true,
            "no" | "n" | "false" | "0" => return false,
            _ => println!("Invalid input. Please enter 'yes' or 'no'."),
        }
    }
}


fn read_u32(prompt: &str) -> u32 {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim().parse::<u32>() {
            Ok(num) => return num,
            Err(_) => println!("Invalid input. Please enter a valid age (integer)."),
        }
    }
}