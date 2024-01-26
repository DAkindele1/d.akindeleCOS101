use std::io;
use std::fs::File;
use std::io::Write;

fn main() {
    struct Company {
        username: String,
        password: String,
        shares: i32,
        liabilities: i32,
        founding_date: i32,
        percentage_leverage: f64,
    }

    let cadbury = Company {
        username: String::from("cadbry"),
        password: String::from("cadbry000"),
        shares: 15_000_000,
        liabilities: 5_500_000,
        founding_date: 1965,
        percentage_leverage: 5_500_000 as f64 / 15_000_000 as f64,
    };

    let champion = Company {
        username: String::from("champn"),
        password: String::from("champn000"),
        shares: 25_000_000,
        liabilities: 8_000_000,
        founding_date: 1974,
        percentage_leverage: 8_000_000 as f64 / 25_000_000 as f64,
    };

    let dangote = Company {
        username:String::from("dango"),
        password:String::from("dango000"),
        shares: 18_000_000,
        liabilities: 10_000_000,
        founding_date: 1970,
        percentage_leverage: 10_000_000 as f64 / 18_000_000 as f64,
    };
    let flour_mills = Company {
        username:String::from("flour"),
        password:String::from("flour000"),
        shares: 32_000_000,
        liabilities: 4_000_000,
        founding_date: 1960,
        percentage_leverage: 4_000_000 as f64 / 32_000_000 as f64,
    };
     let nestle = Company {
        username:String::from("nest"),
        password:String::from("nest000"),
        shares: 8_000_000,
        liabilities: 1_500_000,
        founding_date: 1961,
        percentage_leverage: 1_500_000 as f64 / 8_000_000 as f64,
    };
     let unilever = Company {
        username:String::from("unilvr"),
        password:String::from("unilvr000"),
        shares: 37_000_000,
        liabilities: 11_000_000,
        founding_date: 1923,
        percentage_leverage: 11_000_000 as f64 / 37_000_000 as f64,
    };
     let honeywell = Company {
        username:String::from("honey"),
        password:String::from("honey000"),
        shares: 34_000_000,
        liabilities: 9_000_000,
        founding_date: 1906,
        percentage_leverage: 9_000_000 as f64 / 34_000_000 as f64,
    };
     let nigerian_breweries = Company {
        username:String::from("ngbrews"),
        password:String::from("ngbrews000"),
        shares: 30_000_000,
        liabilities: 12_000_000,
        founding_date: 1946,
        percentage_leverage: 12_000_000 as f64 / 30_000_000 as f64
    };

    let companies_name = vec![
        "Cadbury Nigeria Plc",
        "Champion Breweries Plc",
        "Dangote Sugar Refinery Plc", 
        "Flour Mills Nigeria Plc", 
        "Nestle Nigeria Plc",
        "Unilever Nigeria Plc", 
        "Honeywell Nigeria Plc", 
        "Nigerian Breweries Plc"
    ];
    let companies: Vec<Company> = vec![cadbury, champion, dangote, flour_mills, nestle, unilever, honeywell, nigerian_breweries];

    let file_name = "Company Information.txt";
    let mut file = File::create(file_name).expect("Error: Unable to create or open the file.");

    file.write_all(b"           Company Information\n").expect("Error writing to file");
    file.write_all(
        format!(
            "{:<20} {:<20} {:<30} {:<30} {:<30}\n",
            "Company",
            "Founding Date",
            "Company Shares",
            "Company Liabilities",
            "Percentage Leverages"
        )
        .as_bytes(),
    )
    .expect("Error writing to file");

    for n in 0..companies_name.len() {
        file.write_all(
            format!(
                "{:<20} {:<20} {:<30} {:<30} {:.2}\n",
                companies_name[n],
                companies[n].founding_date,
                companies[n].shares,
                companies[n].liabilities,
                companies[n].percentage_leverage
            )
            .as_bytes(),
        )
        .expect("Error writing to file");
    }

    println!("Company details written to {}", file_name);

    loop {
        let mut username = String::new();
        println!("Please input your username:
            'cadbry' to access data for Cadbury
            'champn' to access data for Champion
            'dango' to access data for Dangote
            'flour' to access data for Flour Mills
            'nest' to access data for Nestle
            'unilvr' to access data for Unilever
            'honey' to access data for Honeywell
            'ngbrew' to access data for Nigerian Breweries");
        io::stdin().read_line(&mut username).expect("Could not read input");
        let username1: &str = username.trim();

        if username1.chars().count() < 3 || username1.chars().count() > 8 {
            println!("Invalid username");
        } else {
            let mut password = String::new();
            println!("Please input a password:
            'cadbry' to access data for Cadbury
            'champn' to access data for Champion
            'dango' to access data for Dangote
            'flour' to access data for Flour Mills
            'nest' to access data for Nestle
            'unilvr' to access data for Unilever
            'honey' to access data for Honeywell
            'ngbrew' to access data for Nigerian Breweries");
            io::stdin().read_line(&mut password).expect("Could not read input");
            let password1: &str = password.trim();

            if password1.chars().all(|a: char| a.is_ascii_lowercase() || a.is_digit(10)) {
                println!("Incorrect password");
            } else {
                password = password1.to_string();
            }

            let mut authenticated = false;

            for company in companies.iter() {
                if company.username == username1 && company.password == password {
                    authenticated = true;
                    break;
                }
            }

            if authenticated {
                break;
            } else {
                println!("Wrong password or username");
                return;
            }
        }
    }
}