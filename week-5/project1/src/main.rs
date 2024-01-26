//Rust program to Derive Roots for 

use std::io;

fn main() {
    println!("Enter the value of a");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Not a valid number");
    let a:f32 = input1.trim().parse().expect("Not a valid input");

    println!("Enter the value of b");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Not a valid number");
    let b:f32 = input2.trim().parse().expect("Not a valid input");

    println!("Enter the value of c");
    let mut input3 = String::new();
    io::stdin().read_line(&mut input3).expect("Not a valid number");
    let c:f32 = input3.trim().parse().expect("Not a valid input");

    //calculate the discriminant
    let d:f32 = b.powf(2.0) - 4.0 * a * c;

    if discriminant > 0.0 {
        //Two distinct real roots 
        let root1 = (-b + d.sqrt(2.0)) / (2.0 * a);
        let root2 = (-b - d.sqrt(2.0)) / (2.0 * a);
        println!("When the discriminant is greater than: Root1= {}, Root2= {}",root1, root2);

    } else if discriminant == 0.0 {
        //one real root
        let root = -b / (2.0 * a);
        println!("Since the discriminant is equal to zero: Root = {}",root);
    } else{
        //Imaginary / No real roots
        println!("No real roots (discriminant is negative).");
    }
    
}

