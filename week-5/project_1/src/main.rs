use std::io;

fn main() {
    println!("\nEnter the value for a...");
    let mut input1 = String::new();
     io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a:f32 = input1.trim().parse().expect("Not a valid number");

    println!("\nEnter the value for b...");
     let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:f32 = input2.trim().parse().expect("Not a valid number");

    println!("\nEnter the value for c...");
     let mut input3 = String::new();
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let c:f32 = input3.trim().parse().expect("Not a valid number");

    let d = b.powf(2.0) - 4.0*a*c ;

    if d > 0.0 {
         println!("There are two distinct roots for the quadratic equation");
     } 
     else if d == 0.0 {
        println!("There is one real root for the quadratic equation");
     }
     else {
         println!("There are no real roots for the quadratic equation");
     }
}
