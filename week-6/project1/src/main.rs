//Rust program to develop a Researchers Publication Incentive System

use std::io;

fn main() {
   for _n in 1..501{

   let mut input1 = String::new();
   println!("Enter your name: ");
   io::stdin().read_line(&mut input1).expect("Not a valid string");
   
   let mut input2 = String::new();
   println!("Enter number of papers written: ");
   io::stdin().read_line(&mut input2).expect("Not a valid string");
   let number:f32 = input2.trim().parse().expect("Not a valid input");

     //Total incentive for number of papers written
     let t1:f32 = 100000.0;
     let t2:f32 = 500000.0;
     let t3:f32 = 800000.0;
     let t4:f32 = 1000000.0;

     if number > 3.0 && number < 5.0
     {
        println!("{}, Your total incentive is {}", input1, t2);
     }
     if number > 5.0 && number < 10.0 {
       println!("{}, Your total incentive is {}", input1, t3);
     }
     if number >= 10.0 {
       println!("{}, Your total incentive is {}", input1, t4);
     }
     if number < 3.0  {
       println!("{}, Your total incentive is {}", input1, t1);
     }
    }
}