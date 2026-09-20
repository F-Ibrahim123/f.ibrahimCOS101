use std::io;

fn main() {
    println!("Welcome to the annual incentive calculator!");

    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Please input your age:");
    io::stdin().read_line(&mut input1).expect("This is not a valid input");
    let age:u8 = input1.trim().parse().expect("This is not a valid input");

    println!("Are you experienced? (Yes/No):  ");
    io::stdin().read_line(&mut input2).expect("This is not a valid input.");
    let input2:bool = true;


        if input2{
         if age >= 40 {
            println!("Your annual incentive is #1,560,000.");
        }
         else if age >= 30 && age <= 39{
            println!("Your annual incentice is #1,480,000");
        }
         else if age < 28{
            println!("Your annual incentive is #1,300,000");
        }
    }
        else{
            println!("Your annual incentive is #100");
        }


}
