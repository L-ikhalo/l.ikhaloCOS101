fn main() {
	let p = 210_000.0f64;
	let r = 5.0f64;
	let n = 3.0f64;


	//amount
	let a = p*(1.0-(r/100.0)).powf(n);

    print!("amount={}", a.ceil());
}
