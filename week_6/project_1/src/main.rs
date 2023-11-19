use std::io;

fn main() {
let mut num:i32 = 0;

  while num < 100 {
    let mut name = String::new();
    let mut email = String::new();
    let mut dp = String::new();
    let mut soo = String::new();

    let mut stat = String::new();
    let mut clas = String::new();
    let mut cgp = String::new();

    println!("Please input your name...");
    io::stdin().read_line(&mut name).expect("Not a valid String");

    println!("Please input your E-mail address...");
    io::stdin().read_line(&mut email).expect("Not a valid String");

    println!("Please input the name of your department...");
    io::stdin().read_line(&mut dp).expect("Not a valid String");

    println!("Please input your State of Origin...");
    io::stdin().read_line(&mut soo).expect("Not a valid String");

    println!("Please input your status in class...");
    io::stdin().read_line(&mut stat).expect("Not a valid String");

    println!("Please input your level...");
    io::stdin().read_line(&mut clas).expect("Not a valid String");
    let class: f32 = clas.trim().parse().expect("That's not a class");

    println!("Please input your CGPA...");
    io::stdin().read_line(&mut cgp).expect("Not a valid String");
    let cgpa: f32 = cgp.trim().parse().expect("Not a CGPA");


        if stat.trim() == "Class Rep" && class > 100.0 && cgpa > 4.0 {
            println!("Hello {}", name);
            println!("Your E-mail Address is {}", email);
            println!("You are a {} student", dp);
            println!("You are a level {} student", class);
            println!("You are from {} state", soo);
            println!("Your CGPA is {}", cgpa);
            println!("You are eligible to vote");
        }
        else
        {
            println!("You are not eligible to vote");
        }
        num += 1;
    }
}
