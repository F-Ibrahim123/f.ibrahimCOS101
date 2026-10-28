use std::io;
struct Laptops{
    brand:String,
    quantity:u32,
    unit_price:u32 
}
impl Laptops{
    fn total_cost(&self)->u32{
        let mut brand1 = String::new();
          println!("There are {} available {} laptops.",self.quantity,self.brand);
          println!("How many {} Laptops do you want to buy? \n",self.brand);
          io::stdin().read_line(&mut  brand1).expect("This is an invalid input");
          let brand1:u32 = brand1.trim().parse().expect("This is an invalid input");
          
            let price = self.unit_price * brand1;
          return price;
          
}
}
fn main() {
    let lap1 = Laptops{
        brand:String::from("HP"),
        quantity: 10,
        unit_price:650_000
    };
    let lap2 = Laptops{
        brand:String::from("IBM"),
        quantity: 6,
        unit_price: 755_000
    };
    let lap3 = Laptops{
        brand:String::from("Toshiba"),
        quantity:10,
        unit_price: 550_000
    };
    let lap4 = Laptops{
        brand:String::from("Dell"),
        quantity: 4,
        unit_price: 850_000
    };
     println!("Welcome to Alaba Electronics! \n");
    let price1 = lap1.total_cost();
    let price2 = lap2.total_cost();
    let price3 = lap3.total_cost();
    let price4 = lap4.total_cost();

    println!("The total cost price is {}",price1+price2+price3+price4);
    

}

