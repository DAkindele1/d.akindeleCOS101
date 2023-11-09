use std::io;

fn main() {
    let mut total:f64 = 0.0;

    println!("\t\tFood\t\t\tPrice");
    println!("P = Poundo Yam with Edinkaiko Soup      -N3,200");
    println!("F = Fried Rice and Chicken    - N3,000");
    println!("A = Amala and Ewedu Soup     -N2,500");
    println!("E = Eba and Egusi Soup     - N2,000");
    println!("W = White Rice and Stew    - N2,500");

    println!("Enter the letter of your order (input q to quit)...");
    loop {
        let mut order = String::new();
     io::stdin().read_line(&mut order).expect("Not a valid string");

        if order == "P" {
            total += 3200.0;
        } 
        else if order == "F" {
            total += 3000.0;
        }
        else if order == "A" {
            total += 2500.0;
        } 
        else if order == "E" {
            total += 2000.0;
        } 
        else if order == "W" {
            total += 2500.0;
        } 
        else if order == "q" {
            break;
        } 
        else {
            println!("Not Available...");
            continue;
        }
    if total > 10000.0{
        let total1:f64 = total -((5.0 / 100.0) * total);
        println!("Total is over N10,000, Discounted total is N{}",total1 );
    } else {
        println!("Total is N{}",total );
    }  
    }  
}
