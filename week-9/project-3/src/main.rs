use std::io::Write;

fn main() {
    let mut commissioner = vec!["Aigbogun Alamba Daudu","Murtala Afeex Bendu","Okorocha Calistus Ogbona","Adewale Jimoh Akanbi","Osazuwa Faith Etieye"];
    let mut ministry = vec!["Internal Affairs","Justice","Defense","Power & Steel","Petroleum"];
    let mut zone = vec!["South West","North East","South South","South West","South East"];

    let mut file = std::fs::File::create("convicted_ministers.csv").expect("File cannot be created");
    file.write_all("Commissioner,Ministry,Zone\n".as_bytes()).expect("Write Failed");
    for i in 0..zone.len(){
        let data = format!("{},{},{}\n",commissioner[i],ministry[i],zone[i]);
        file.write_all(data.as_bytes()).expect("Write Failed");
    }
    println!("Data Written");
}
