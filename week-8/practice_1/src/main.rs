fn main() {
    
    // Using Vec::new()
    let v : Vec<i64> = Vec::new();
    
    // printing the size of vector
    println!("\nThe length of Vec::new is: {}",v.len());

    // Using Macro
    let v = vec!["Lisa","Ikanke","Afam","Dozie","Lesley"];

    // printing the size of vector
    println!("\nThe length of vec macro is: { }",v.len());

}
