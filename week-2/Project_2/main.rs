fn main() {
	let t:f64 = 450_000.00; //t stands for Toshiba
	let nt:f64 = 2.0; //nt stands for quantity of Toshiba
	let m:f64 = 1_500_000.00; //m stands for Mac
	let nm:f64 = 1.0; //nm stands for quantity of Mac
	let h:f64 = 750_000.00; //h stands for of Hp
	let nh:f64 = 3.0; //nh stands for quantity of Hp
	let d:f64 = 2_850_000.00; //d stands for Dell
	let nd:f64 = 3.0; //nd stands for quantity of Dell
	let a:f64 = 250_000.00; //a stands for Acer
	let na:f64 = 1.0; //na stands for quantity of Acer

	//sum 
	let s = (t * nt) + (m * nm) + (h * nh) + (d * nd) + (a * na);
	println!("Sum is {}", s);

	//total quantity
	let n = nt + nm + nh + nd + na;

	//average
	let a = s / n;
	println!("Average is {}", a);
	 
}