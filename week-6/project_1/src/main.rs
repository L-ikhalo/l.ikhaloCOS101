use std::io;

fn main() {
    // Display the menu
    println!("===== RESTAURANT MENU =====");
    println!("P - Poundo Yam / Edikaikong Soup - N3200");
    println!("F - Fried Rice & Chicken       - N3000");
    println!("A - Amala & Ewedu Soup         - N2500");
    println!("E - Eba & Egusi Soup           - N2000");
    println!("W - White Rice & Stew           - N2500");

    // Ask for food type
    println!("\nEnter food type (P, F, A, E, W):");

    let mut food_type = String::new();
    io::stdin()
        .read_line(&mut food_type)
        .expect("Failed to read input");

    let food_type = food_type.trim().to_uppercase();

    // Determine the price
    let price = match food_type.as_str() {
        "P" => 3200,
        "F" => 3000,
        "A" => 2500,
        "E" => 2000,
        "W" => 2500,
        _ => {
            println!("Invalid food type!");
            return;
        }
    };

    // Ask for quantity
    println!("Enter quantity:");

    let mut quantity = String::new();
    io::stdin()
        .read_line(&mut quantity)
        .expect("Failed to read input");

    let quantity: u32 = quantity.trim().parse().expect("Please enter a number");

    // Calculate total
    let total = price * quantity;

    println!("\nTotal before discount: N{}", total);

    // Apply 5% discount if total is greater than N10,000
    if total > 10_000 {
        let discount = total * 5 / 100;
        let final_total = total - discount;

        println!("Discount (5%): N{}", discount);
        println!("Total charge: N{}", final_total);
    } else {
        println!("Total charge: N{}", total);
    }
}