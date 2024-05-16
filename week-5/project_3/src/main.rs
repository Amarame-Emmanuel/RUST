/* Rust program that display the menu 
for the food items available to take 
order from the customers*/


use std::io;

fn main() {

    let _p:f32 = 3200.0;
    let _f:f32 = 3000.0;
    let _a:f32 = 2500.0;
    let _e:f32 = 2000.0;
    let _w:f32 = 2500.0;

    let mut input5 = String::new();
    println!("P = Poundo Yam/Edinkaiko Soup -N3,200\nF = Fried Rice and Chicken -N3,000\nA = Amala & Ewedu Soup -N2,500\nE = Eba and Egusi Soup - N2,000\nW = White Rice & Stew -N2,500\n[Press Enter to make order]");
    io::stdin().read_line(&mut input5).expect("Not a valid string");


    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    let mut input4 = String::new();
    let mut input5 = String::new();

    println!("How many Poundo yam are you ordering? :");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a:f32 = input1.trim().parse().expect("Not a valid string");

    println!("How many Fried rice and chicken are you ordering? :");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:f32 = input2.trim().parse().expect("Not a valid string");

    println!("How many Amala and Ewedu soup are you ordering? :");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let c:f32 = input3.trim().parse().expect("Not a valid string");

    println!("How many Eba and Egusi soup are you ordering? :");
    io::stdin().read_line(&mut input4).expect("Not a valid string");
    let d:f32 = input4.trim().parse().expect("Not a valid string");

    println!("How many White rice and stew are you ordering? :");
    io::stdin().read_line(&mut input5).expect("Not a valid string");
    let f:f32 = input5.trim().parse().expect("Not a valid string");

    let _total:f32 = (a*_p)+(b*_f)+(c*_a)+(d*_e)+(f*_w);

    if _total > 10000.0
    {
        let d = _total * 0.95;
        println!("Total is {}", d); 
    }
    
    else 
    {
        println!("Total is {} ", _total);
    }     



}

