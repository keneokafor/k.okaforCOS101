use std::io;
use std::io::Write;

fn main() {

    let mut lager = Vec::new();
    let mut stout = Vec::new();
    let mut non_alcoholic = Vec::new();

    println!("Welcome to Nigeria Breweries PLC");

loop {
    println!("What category of drinks would you like to order from? (Lager, Stout, Non-Alcoholic)");
    loop {
        let mut welcome_input = String::new();
        io::stdin().read_line(&mut welcome_input).expect("Failed to read input");
        let welcome_input = welcome_input.trim().to_lowercase();

    
        if welcome_input == "lager" {
            println!("How many lager drinks would you like to add to order?");
            let mut lager_no = String::new();
            io::stdin().read_line(&mut lager_no).expect("Failed to read input");
            let lager_no:u32 = lager_no.trim().parse().expect("Failed to read input");

            for x in 1..=lager_no {
                println!("Input drink {}", x);
                let mut lager_drinks = String::new();
                io::stdin().read_line(&mut lager_drinks).expect("Failed to read input");
                lager.push(lager_drinks);
            }
            break;
        }

        else if welcome_input == "stout" {
            println!("How many stout drinks would you llke to add to order?");
            let mut stout_no = String::new();
            io::stdin().read_line(&mut stout_no).expect("Failed to read input");
            let stout_no:u32 = stout_no.trim().parse().expect("Failed to read input");

            for x in 1..=stout_no {
                println!("Input drink {}", x);
                let mut stout_drinks = String::new();
                io::stdin().read_line(&mut stout_drinks).expect("Failed to read input");
                stout.push(stout_drinks);
            }
            break;
        }

        else if welcome_input == "non-alcoholic" {
            println!("How many non-alcoholic drinks would you like to add to order?");
            let mut non_alcoholic_no = String::new();
            io::stdin().read_line(&mut non_alcoholic_no).expect("Failed to read input");
            let non_alcoholic_no:u32 = non_alcoholic_no.trim().parse().expect("Failed to read input");

            for x in 1..=non_alcoholic_no {
                println!("Input drink {}", x);
                let mut non_alcoholic_drinks = String::new();
                io::stdin().read_line(&mut non_alcoholic_drinks).expect("Failed to read input");
                non_alcoholic.push(non_alcoholic_drinks);
            }
            break;
        }
        else {
            println!("Invalid Input
                    \nPlease input either Lager, Stout or Non-Alcoholic");
            continue;
        }
    }
    
    println!("Would you like to order anything else? (Y for Yes and  N for No)");
    
        let mut another_choice = String::new();
        io::stdin().read_line(&mut another_choice).expect("Failed to read input");
        let another_choice = another_choice.trim().to_lowercase();

        if another_choice == "y" {
            continue;
        }
        else if another_choice == "n" {
            break;
        }
        else {
            break;
        }
}
    println!("Thank you, your order has been fulfilled");

    let mut file = std::fs::File::create("Heineken Drink System.txt").expect("Failed to create file");

    if lager.len() > 0 {
        file.write_all("LAGER DRINKS:\n\n".as_bytes()).expect("Failed to write into file");

        for index in 0..lager.len() {
            file.write_all(lager[index].as_bytes()).expect("Failed to write into file");
        }
    }

    if stout.len() > 0 {
        file.write_all("\n\nSTOUT DRINKS:\n\n".as_bytes()).expect("Failed to write into file");

        for index in 0..stout.len() {
            file.write_all(stout[index].as_bytes()).expect("Failed to write into file");
        }
    }

    if non_alcoholic.len() > 0 {
        file.write_all("\n\nNON-ALCOHOLIC DRINKS: \n\n".as_bytes()).expect("Failed to write into file");

        for index in 0..non_alcoholic.len() {
            file.write_all(non_alcoholic[index].as_bytes()).expect("Failed to write into file");
        }
    }

}