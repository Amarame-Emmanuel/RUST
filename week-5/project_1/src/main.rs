// Rust program to find the roots of a quadratic equation

use std::io;
fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();


    println!("Enter value of A: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a:f32 = input1.trim().parse().expect("Not a valid string");

    println!("Enter value of B: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:f32 = input2.trim().parse().expect("Not a valid string");

    println!("Enter vale of C: ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let c:f32 = input3.trim().parse().expect("Not a valid string");

    let descriminant:f32 = (b*b) - (4.0 * (a * c));

    let d:f32 = (-b + ((b * b) - (4.0 * a * c)).sqrt())/(2.0*a);
    let e:f32 = (-b - ((b * b) - (4.0 * a * c)).sqrt())/(2.0*a);

    if descriminant > 0.0 
    {
        println!("Roots {} and {} are real and distinct",d,e);
    }

    else if descriminant == 0.0 
    {
        println!("There is exactly one root {}",d );
    }

    else 
    {
        println!("Roots {} and {} are imaginary",d,e);
    }

}