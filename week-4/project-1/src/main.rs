use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter your value for a: ");
    io::stdin().read_line(&mut input1).expect("This is not a valid input");
    let a:f32 = input1.trim().parse().expect("This is not a valid input");
    
    println!("Enter your value for b: ");
    io::stdin().read_line(&mut input2).expect("This is not a valid input");
    let b:f32 = input2.trim().parse().expect("This is not a valid input");

    println!("Enter your value for c: ");
    io::stdin().read_line(&mut input3).expect("This is not a valid input");
    let c:f32 = input3.trim().parse().expect("This is not a valid input");

    let d:f32 = b*b - 4.0*a*c;

    if d>0.0 {
        let root_1:f32 = (-b + (b*b - 4.0*a*c).sqrt()) / (2.0 * a);
        let root_2:f32 = (-b - (b*b - 4.0*a*c).sqrt()) / (2.0 * a);
        println!("The 2 distinct rootsof these values are {} and {}",root_1,root_2);
    }
    else if d == 0.0{
        let root_3:f32 = -b/2.0*a;
        println!("The root of these values is {}",root_3);
    }
    else{
        println!("There are no real roots.");
    }

}
