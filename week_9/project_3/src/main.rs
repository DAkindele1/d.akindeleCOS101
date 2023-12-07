use std::fs::File;
use std::io::Write;

fn main() {

    let comissioner_names = vec!["Aigbogun Alamba Dauda", "Murtala Afeez Bendu", "Okorocha Calistus Ogbona", "Adewale Jimoh Akanbi", "Osazuwa Faith Etieye"];
    let ministries = vec!["Internal Affairs", "Justice", "Defense", "Power & Steel", "Petroleum"];
    let geopolitical = vec!["South West", "North East", "South South", "South West", "South East"];

    let file_name = "project3.txt";

    let mut file = File::create(file_name).expect("Error: Unable to create or open the file.");

    file.write_all(b"           Convicted Ministers\n").expect("Error writing to file");

    file.write_all(format!("{:<20} {:<20} {:<30}\n", "Name of Comissioners", "Ministry", "Geo-Political Zone")
        .as_bytes())
        .expect("Error writing to file");

    for n in 0..comissioner_names.len() {
        file.write_all(
            format!("{:<20} {:<20} {:<30}\n", comissioner_names[n], ministries[n], geopolitical[n])
            .as_bytes()
        )
        .expect("Error writing to file");
    }

    println!("Minister details written to {}", file_name);
}

