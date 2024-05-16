fn main() {
    let mut siblings : Vec<String> = Vec::new();
    let mut age: Vec<i32> = Vec::new();
    let mut offspring: Vec<String> = Vec::new();
    let mut city: Vec<String> = Vec::new();
    let mut university: Vec<String> = Vec::new();
    let mut course_of_study: Vec<String> = Vec::new();
    let mut secondary: Vec<String> = Vec::new();
    let mut class: Vec<String> = Vec::new();
    let mut input1 = String::new();
    println!("How many siblings do you have?");
    std::io::stdin().read_line(&mut input1).expect("Failed to read input");
    let siblings_num:i32 = input1.trim().parse().expect("Invalid input");
    for count in 0..siblings_num 
    {
        let mut input2 = String::new();
        println!("Enter Sibling {}", count+1);
        std::io::stdin().read_line(&mut input2).expect("Failed to read input");
        let new_siblings:String = input2.trim().parse().expect("Invalid input");
        siblings.push(new_siblings);
        let mut input3 = String::new();
        println!("Enter Age of Sibling {}",count+1);
        std::io::stdin().read_line(&mut input3).expect("Failed to read input");
        let new_age:i32 = input3.trim().parse().expect("Invalid input");
        age.push(new_age);

        if age > vec![18]
        {
            let mut input4 = String::new();
            println!("Is sibling married?[true/false]");
            std::io::stdin().read_line(&mut input4).expect("Failed to read input");
            let a:bool = input4.trim().parse().expect("Invalid input");

            if a == false
            {
                let mut  input5 = String::new();
                println!("Student[true]\nWorker[false]");
                std::io::stdin().read_line(&mut input5).expect("Failed to read input");
                let b:bool = input5.trim().parse().expect("Invalid input");

                if b == true
                {
                    println!("What University?");
                    let mut input8 = String::new();
                    std::io::stdin().read_line(&mut input8).expect("Failed to read input");
                    let new_university:String = input8.trim().parse().expect("Invalid nput");
                    university.push(new_university);
                    println!("What course of study?");
                    let mut input9 = String::new();
                    std::io::stdin().read_line(&mut input9).expect("Failed to read input");
                    let new_course_of_study:String = input9.trim().parse().expect("Invalid input");
                    course_of_study.push(new_course_of_study);
                }

                else
                {
                    println!("Worker");
                }  
            }
            else 
            {
                println!("Do you have any offspring");
                let mut input6 = String::new();
                std::io::stdin().read_line(&mut input6).expect("Failed to read input");
                let new_offspring:String = input6.trim().parse().expect("Invalid input");
                offspring.push(new_offspring);
                println!("What city do they stay in?");
                let mut input7 = String::new();
                std::io::stdin().read_line(&mut input7).expect("Failed to read input");
                let new_city:String = input7.trim().parse().expect("Invalid input");
                city.push(new_city);
            }

        }

        else if age < vec![18]
        {
                   println!("Are they eligible for WAEC [true/false]");
                   let mut input10 = String::new();
                   std::io::stdin().read_line(&mut input10).expect("Failed to read input");
                   let c:bool = input10.trim().parse().expect("Invalid input");

                if c == true
             {
                    let mut input11 = String::new();
                    println!("What secondary school did they attend?");
                    std::io::stdin().read_line(&mut input11).expect("Failed to read input");
                    let new_secondary:String = input11.trim().parse().expect("Invalid input");
                    secondary.push(new_secondary);
             }
                else
             {
            let mut input12 = String::new();
            println!("What is their current class level");
            std::io::stdin().read_line(&mut input12).expect("Failed to read input");
            let new_class:String = input12.trim().parse().expect("Invalid input");
            class.push(new_class);
            }

        }

    }
    for i in 0..siblings_num
    {
        println!("Detailes for sibling {} is : \nName:{}\nAge:{}\nOffspring:{}\nCity:{}\nUniversity:{}\nCourse of study:{}\nSecondary School{}\nCurrent Class level{}",i,siblings[i],age[i],offspring[i],city[i],university[i],course_of_study[i],secondary[i],class[i]);
    }
    
}    
    
        
    
    

     