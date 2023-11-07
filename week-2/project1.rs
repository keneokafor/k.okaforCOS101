fn main() {
	let p = 520000000;
	let n = 5;
	let r = 10;

	//compound interest
	let a = p * ((1 + (r/100))^n);
	println!("Amount is {}", a );
	let ci = a-p;
	println!("Compound Interest is {} ", ci );
}