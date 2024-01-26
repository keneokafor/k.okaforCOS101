//Rust program to manage the diagnostic history of patients in a clinic
 
use std::io;

fn main() {
   let mut input1 = String::new();
   let mut input2 = String::new();
   let mut input3 = String::new();
   let mut input4 = String::new();
   let mut input5 = String::new();
   let mut input6 = String::new();
   let mut input7 = String::new();
   let mut input8 = String::new();

   println!("Enter your name: ");
   io::stdin().read_line(&mut input1).expect("Not a valid string");
   let name = input1.trim().parse().expect("Not a valid input");

   println!("Enter your age: ");
   io::stdin().read_line(&mut input2).expect("Not a valid string");
   let age:f32 = input2.trim().parse().expect("Not a valid input");

   println!("Enter your Email Address: ");
   io::stdin().read_line(&mut input3).expect("Not a valid string");
   let email_address:f32 = input3.trim().parse().expect("Not a valid input");

   println!("Enter your phone number: ");
   io::stdin().read_line(&mut input4).expect("Not a valid string");
   let phone_number:i32 = input4.trim().parse().expect("Not a valid input");

   println!("Enter number of siblings: ");
   io::stdin().read_line(&mut input5).expect("Not a valid string");
   let number_of_siblings:f32 = input5.trim().parse().expect("Not a valid input");

   println!("Enter number of children: ");
   io::stdin().read_line(&mut input6).expect("Not a valid string");
   let number_of_children:f32 = input6.trim().parse().expect("Not a valid input");

   println!("Enter your medical diagnosis: ");
   io::stdin().read_line(&mut input7).expect("Not a valid string");
   let mut medical_diagnosis= input7.trim().parse().expect("Not a valid input");

   println!("Enter your village of residence: ");
   io::stdin().read_line(&mut input8).expect("Not a valid string");
   let mut village_of_residence = input8.trim().parse().expect("Not a valid input");
   
   //Total charges for all the medical diagnosis 
   let t1:f32 = 1200000.00;
   let t2:f32 = 550000.00;
   let t3:f32 = 1500000.00;
   let t4:f32 = 800000.00;
   let t5:f32 = 450000.00;

   //The discounts for all the medical diagnosis
   let a:f32 = t1 * 0.2;
   let b:f32 = t2 * 0.05;
   let c:f32 = t3 * 0.15;
   let d:f32 = t4 * 0.1;
   let e:f32 = t5 * 0.1;

   if medical_diagnosis =="Alzheimers" && age > 50.0 && number_of_children > 4.0 && village_of_residence ="Akpabom" {
       println!("Your total charge is {} with details {}, {}, {}, {}, {}, {}, {}, {} ", a, name, age, phone_number, email_address, medical_diagnosis, village_of_residence, number_of_children, number_of_siblings);
   }
   else {
       println!("Your total charge is {} with details {}, {}, {}, {}, {}, {}, {}, {} ", t1, name, age, phone_number, email_address, medical_diagnosis, village_of_residence, number_of_children, number_of_siblings);
   }
   if medical_diagnosis =="Arrythmia" && age = 30.0 && number_of_siblings > 4.0 && village_of_residence = "Ngbauji"{
       println!("Your total charge is {} with details {}, {}, {}, {}, {}, {}, {}, {} ", b, name, age, phone_number, email_address, medical_diagnosis, village_of_residence, number_of_children, number_of_siblings);
   }
   else {
       println!("Your total charge is {} with details {}, {}, {}, {}, {}, {}, {}, {} ", t2, name, age, phone_number, email_address, medical_diagnosis, village_of_residence, number_of_children, number_of_siblings );
   }
   if medical_diagnosis =="CKD" && age > 40.0 && number_of_children > 3.0 && number_of_siblings > 3.0 && village_of_residence = "Atabrikang"{
       println!("Your total charge is {} with details {}, {}, {}, {}, {}, {}, {}, {} ", c, name, age, phone_number, email_address, medical_diagnosis, village_of_residence, number_of_children, number_of_siblings);
   }
   else {
       println!("Your total charge is {} with details {}, {}, {}, {}, {}, {}, {}, {} ", t3, name, age, phone_number, email_address, medical_diagnosis, village_of_residence, number_of_children, number_of_siblings );
   }
   if medical_diagnosis =="Diabetes" && age >= 28.0 && number_of_children >= 2.0  && village_of_residence = "Okorobilom" {
       println!("Your total charge is {} with details {}, {}, {}, {}, {}, {}, {}, {} ", d, name, age, phone_number, email_address, medical_diagnosis, village_of_residence, number_of_children, number_of_siblings);
   }
   else {
       println!("Your total charge is {} with details {}, {}, {}, {}, {}, {}, {}, {} ", t4, name, age, phone_number, email_address, medical_diagnosis, village_of_residence, number_of_children, number_of_siblings );
   }
   if medical_diagnosis =="Arthritis" && age > 58.0 && number_of_children > 5.0 && number_of_siblings > 5.0 && village_of_residence = "Emeremen"{
       println!("Your total charge is {} with details {}, {}, {}, {}, {}, {}, {}, {} ", e, name, age, phone_number, email_address, medical_diagnosis, village_of_residence, number_of_children, number_of_siblings);
   }
   else {
       println!("Your total charge is {} with details {}, {}, {}, {}, {}, {}, {}, {} ", t5, name, age, phone_number, email_address, medical_diagnosis, village_of_residence, number_of_children, number_of_siblings );
   }


}
