fn main() {
//Declaring variables...
	let t_amount:f64 = 450_000.00;
	let t_qty:f64 = 2.0;
	let m_amount:f64 = 1_500_000.00;
	let m_qty:f64 = 1.0;
	let h_amount:f64 = 750_000.0;
	let h_qty:f64 = 3.0;
	let d_amount:f64 = 2_850_000.0;
	let d_qty:f64 = 3.0;
	let a_amount:f64 = 250_000.0;
	let a_qty:f64 = 1.0;
//Declaring calculations...
    let sum_amount = t_amount + m_amount + h_amount + d_amount + a_amount;
    let sum_qty = t_qty + m_qty + h_qty + d_qty + a_qty;
    let aver = sum_amount/sum_qty;
//Printing output...
    println!("The total sum of sales by P.M. Okeke and Sons Ltd is N{}", sum_amount);  
   println!("The average of sales by P.M. Okeke and Sons Ltd is N{}", aver);

   }