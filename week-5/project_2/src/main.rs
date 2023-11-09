use std::io;

fn main() {
    println!("\nEnter number of years worked...");

    println!("\nEnter your age...");
    let mut input2 = String::new();
     io::stdin().read_line(& mut input2).expect("Not a valid string");
    let b:i32 = input2.trim().parse().expect("Not a valid number");

    if a >= 5 && b >= 40 {
        println!("The incentive of the employee is N1560000");
    }
    else if a >= 5 && b >= 30 && b <= 40 {
        println!("The incentive of the employee is N1480000");
    }
    else if a >= 5 && b < 28 {
        println!("The incentive of the employee is N1300000");
    }
    else {
        println!("The incentive of the employee is N100000");
    }

}
