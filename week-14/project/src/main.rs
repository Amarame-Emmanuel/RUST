use std::io::Read;
use std::io;
fn main() {
    let mut input= String::new();
    println!("What is your role he management?\n Press 1 if Administrator\n Press 2 if Project Managaer\n Press 3 if Employee\n Press 4 if Customer\n Press 5 if Vendor");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let _input:i32 = input.trim().parse().expect("Failed to read input");

    if _input == 1
    {
        let mut file = std::fs::File::open("globacom_dbase.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
    }
    else if _input == 2
    {
        let mut file = std::fs::File::open("project_tb.sql").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        println!("{}", contents);
    }
    else if _input == 3
    {
        let mut file = std::fs::File::open("staff_tb.sql").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        println!("{}", contents);
    }
    else if _input == 4
    {
        let mut file = std::fs::File::open("customer_tb.sql").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        println!("{}", contents);
    }
    else if _input == 5
    {
        let mut file = std::fs::File::open("dataplan_tb.sql").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        println!("{}", contents);
    }

}
