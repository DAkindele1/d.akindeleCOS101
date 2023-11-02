use std::io;

fn main() {
    let mut time = String::new();
    let mut distance = String::new();
    
    println!("Enter time in hours...");
    io::stdin().read_line(&mut time).expect("Not a float value");
    let a:f32 = time.trim().parse().expect("Not a valid number");

    println!("Enter distance in miles...");
    io::stdin().read_line(&mut distance).expect("Not a float value");
    let b:f32 = distance.trim().parse().expect("Not a valid number");

    let c:f32 = b * 1.609;
    let speed:f32 = c/a;

    println!("The speed of the car is {}km/hr", speed );

}
