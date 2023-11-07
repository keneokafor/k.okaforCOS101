//Rust program to calculate the speed of a car

use std::io;
fn main() 
{
    let mut distance = String::new();
    let mut time = String::new();
    
    println!("Enter distance travelled by the car: ");
    io::stdin().read_line(&mut distance).expect("Not a valid string");
    let distance:f64 = distance.trim().parse().expect("Not a valid number");

    println!("Enter time taken to travel by the car: ");
    io::stdin().read_line(&mut time).expect("Not a valid string");
    let time:f64 = time.trim().parse().expect("Not a valid number");

    let speed:f64 = (distance* 1.60934)/time;

    println!("Speed of car: {} kilometers per hour",speed);
}