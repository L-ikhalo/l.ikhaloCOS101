
use std::io;

fn main() {
    loop {
        let mut candidates: Vec<(String, u32)> = Vec::new();
        let mut number = String::new();

        println!("\nEY INTERVIEW");
        println!("Enter the number of candidates:");

        io::stdin().read_line(&mut number).expect("Failed to read input");

        let number: usize = match number.trim().parse() {
            Ok(value) => value,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        if number == 0 {
            println!("Please enter at least one candidate.");
            continue;
        }

        for i in 0..number {
            let mut name = String::new();
            let mut experience = String::new();

            println!("Enter the name of candidate {}:", i + 1);
            io::stdin().read_line(&mut name).expect("Failed to read input");

            println!("Enter years of programming experience:");
            io::stdin().read_line(&mut experience).expect("Failed to read input");

            let name = name.trim().to_string();

            let experience: u32 = match experience.trim().parse() {
                Ok(value) => value,
                Err(_) => {
                    println!("Invalid experience. Please restart the program.");
                    continue;
                }
            };

            candidates.push((name, experience));
        }

        if candidates.len() > 0 {
            let mut highest = 0;

            for i in 0..candidates.len() {
                if candidates[i].1 > candidates[highest].1 {
                    highest = i;
                }
            }

            println!("\nCandidate with the highest experience:");
            println!("Name: {}", candidates[highest].0);
            println!("Years of experience: {}", candidates[highest].1);
        }

        let mut answer = String::new();

        println!("\nDo you want to interview another group? (y/n)");
        io::stdin().read_line(&mut answer).expect("Failed to read input");

        if answer.trim() != "y" {
            println!("Program ended.");
            break;
        }
    }
}
