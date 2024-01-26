use std::io::Write;

fn main() {
    // Creating the 
    let number:[i32; 5] = [1,2,3,4,5];
    let names:[&str; 5] = ["Aigbogun Alamba Daudu","Murtala Afeez Bendu","Okorocha Calistus","Adewale Jimoh Akanbi","Osazuwa Faith Etieye"];
    let ministry:[&str; 5] = ["Internal Affairs","Justice","Defense","Power & Steel","Petroleum"];
    let zone:[&str; 5] = ["South West","North East","South South","South West","South East"];
    
    let mut info: Vec<(i32, &str, &str, &str)> = Vec::new();
    
    for index in 0..number.len() {
        let person_info = (number[index],names[index],ministry[index],zone[index]);
        info.push(person_info);
    }

    // Creating a File
    let mut file = std::fs::File::create("Political Dataset.txt").expect("Failed to create file");

    let header = ["S/N", "Name of Commissioner", "Ministry", "Geopolitical Zone"];

    for element in &header {
        file.write_all(element.as_bytes()).expect("Failed to write to file");
        file.write_all(b"\t\t\t").expect("Failed to write to file");
    }
        file.write_all(b"\n").expect("Failed to write to file");

    for person in &info {
        file.write_all(person.0.to_string().as_bytes()).expect("Failed to write to file");
        file.write_all(b"\t\t\t").expect("Failed to write to file");

        file.write_all(person.1.as_bytes()).expect("Failed to write to file");
        file.write_all(b"\t\t\t").expect("Failed to write to file");

        file.write_all(person.2.as_bytes()).expect("Failed to write to file");
        file.write_all(b"\t\t\t").expect("Failed to write to file");

        file.write_all(person.3.as_bytes()).expect("Failed to write to file");

        file.write_all(b"\n").expect("Failed to write to file");
        
    }


}
