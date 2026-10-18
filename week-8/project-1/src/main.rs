use std::io;

fn main() {
    
    let mut sector = String::new();
    let mut input1 = String::new();
    let mut exp = String::new();

    let a = vec!["Intern","Administrator","Senior Administrator","Office Manager","Director","CEO"];
    let b = vec!["","Research Assistant","PHD Candidate","Post-Doc Researcher","Senior Lecturer","Dean"];
    let c = vec!["Paralegal","Junior Associate","Associate","Senior Associate 1-2","Senior Asoociate 3-4","Partner"];
    let d = vec!["Placement","Classroom Teacher","Senior Teacher","Leading Teacher","Deputy Principal","Principal"];

    println!("Welcome to the Public Service APS Level Checker!");
    println!("\nFollow the instructions to determine your staff level!");
    
    println!("\nFirst which sector do you work in?");
    println!("Input:");
    println!("a for Office Admin");
    println!("b for Academics");
    println!("c for Law");
    println!("d for Teaching");
    io::stdin().read_line(&mut sector).expect("This is an invalid input");
    let sector: char = sector.trim().parse().expect("This is an invalid response");

    println!("Do you want to determine your APS level by: \n1.position \nor \n2.experience? \n(Input 1 or 2)");
    io::stdin().read_line(&mut input1).expect("This is not a valid input");
    let choice:u16 = input1.trim().parse().expect("This is an invalid input");

    if choice == 1 {
     println!("Now which position do you hold?");
     println!("Input:");
     match sector {
        'a' => position_check(&a) ,
        'b' => position_check(&b) ,
        'c' => position_check(&c) ,
        'd' => position_check(&d) ,
        _ => println!("This is not a valid position"),
        }
    }
    else if choice ==2{
    println!("How many years of experience do you have?");
    io::stdin().read_line(&mut exp).expect("This is not a valid input");
    let exp:u16 = exp.trim().parse().expect("This is an invalid input");

     if exp >=1 && exp <=2{
        println!("Your aps level is APS 1-2");
     }
     else if exp >= 3 && exp <=5{
        println!("Your aps level is APS 3-5"); 
     }
     else if exp >=5 && exp <= 8{
        println!("Youraps level is APS 5-8"); 
     }
     else if exp >= 8 && exp <= 10{
        println!("Your aps level is ELI 8-10"); 
     }
     else if exp >= 10 && exp<= 13 {
        println!("Your aps level is ELI 10-13"); 
     }
     else if exp>13{
        println!("Your aps level is SES"); 
     }
    
    }
    else {
        println!("This is not a valid option!");
    }
}


fn position_check(x: &[&str]) {
    let mut position = String::new();
    let mut aps = "APS Level";

    for i in 0..x.len(){
        println!("{} for {}",i + 1,x[i]);
    }
    io::stdin().read_line(&mut position).expect("This is an invalid input");
    let position:u16 = position.trim().parse().expect("This is not a valid input");

    match position{
        1 => aps = "APS 1-2",
        2 => aps = "APS 3-5",
        3 => aps = "APS 5-8",
        4 => aps = "ELI 8-10",
        5 => aps = "ELI 10-13",
        6 => aps = "SES",
        _ => println!("This is not a valid input"),
    }
    println!("Your APS Level is {}",aps);


}
