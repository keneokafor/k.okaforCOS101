fn main () {
	let p = 210000;
	let r = 5;
	let n = 3;

	//depreciation
	let dep = p* (1 -(r/100))^n;
	println!("Depreciation is {}", dep);
}