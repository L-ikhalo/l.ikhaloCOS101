fn main() {
	let toshiba = 2*450_000;
	let mac = 1*1_500_000;
	let hp = 3*750_000;
	let dell = 3*2_850_000;
	let acer = 1*250_000;

	// total sum
	let sum = toshiba + mac + hp + dell + acer;
	println!("sum={}",sum);

	// average
	let average =sum/10;
	print!("average={}",average);
}