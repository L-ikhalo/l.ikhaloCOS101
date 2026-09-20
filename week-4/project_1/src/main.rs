use std::io;

fn main() {
    println!("--- Quadratic Equation Solver ---");

    // Read user input for a, b, and c
    let a = read_input("Enter value for a: ");
    let b = read_input("Enter value for b: ");
    let c = read_input("Enter value for c: ");

    
    if a == 0.0 {
        println!("Error: 'a' cannot be zero in a quadratic equation.");
        return;
    }

    let d = b * b - 4.0 * a * c;

    println!("\nDiscriminant (d) = {}", d);

    // Determine roots based on discriminant value
    if d > 0.0 {
        // Two distinct real roots
        let root1 = (-b + d.sqrt()) / (2.0 * a);
        let root2 = (-b - d.sqrt()) / (2.0 * a);
        println!("The equation has two distinct real roots:");
        println!("Root 1 = {:.2}", root1);
        println!("Root 2 = {:.2}", root2);
    } else if d == 0.0 {
        // Exactly one real root
        let root = -b / (2.0 * a);
        println!("The equation has exactly one real root:");
        println!("Root = {:.2}", root);
    } else {
        // No real roots
        println!("d is less than zero: No real roots exist.");
    }
}


fn read_input(prompt: &str) -> f64 {
    loop {
        use std::io::Write;
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse::<f64>() {
            Ok(num) => return num,
            Err(_) => println!("Invalid input. Please enter a valid number."),
        }
    }
}