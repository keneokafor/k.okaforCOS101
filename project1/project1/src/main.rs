use std::fs::File;
use std::io::{self, Write};

// We are going to write a struct program to represent the companies
#[derive(Debug)]
struct Company {
    name_of_company: String,
    year_founded: i32,
    number_of_shares: i32,
    number_of_liabilities: i32,
}

impl Company {
    // We are going to calculate the percentages for the companies
    fn calculate_leverage(&self) -> f64 {
        (self.number_of_shares as f64 - self.number_of_liabilities as f64) / self.number_of_shares as f64 * 100.0

    // Not sure of this previous line...Crosscheck before submission**
    }
}

fn main() {
    // We are going to create a vector file to store the companies data
    let companies = vec![
        Company {
            name_of_company: String::from("Cadbury"),
            year_founded: 1965,
            number_of_shares: 15_000_000,
            number_of_liabilities: 5_500_000,
        },
         Company {
            name_of_company: String::from("Champion"),
            year_founded: 1974,
            number_of_shares: 25_000_000,
            number_of_liabilities: 8_000_000,
        },
         Company {
            name_of_company: String::from("Dangote Sugar"),
            year_founded: 1970,
            number_of_shares: 18_000_000,
            number_of_liabilities: 10_000_000,
        },
         Company {
            name_of_company: String::from("Flour Mills"),
            year_founded: 1960,
            number_of_shares: 32_000_000,
            number_of_liabilities: 4_000_000,
        },
         Company {
            name_of_company: String::from("Nestle"),
            year_founded: 1961,
            number_of_shares: 8_000_000,
            number_of_liabilities: 1_500_000,
        },
         Company {
            name_of_company: String::from("Unilever"),
            year_founded: 1923,
            number_of_shares: 37_000_000,
            number_of_liabilities: 11_000_000,
        },
         Company {
            name_of_company: String::from("Honeywell"),
            year_founded: 1906,
            number_of_shares: 34_000_000,
            number_of_liabilities: 9_000_000,
        },
         Company {
            name_of_company: String::from("Nigerian Brewies"),
            year_founded: 1946,
            number_of_shares: 30_000_000,
            number_of_liabilities: 12_000_000,
        },
        
    ];

    // We are going to get the username and password
    let mut username = String::new();
    let mut password = String::new();

    // we are going to input the criteria for validating the username and password
    loop {
        println!("Enter username:");
        io::stdin().read_line(&mut username).expect("Failed to read line");


        if validate_username(&username) {
            break;
        } else {
            println!("Invalid username. Please try again.");
            username.clear();
        }
    }

    loop {
        println!("Enter password:");
        io::stdin().read_line(&mut password).expect("Failed to read line");

        if validate_password(&password) {
            break;
        } else {
            println!("Invalid password. Please try again.");
            password.clear();
        }
    }

    // Access the vector file only if the username and password are correct
    if username.trim() == &companies[0].name_of_company[0..4] && password.trim().is_empty() {
        // We are going to create the file containing data
        let mut file = File::create("company_info.txt").expect("Failed to create file");

        for company in &companies {
            // We are going to display company data
            println!("{:?}", company);
            file.write_all(format!("{:?}\n", company).as_bytes())
                .expect("Failed to write to file");

            // Check conditions and perform calculations
            let leverage = company.calculate_leverage();
            if company.number_of_shares > 20_000_000 {
                let multiple_of_leverage = leverage * 2.0;
                file.write_all(format!("Multiple of Leverage: {}\n", multiple_of_leverage).as_bytes())
                    .expect("Failed to write to file");
            }

            if company.number_of_liabilities < 10_000_000 {
                let five_percent_of_leverage = leverage * 0.05;
                file.write_all(format!("5% of Leverage: {}\n", five_percent_of_leverage).as_bytes())
                    .expect("Failed to write to file");
            }
        }
    } else {
        println!("Invalid Username and/or Password. Please retry!");
    }
}

// We are going to validate username based on given criteria
fn validate_username(username: &str) -> bool {
    let username = username.trim();
    username.len() >= 3 && username.len() <= 8 && username.chars().all(|c| c.is_lowercase())
}

// We are going to validate password based on given criteria
fn validate_password(password: &str) -> bool {
    let password = password.trim();
    password.chars().all(|c| c.is_ascii_alphanumeric() && c.is_lowercase())
        && !password.chars().any(|c| "$#@".contains(c))
}
