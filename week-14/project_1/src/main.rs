use std::io::Read;
use std::io;

fn main() {
    let mut _input= String::new();
    println!("What is your role in the management?\nAdministrator\nProject Manager\nEmployee\nCustomer\nVendor");
    io::stdin().read_line(&mut _input).expect("Failed to read input");


    if _input == "Administrator"
    {
        let mut file = std::fs::File::open("globacom_dbase.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
    }
    else if _input == "Project Manager"
    {
        let mut file = std::fs::File::open("project_tb.sql").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        println!("{}", contents);
    }
    else if _input == "Employee"
    {
        let mut file = std::fs::File::open("staff_tb.sql").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        println!("{}", contents);
    }
    else if _input == "Customer"
    {
        let mut file = std::fs::File::open("customer_tb.sql").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        println!("{}", contents);
    }
    else if _input == "Vendor"
    {
        let mut file = std::fs::File::open("dataplan_tb.sql").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        println!("{}", contents);
    }

}
