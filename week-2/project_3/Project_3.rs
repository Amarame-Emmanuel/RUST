fn main() {
	let p:f64 = 210_000.00;
	let r:f64 = 5.0;
	let n:f64 = 3.0;

	let b = 1.0 - (r / 100.0);
	let b = f64 ::powf(b,n);

	//Depreciation
	let ci = p * b;
	println!("Depreciation is {}", ci);

	}