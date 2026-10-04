use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut price:u32 = 0;
    let mut more = String::new();
    let mut input3 = String::new();
    let mut input4 = String::new();
    let mut more2 = String::new();
    

    
    println!("MENU:");
    println!("Poundo Yam & Eddikaikong Soup - #3,200");
    println!("Fried Rice & Chicked - #3,000");
    println!("Amala & Ewedu - #2,500");
    println!("Eba & Egusi - #2,000");
    println!("White Rice & Stew - #2,500");

    println!("\nTo Order, Input: \n P for Poundo Yam & Edikaikong \n F for Fried Rice & Chicken \n A for Amala & Ewedu \n E for Eba & Egusi \n W for White Rice & Stew:\n");
    io::stdin().read_line(&mut input1).expect("This is not a valid input");
    let food1:char = input1.trim().parse().expect("This is not a valid input");

    println!("What Quantity of this would you like to buy?");
    io::stdin().read_line(&mut input2).expect("This is not a valid input");
    let qty1:u32 = input2.trim().parse().expect("This is not a valid input.");

    if food1 == 'P'{
          price = 3_200 * qty1;
    }
    else if food1 == 'F'{
          price = 3_000 * qty1;
    }
    else if food1 == 'A'{
        price = 2_500 * qty1;
    }
    else if food1 == 'E'{
        price = 2_000 * qty1;
    }
    else if food1 == 'W'{
        price =  2_500 * qty1;
    }
    else{
        println!("Invalid food input");
    }

    println!("Do You Still Want To Order More Food?(Yes/No)");
    io::stdin().read_line(&mut more).expect("This is not a valid input");

    while more.trim() == "Yes"{
        println!("Input Your Food Choice:\n");
        io::stdin().read_line(&mut input3).expect("This is not a valid input");
        let food2:char = input3.trim().parse().expect("This is not a valid input");

        println!("What Quantity of this would you like to buy?");
        io::stdin().read_line(&mut input4).expect("This is not a valid input");
        let qty2:u32 = input4.trim().parse().expect("This is not a valid input.");

        if food2 == 'P'{
          price = price + 3_200 * qty2;
    }
    else if food2 == 'F'{
          price = price + 3_000 * qty2;
    }
    else if food2 == 'A'{
        price = price + 2_500 * qty2;
    }
    else if food2 == 'E'{
        price = price + 2_000 * qty2;
    }
    else if food2 == 'W'{
        price =  price + 2_500 * qty2;
    }
    else{
        println!("Invalid food input");
    }

    println!("Do You Still Want To Order More Food?(Yes/No)");
    io::stdin().read_line(&mut more2).expect("This is not a valid input");

    if more2.trim() == "Yes"{
        more = "Yes".to_string();
    }
    else{
        more = "No".to_string();
    }


    }


/*if price > 10_000{
    price = price * (95/100);
    
}*/
println!("The price of your food is: {}",price);

    

    
    




    
}
