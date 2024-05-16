use std::io;

fn trapezium() {
    let mut input1 = String::new();
    println!("Enter the height of the trapezium:");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let a:i32 = input1.trim().parse().expect("failed to read input");

    let mut input2 = String::new();
    println!("Enter the base1 of the trapezium");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let b:i32 = input2.trim().parse().expect("Failed to read input");

    let mut input3 = String::new();
    println!("Enter the base2 of the trapezium");
    io::stdin().read_line(&mut input3).expect("Failed to read input");
    let c:i32 = input3.trim().parse().expect("Failed to read input");

    let area_of_trapezium = a/2 * (b + c);
    println!("Area of trapezium is : {}", area_of_trapezium);
}

fn rhombus() {
    let mut input1 = String::new();
    println!("Enter the value for the first diadonal of the rhombus");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let a:i32 = input1.trim().parse().expect("Failed to read input");

    let mut input2 = String::new();
    println!("Enter the value for the second diadonal of the rhombus");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let b:i32 = input2.trim().parse().expect("Failed to read input");

    let area_of_rhombus = 1/2 * a * b;
    println!("Area of the rhombus is : {} ",area_of_rhombus);

}   

fn parallelogram() {
    let mut input1 = String::new();
    println!("Enter the value for the base of the parallelogram");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let a:i32 = input1.trim().parse().expect("Failed to read input");

    let mut input2 = String::new();
    println!("Enter value for the altitude of the parallelogram");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let b:i32 = input2.trim().parse().expect("Failed to read input");

    let area_of_parallelogram = a*b;
    println!("Area of the parallelogram is : {}",area_of_parallelogram);
}

fn cube() {
    let mut input1 = String::new();
    println!("Enter the value the length of the sides of the cube");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let a:i32 = input1.trim().parse().expect("Failed to read input");

    let area_of_cube = 6*a*a;
    println!("Area of the cube is : {}",area_of_cube);
}

fn cylinder() {
    let mut input1 = String::new();
    println!("Enter the value for the radius of the cylinder");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let a:i32 = input1.trim().parse().expect("Failed to read input");

    let mut input2 = String::new();
    println!("Enter the value for the height of the cylinder");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let b:i32 = input2.trim().parse().expect("Failed to read input");

    let volume_of_cylinder = 22/7 * a * a * b;
    println!("Volume of the cylinder is: {}",volume_of_cylinder);
}

fn main() {
    let mut input4 = String::new();
    println!("Welcome! \nPlease select what equation you need. \nArea trapezium \tPress 1 \nArea of rhombus \tPress 2 \nArea of parallelogram \tPress 3 \nArea of cube \tPress 4 \nVolume of cylinder \tPress 5 ");
    io::stdin().read_line(&mut input4).expect("Failed to read input");
    let d:i32 = input4.trim().parse().expect("Failed to read input");

    if d == 1
    {
        trapezium()
    }
    else if d == 2
    {
        rhombus()
    }
    else if d == 3
    {
        parallelogram()
    }
    else if d == 4
    {
        cube()
    }
    else if d == 5
    {
        cylinder()
    }

}