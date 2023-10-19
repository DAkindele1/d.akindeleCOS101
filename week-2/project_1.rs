fn main() {
	let p:f32 = 520_000_000.0;
	let r:f32 = 10.0;
	let n:f32 = 5.0;
	//Compound Intrest
	let a = p*(1.0 + r/n);
	println!("Amount is {}",a);
	let ci = a-p;
	println!("Compound Interest is {}",ci);

}