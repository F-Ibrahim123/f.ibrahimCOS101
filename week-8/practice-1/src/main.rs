fn main() {
    //Using Vec::new()
    let v1 : Vec<i64> = Vec::new();

    //printing the size of the vector
    println!("\nThe length of Vec::new() is {}",v1.len());

    //Using macro
    let v2 = vec!["Grace","Fareedah","Ibrahim","Oyinkansola","Haleemah"];

    //printing the size of the vector
    println!("\nThe length of vec macro is: {}",v2.len());
}
