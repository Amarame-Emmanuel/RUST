fn main() {
	let p:f64 = 520_000_000.00;
	let r:f64 = 10.0;
	let n:f64 = 5.0;

	let b = 1.0 +(r / 100.0);
	let b = f64 ::powf(b,n);

	//compound interest
	let a = p * b;
	println!("Amount is {}", a);
	let ci = a - p;
	println!("Compound interest is {}", ci);
	
}