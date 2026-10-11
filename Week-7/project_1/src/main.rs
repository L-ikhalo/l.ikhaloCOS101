
use std::io;

fn get_number() -> f64 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().parse().expect("Please enter a valid number")
}

fn area_rectangle() {
    println!("Enter the length:");
    let length = get_number();

    println!("Enter the width:");
    let width = get_number();

    let area = length * width;
    println!("Area of rectangle = {}", area);
}

fn area_circle() {
    println!("Enter the radius:");
    let radius = get_number();

    let area = 3.142 * radius * radius;
    println!("Area of circle = {}", area);
}

fn volume_cube() {
    println!("Enter the side of the cube:");
    let side = get_number();

    let volume = side * side * side;
    println!("Volume of cube = {}", volume);
}

fn volume_cylinder() {
    println!("Enter the radius:");
    let radius = get_number();

    println!("Enter the height:");
    let height = get_number();

    let volume = 3.142 * radius * radius * height;
    println!("Volume of cylinder = {}", volume);
}

fn main() {
    loop {
        println!("\n--- AREA AND VOLUME CALCULATOR ---");
        println!("1. Area of Rectangle");
        println!("2. Area of Circle");
        println!("3. Volume of Cube");
        println!("4. Volume of Cylinder");
        println!("5. Exit");

        println!("Enter your choice:");
        let choice = get_number();

        match choice as i32 {
            1 => area_rectangle(),
            2 => area_circle(),
            3 => volume_cube(),
            4 => volume_cylinder(),
            5 => {
                println!("Thank you for using the calculator!");
                break;
            }
            _ => println!("Invalid choice. Please try again."),
        }
    }
}
