use std::io;

    fn main() {
    let mut age = String::new();
    let mut experience = String::new();

    println!("\nEnter experience type, 'experienced' or 'inexperienced': ");
    io::stdin().read_line(&mut experience).expect("Not a valid input");
    experience = experience.trim().parse().expect("Not a valid number");

    println!("\nEnter the age of the employee:", );
    io::stdin().read_line(&mut age).expect("Not a valid input");
    let age:f32 = age.trim().parse().expect("Not a valid number");

    if experience == "experienced" && age >= 40.00
    {
       println!("Annual incentive is 1,560,000");
    }
    else if experience == "experienced" && age >= 30.00 && age <= 40.00
    {
       println!("Annual incentive is 1,480,000");
    } 
    else if experience == "experienced" && age < 28.00 
     {
       println!("Annual incentive is 1,300,000");
    } 
    else if experience == "inexperienced"
     {
       println!("Annual incentive is 100,000");
    } 
    
}
