fn main () {
	let t = 450000;
	let m = 1500000;
	let h = 750000;
	let d = 2850000;
	let a = 250000;

	//sum and average
	let sum = t+m+h+d+a;
	println!("The sum of the sales is {}", sum );

	let average = sum/5;
	println!("The average of the sales is {}", average);
}