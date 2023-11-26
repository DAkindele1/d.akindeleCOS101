use std::io;

fn aot(h:i32, b1:i32, b2:i32) {
    let trap = h/2 * (b1 + b2);
    println!("The area of the trapezium is {}",trap);
    
}
fn aor(d1:i32, d2:i32) {
    let rhom = 1/2 * (d1 * d2);
    println!("The area of the rhombus is {}",rhom);
}
fn aop(b:i32, a:i32) {
    let parr = b * a;
    println!("The area of the parallelogram is {}",parr);
}
fn aoc(l:i32) {
    let cub = 6 * (l^2);
    println!("The area of the cube is {}",cub);
}
fn voc(r:i32, h2:i32) {
    let pi = 22/7;
    let cyl = pi * r^2 * h2;
    println!("The volume of the cylinder is {}",cyl);
}
fn main() {
    println!("Key:
        For area of trapezium, input 1
        For area of rhombus, input 2
        For area of parallelogram, input 3
        For area of cube, input 4
        For volume of cylinder, input 5");

 let mut input = String::new();
 println!("\n\nPlease input the equation you want to solve...");
 io::stdin().read_line(&mut input).expect("Failed to read input");
 let input1:i32 = input.trim().parse().expect("Not a valid input");

  if input1 == 1 {
     let mut input2 = String::new();
     println!("Enter input for height of the trapezium..." );
     io::stdin().read_line(&mut input2).expect("Failed to read input");
     let h:i32 = input2.trim().parse().expect("Invalid input");
 
     let mut input3 = String::new();
     println!("Enter input for the first base of the trapezium..." );
     io::stdin().read_line(&mut input3).expect("Failed to read input");
     let b1:i32 = input3.trim().parse().expect("Invalid input");

     let mut input4 = String::new();
     println!("Enter input for the second base of the trapezium..." );
     io::stdin().read_line(&mut input4).expect("Failed to read input");
     let b2:i32 = input4.trim().parse().expect("Invalid input");

     aot(h, b1, b2)
    }
    else if input1 == 2 {
     let mut input5 = String::new();
     println!("Enter input for the first diagonal of the rhombus..." );
     io::stdin().read_line(&mut input5).expect("Failed to read input");
     let d1:i32 = input5.trim().parse().expect("Invalid input");

     let mut input6 = String::new();
     println!("Enter input for the second diagonal of the rhombus..." );
     io::stdin().read_line(&mut input6).expect("Failed to read input");
     let d2:i32 = input6.trim().parse().expect("Invalid input");

     aor(d1, d2)
    }
    else if input1 == 3 {
     let mut input7 = String::new();
     println!("Enter input for the base of the parallelogram..." );
     io::stdin().read_line(&mut input7).expect("Failed to read input");
     let b:i32 = input7.trim().parse().expect("Invalid input");

     let mut input8 = String::new();
     println!("Enter input for the altitude of the parallelogram..." );
     io::stdin().read_line(&mut input8).expect("Failed to read input");
     let a:i32 = input8.trim().parse().expect("Invalid input");

     aop(b, a)
    }
    else if input1 == 4 {
     let mut input9 = String::new();
     println!("Enter input for the length of the cube..." );
     io::stdin().read_line(&mut input9).expect("Failed to read input");
     let l:i32 = input9.trim().parse().expect("Invalid input");

     aoc(l)
    }
    else if input1 == 5 {

     let mut input10 = String::new();
     println!("Enter input for the radius of the cylinder..." );
     io::stdin().read_line(&mut input10).expect("Failed to read input");
     let r:i32 = input10.trim().parse().expect("Invalid input");

     let mut input11 = String::new();
     println!("Enter input for the height of the cylinder..." );
     io::stdin().read_line(&mut input11).expect("Failed to read input");
     let h2:i32 = input11.trim().parse().expect("Invalid input");

     voc(r, h2, )
    }
    else   {
        println!("Sorry, this equation is not supported by the program");
    }
}

