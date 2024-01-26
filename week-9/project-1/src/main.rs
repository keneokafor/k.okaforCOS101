use std::io::Write;

fn main() {
    let mut file = std::fs::File::create("drinks1.txt").expect("create failed");

    let comp_name = "Nigerian Brewery Limited";

    let category1 = vec!["33 Export","Desperados","Goldberg","Gulder","Heineken","Star"];

    let category2 = vec!["Legend","Turbo King","Williams"];

    let category3 = vec!["Maltina","Amstel Malta","Malta Gold","Fayrouz"];

 

    file.write_all(comp_name.as_bytes()).expect("write failed");

    file.write_all(b"\n\n").expect("write failed");
    file.write_all("Lager".as_bytes()).expect("write failed");

    for index in 0..category1.len() {
        file.write_all(b"\n").expect("write failed");
        file.write_all(category1[index].as_bytes()).expect("write failed");

    }
    file.write_all(b"\n").expect("write failed");

    file.write_all(b"\n").expect("write failed");
    file.write_all("Stout".as_bytes()).expect("write failed");

    for index in 0..category2.len() {
        file.write_all(b"\n").expect("write failed");
        file.write_all(category2[index].as_bytes()).expect("write failed");
    }
    file.write_all(b"\n").expect("write failed");

    file.write_all(b"\n").expect("write failed");
    file.write_all("Non-Alchoholic".as_bytes()).expect("write failed");

    for index in 0..category3.len() {
        file.write_all(b"\n").expect("write failed");
        file.write_all(category3[index].as_bytes()).expect("write failed");
    }
    
    println!("Data written to file");
}
