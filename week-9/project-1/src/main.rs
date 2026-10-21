use std::io::Write;

fn main(){
    let categories = "Lager,Stout,Non-Alcoholic\n";
    let row1 = "33 Export,Legend,Maltina\n";
    let row2 = "Desperados,Turbo King,Amstel Malta\n";
    let row3 = "Goldberg,Williams,Malta Gold\n";
    let row4 = "Gulder,,Fayrouz\n";
    let row5 = "Heineken,,\n";
    let row6 = "Star,,\n";

    let mut file = std::fs::File::create("NB.csv").expect("File cannot be created");
    file.write_all(categories.as_bytes()).expect("the file cannot be written");
    file.write_all(row1.as_bytes()).expect("the file cannot be written");
    file.write_all(row2.as_bytes()).expect("the file cannot be written");
    file.write_all(row3.as_bytes()).expect("the file cannot be written");
    file.write_all(row4.as_bytes()).expect("the file cannot be written");
    file.write_all(row5.as_bytes()).expect("the file cannot be written");
    file.write_all(row6.as_bytes()).expect("the file cannot be written");

    println!("\nData written to file.");

}