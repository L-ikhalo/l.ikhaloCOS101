// declare structure of a laptop
struct Laptop {
    brand: String,
    price: u32,
}
// method to calculate the price of each laptop
impl Laptop {
    fn cost(&self, quantity: u8) -> u32 {
       //change quantity to u32 so it can multiply with self.price 
        self.price * quantity as u32
    }
}
fn main() {
    //instantiate the structs of each laptop with their brand and prices
    let hp = Laptop{
        brand: String::from("HP"),
        price: 650000,
    };

    let ibm = Laptop{
        brand: String::from("IBM"),
        price: 755000,
    };

    let toshiba = Laptop{
        brand: String::from("Toshiba"),
        price: 550000,
    };

    let dell = Laptop{
        brand: String::from("Dell"),
        price: 850000,
    };
     //calculate the cost of purchasing 3 units of each laptop as well as the total cost
    let hp_cost = hp.cost(3);
    let ibm_cost = ibm.cost(3);
    let toshiba_cost = toshiba.cost(3);
    let dell_cost = dell.cost(3);

    let total = hp_cost + ibm_cost + toshiba_cost + dell_cost;

     // print individual laptop costs, total amount, and thank you message
     println!("Alaba Electronics Laptop purchase");
     println!("{}: N{}", hp.brand, hp_cost);
     println!("{}: N{}", ibm.brand, ibm_cost);
     println!("{}: N{}", toshiba.brand, toshiba_cost);
     println!("{}: N{}", dell.brand, dell_cost);
     println!("Total cost: N{}", total);
     println!("Thank you for shopping with us, hope to see you again next time!");
}