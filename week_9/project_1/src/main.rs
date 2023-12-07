use std::io::Write;
fn main() {
    let line1 = "\nLager: 33 Export, Desperados, Goldberg, Gulder, Heineken, Star";
    let line2 = "\nStout: Legend, Turbo King, Williams";
    let line3 = "\nMaltina, Amstel Malta, Malta Gold, Fayrouz";

    let mut file = std::fs::File:: create("project_1.txt").expect("create failed");
    file.write_all(line1.as_bytes()).expect("write failed");
    file.write_all(line2.as_bytes()).expect("write failed");
    file.write_all(line3.as_bytes()).expect("write failed");
    println!("\nData written to file");
}
