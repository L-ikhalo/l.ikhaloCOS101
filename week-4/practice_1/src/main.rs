// Rust program to output name and age

use std::io;

fn main() {
    println!("\n Student Information Management System!");

    // input name
    println!("\nPlease Enter Your Name.");
    let mut name = String::new();
        io::stdin()
        .read_line(&mut name)
        .expect("Failed to read input");
    println!("Your Name is: {}", name);

    // input age
    println!("\nEnter Your Age.");
    let mut age = String::new();
        io::stdin().read_line(&mut age).expect("Failed to read input");
    let age:u8 = age.trim().parse().expect("Input not an integer");
    println!("Your age is: {}", age);
}
