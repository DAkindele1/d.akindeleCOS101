use std::io::Read;
use std::io;

fn admin() {
    let mut file = std::fs::File::open("globacom_dbase.sql").unwrap();
    let mut globacom = String::new();
    file.read_to_string(&mut globacom).unwrap();
    print!("{}",globacom);
}

fn project() {
    let mut file1 = std::fs::File::open("project_tb.sql").unwrap();
    let mut project = String::new();
    file1.read_to_string(&mut project).unwrap();
    print!("{}",project);
}

fn employee() {
    let mut file2 = std::fs::File::open("staff_tb.sql").unwrap();
    let mut staff = String::new();
    file2.read_to_string(&mut staff).unwrap();
    print!("{}",staff);
}

fn customer() {
    let mut file3 = std::fs::File::open("customer_tb.sql").unwrap();
    let mut customer = String::new();
    file3.read_to_string(&mut customer).unwrap();
    print!("{}",customer);
}

fn vendor() {
    let mut file4 = std::fs::File::open("dataplan_tb.sql").unwrap();
    let mut vendor = String::new();
    file4.read_to_string(&mut vendor).unwrap();
    print!("{}",vendor);
}

fn main() {
    let mut person = String::new();
    println!("Please enter your designation (in number):
        1. Administrator
        2. Project Manager
        3. Employee
        4.Customer
        5. Vendor");
    io::stdin().read_line(&mut person).expect("Could not read input");
    let person1:i32 = person.trim().parse().expect("Not a valid input");

    if person1 == 1 {
       admin();
    }
    if person1 == 2 {
        project();
    }
    if person1 == 3 {
        employee();
    }
    if person1 == 4 {
        customer();
    }
    if person1 == 5 {
        vendor();
    }
    else {
        println!("Not a valid key");
    }
}