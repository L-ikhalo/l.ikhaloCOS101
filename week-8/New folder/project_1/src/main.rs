
use std::io;

fn main() {
    loop {
        let levels = vec!["APS 1-2", "APS 3-5", "APS 5-8", "EL1 8-10", "EL2 10-13", "SES"];

        let office_admin = vec![
            "Intern", "Administrator", "Senior Administrator",
            "Office Manager", "Director", "CEO",
        ];

        let academic = vec![
            "-", "Research Assistant", "PhD Candidate",
            "Post-Doc Researcher", "Senior Lecturer", "Dean",
        ];

        let lawyer = vec![
            "Paralegal", "Junior Associate", "Associate",
            "Senior Associate 1-2", "Senior Associate 3-4", "Partner",
        ];

        let teacher = vec![
            "Placement", "Classroom Teacher", "Senior Teacher",
            "Leading Teacher", "Deputy Principal", "Principal",
        ];

        let mut category = String::new();
        let mut job = String::new();

        println!("\nAPS LEVEL CHECKER");
        println!("1. Office Admin");
        println!("2. Academic");
        println!("3. Lawyer");
        println!("4. Teacher");
        println!("Choose your category:");

        io::stdin().read_line(&mut category).expect("Failed to read input");

        let category: u32 = match category.trim().parse() {
            Ok(value) => value,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        println!("Enter your job title:");
        io::stdin().read_line(&mut job).expect("Failed to read input");
        let job = job.trim();

        let mut found = false;

        if category == 1 {
            for i in 0..office_admin.len() {
                if office_admin[i] == job {
                    println!("Position: {}", office_admin[i]);
                    println!("Level: {}", levels[i]);
                    found = true;
                }
            }
        } else if category == 2 {
            for i in 0..academic.len() {
                if academic[i] == job {
                    println!("Position: {}", academic[i]);
                    println!("Level: {}", levels[i]);
                    found = true;
                }
            }
        } else if category == 3 {
            for i in 0..lawyer.len() {
                if lawyer[i] == job {
                    println!("Position: {}", lawyer[i]);
                    println!("Level: {}", levels[i]);
                    found = true;
                }
            }
        } else if category == 4 {
            for i in 0..teacher.len() {
                if teacher[i] == job {
                    println!("Position: {}", teacher[i]);
                    println!("Level: {}", levels[i]);
                    found = true;
                }
            }
        } else {
            println!("Invalid category.");
            found = true;
        }

        if found == false {
            println!("Job title not found. Check your spelling.");
        }

        let mut answer = String::new();

        println!("\nDo you want to check another staff member? (y/n)");
        io::stdin().read_line(&mut answer).expect("Failed to read input");

        if answer.trim() != "y" {
            println!("Program ended.");
            break;
        }
    }
}
