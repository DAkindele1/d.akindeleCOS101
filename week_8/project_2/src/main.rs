use std::io;

fn dets(name: &mut Vec<String>, degree: &mut Vec<String>, exp: &mut Vec<usize>){
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("PLease input your name...");
    io::stdin().read_line(&mut input1).expect("invalid input");
    let name1 = input1.trim();
    name.push(name1.to_string());

    println!("What degree are you pursuing?");
    io::stdin().read_line(&mut input2).expect("invalid");
    let b_sc = input2.trim();
    degree.push(b_sc.to_string());

    println!("For how many years have you been programming?");
    io::stdin().read_line(&mut input3).expect("Invalid input");
    let years:usize = input3.trim().parse().expect("Invalid input");
    exp.push(years);
}
fn main(){
    let mut name: Vec<String> = Vec::new();
    let mut degree: Vec<String> = Vec::new();
    let mut exp: Vec<usize> = Vec::new();
    let mut input4 =String::new();
    println!("How many people are being interviewed?");
    io::stdin().read_line(&mut input4).expect("invalid");
    let interviewed:usize = input4.trim().parse().expect("invalid");

    for j in 0..interviewed{
        dets(&mut name, &mut degree, &mut exp);
    }

    let most_experience = checker(interviewed, &exp);
    println!("\nThe person with the most experience is {} 
    They are studying {}
    They have {} years of experience",name[most_experience], degree[most_experience],
        exp[most_experience]);
}
fn checker(input4:usize, years: &Vec<usize>) -> usize {
    let mut experience = 0;
    let mut max_experience = 0;
    for j in 0..input4{
        if years[j] > experience{
            experience = years[j];
            max_experience = j;
        }
    }
    return max_experience;
}