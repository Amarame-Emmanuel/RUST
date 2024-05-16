/* Rust program that takes as input the experience and
age of an employee to determine the annual incentive */

use std::io;

fn main(){
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Are you experienced? (Answer with true or false): ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let _years:bool = input1.trim().parse().expect("Not a valid string");

    println!("Enter age: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let _age:f32 = input2.trim().parse().expect("Not a valid string");

    if _years == true && _age >= 40.0
    {
        println!("Annual incentive is N1,560,000");
    }

    else if _years ==true && _age >= 30.0 && _age < 40.0 
    {
        println! ("Annual incentive is N1,480,000");
    }

    else if _years == true && _age < 28.0
    {
        println!("Incentive is N1,300,000 per month");
    }
    else if _years == false
    {
        println!("Annual incentive is N100,000");
    }


}
