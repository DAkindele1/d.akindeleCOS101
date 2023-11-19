use std::io;

fn main() {
    let mut num:i32 = 0;
    
    while num < 500 {
        let mut name = String::new();
        let mut numop = String::new();

        println!("\n\nNRG Researchers Publication Incentive System");

        println!("Please enter your name...");
        io::stdin().read_line(&mut name).expect("This is not a string");

        println!("Plese enter the number of papers you have published...");
        io::stdin().read_line(&mut numop).expect("This is not a string");
        let nump:i32 = numop.trim().parse().expect("This is not a number ");

        if nump >= 3 && nump <= 5
        {
            println!("Hello {}",name );
            print!("Your incentive from the RPIS is N{}",500_000 );
        }
        else if nump >= 5 && nump <= 10
        {
            println!("Hello {}",name );
            println!("Your incentive from the RPIS is N{}",800_000 );
        }
        else if nump > 10 
        {
            println!("Hello {}",name );
            println!("Your incentive from the RPIS is N{}",1_000_000 );
        }
        else if nump < 3
        {
            println!("Hello {}",name );
            println!("Your incentive from the RPIS is N{}",100_000 );
            println!("...");

        }
        num+=1;

    }
    
}
