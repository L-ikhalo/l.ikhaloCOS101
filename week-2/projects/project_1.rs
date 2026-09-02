fn main() {
    // Values from the problem
    let p:f64 = 520000000.0;
    let r:f64 = 10.0;
    let n:f64 = 5.0;
    // Total final amount
    let a = p * (1.0 + r / 100.0).powf(n);
    // Compound interest
    let ci = a - p;
    // Print the results
    println!("Amount = {}", a.ceil());
    println!("Compound Interest = {}", ci.ceil());
}