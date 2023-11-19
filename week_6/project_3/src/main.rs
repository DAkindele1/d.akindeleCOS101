use std::io;

fn main() {
    let mut a = String::new();

    println!("Please enter n...");
    io::stdin().read_line(&mut a).expect("Not a valid string");
    let n:u32 = a.trim().parse().expect("Not a valid number");

    for b in 1..=n {
        for c in 1..=12 {
            let table = b * c;
            println!("The result of {} x {} is {}", b, c, table);
        }
        println!("",  );
    }
}
