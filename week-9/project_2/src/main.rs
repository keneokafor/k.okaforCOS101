use std::io::Write;

fn main() {
    // HEADER INFORMATION
    let header = vec!["Student Name", "Matric. Number", "Department", "Level"];

    // VECTOR TO STORE STUDENT INFO AS TUPLES
    let mut student_info: Vec<(&str, &str, &str, i32)> = Vec::new();

    let student_1 = ("Oluchi Mordi", "ACC10211111", "Accounting",300);
    student_info.push(student_1);

    let student_2= ("Adams Aliyu","ECO10110101", "Economics", 100);
    student_info.push(student_2);

    let student_3 = ("Shania Bolade","CSC10328828","Computer", 200);
    student_info.push(student_3);

    let student_4 = ("Adekunle Gold","EEE11020202","Electrical", 200);
    student_info.push(student_4);

    let student_5 = ("Blanca Edemoh","MEE10202001","Mechanical",100);
    student_info.push(student_5);

    // CREATING A FILE NAMED SIMS
    let mut file = std::fs::File::create("SIMS.txt").expect("Failed to create file");

    //WRITING THE HEADER TO FILE
    for item in &header {
        file.write_all(item.as_bytes()).expect("Failed to write to file");
        file.write_all(b"\t").expect("Failed to write to file"); // ADD A TAB AFTER EACH HEADER
    }
        file.write_all(b"\n").expect("Failed to write to file"); // ADD A NEW LINE AFTER THE HEADER

        // WRITING STUDENT INFO TO FILE
    for student in &student_info {
        file.write_all(student.0.as_bytes()).expect("Failed to write into file");
        file.write_all(b"\t").expect("Failed to write to file"); // ADDS A TAB AFTER EACH ELEMENT

        file.write_all(student.1.as_bytes()).expect("Failed to write into file");
        file.write_all(b"\t").expect("Failed to write to file");

        file.write_all(student.2.as_bytes()).expect("Failed to write into file");
        file.write_all(b"\t").expect("Failed to write to file");

        file.write_all(student.3.to_string().as_bytes()).expect("Failed to write into file");
        file.write_all(b"\n").expect("Failed to write to file"); // ADDS A NEW LINE AFTER EACH STUDENT
    }
    

}
