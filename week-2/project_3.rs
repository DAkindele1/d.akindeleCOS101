fn main() {
	let p:f32 = 210_000.00;
	let r:f32 = 5.0;
	let n:f32 = 3.0;
	//Now for depreciation...
	let a = p*(1.0 - r/100.0).powf(n);
	println!("The value of the TV after three years is N{}",a);

} 