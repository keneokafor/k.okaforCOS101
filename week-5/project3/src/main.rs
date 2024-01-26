//Rust program that displays the following menu for the food items available to take order from the customer.

use std::io;

fn main() {
    let mut input1 = String::new();
    let mut  input2 = String::new();
    let mut input3 = String::new();
    let mut input4 = String::new();
    let mut input5 = String::new();

    println!("Enter Quantity Of Pounded Yam And Edinkaiko Soup: " );
    io::stdin().read_line(&mut input1).expect("Not a valid number");
    let poundo_yam_and_edinkaiko_soup:f32 = input1.trim().parse().expect("Not A Valid Order");

    println!("Enter The Quantity Of Fried Rice And Chicken: ");
    io::stdin().read_line(&mut input2).expect("Not a Valid Quantity");
    let fried_rice_and_chicken:f32 = input2.trim().parse().expect("Not a valid number");

    println!("Enter The Quantity of Amala And Ewedu Soup: ");
    io::stdin().read_line(&mut input3).expect("Not A Valid Number");
    let amala_and_ewedu_soup:f32 = input3.trim().parse().expect("Not a vaid Quantity");

    println!("Enter The Quantity of Eba And Egusi Soup: ");
    io::stdin().read_line(&mut input4).expect("Not a Valid Number");
    let eba_and_egusi_soup:f32 = input4.trim().parse().expect("Not a Valid Input");

    println!("Enter The Quantity Of White Rice And Stew: ");
    io::stdin().read_line(&mut input5).expect("Not a Valid Input");
    let white_rice_and_stew:f32 = input5.trim().parse().expect("Not a Valid Quantity");

    let p:f32 = poundo_yam_and_edinkaiko_soup * 3200.00;
    let f:f32 = fried_rice_and_chicken * 3000.00;
    let a:f32 = amala_and_ewedu_soup * 2500.00;
    let e:f32 = eba_and_egusi_soup * 2000.00;
    let w:f32 = white_rice_and_stew * 2500.00;
  // Calculate The Total 
   let t:f32 = p + f + a + e + w;
   println!("The Total Order Is N{} :",t);


   //The Discount 
   let d:f32 = t * 0.5;
   println!("The Discount is N{}",d);
   
   if t > 10_000.00
   {
    println!("The Discount is N{}",d);
   }
   else
   { 
    println!("The Final Order = Total Order :");
   }

   // The Final Charge 
   let fo:f32 = t - d;
   println!("The Final Charge is N{}",fo);
     
}
