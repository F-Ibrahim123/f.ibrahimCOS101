use std::io::Read;
use std::io::Write;
use std::fs;

fn main(){
    let mut info = vec![("Oluchi Mordi","ACC10211111","Accounting",300),("Adams Aliyu","ECO10110101","Economics",100),("Shaina Bolade","CSC10328828","Computer",200),("Adekunle Gold","EEE11020202","Electrical",200),("Blanca Edemoh","MEE10202001","Mechanical",100)];
    let mut file = std::fs::File::create("SMS.csv").expect("Could not create file");
    file.write_all("StudentName,Matric.Number,Department,Level\n".as_bytes()).expect("Write Failed");
    
    for i in 0..info.len() {
             let mut info2 = format!("{},{},{},{}\n",info[i].0,info[i].1,info[i].2,info[i].3);
             println!("{}",info2);
             file.write_all(info2.as_bytes()).expect("Write Failed");
             
    }
    println!("Data written to file");
}