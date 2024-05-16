use std::io::Write;

fn main() {

    let lager = "Lager: 33 Export, Desperados, Goldberg, Gulder, Heineken, Star\n";
    let stout = "Stout: Legend, Turbo King, Williams\n";
    let non_alcoholic = "Non-Alcoholic: Maltina, Amstel Malta, Malta Gold, Fayrouz\n";

    let mut file = std::fs::File::create("project.txt").expect("create field");
    file.write_all(lager.as_bytes()).expect("write failed");
    file.write_all(stout.as_bytes()).expect("write failed");
    file.write_all(non_alcoholic.as_bytes()).expect("write failed");
    println!("\nData written to file");
}