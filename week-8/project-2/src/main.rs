use std::io;

fn main() {
    
    println!("Welcome to the experience level checker for Ernst & Young Global Limited");
    println!("How many applicants are you interviewing?");
    let mut count = String::new();
    io::stdin().read_line(&mut count).expect("This is an invalid input");
    let count:u8 = count.trim().parse().expect("This is an invalid input");
    let mut v: Vec<(String,u8)> = Vec::new();
    for i in 0..count{
        println!("Applicant {}",i + 1);
        let mut a = String::new();
    println!("What is your name?");
    io::stdin().read_line(&mut a).expect("This is an invalid input");
    let a = a.trim().to_string();
    let mut b = String::new();
    println!("How many years of experience do you have?");
    io::stdin().read_line(&mut b).expect("This is an invalid input");
    let b:u8 = b.trim().parse().expect("Thi is an invalid input");
    v.push((a,b));
   }
   if let Some((a,b)) = v.iter().max_by_key(|(_,b)| b){
    println!("The most experienced applicant is {} with {} years",a,b);
   }
}

    