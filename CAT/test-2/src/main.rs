use std::fs::File;
use std::io::{self, Write};

struct Company {
    name: String,
    year: i32,
    shares: f64,
    liabilities: f64,
}

impl Company {
    fn lev_percent(&self) -> (f64, f64, f64) {
        let leverage = (self.shares - self.liabilities) / self.shares;
        let percentage_leverage = leverage * 100.0;
        let multiple_lev = percentage_leverage * leverage;

        (leverage, percentage_leverage, multiple_lev)
    }
}

fn login(company_name: &str) -> bool {
    let username = &company_name[..4];

    println!("Enter username:");
    let mut user = String::new();
    io::stdin()
    .read_line(&mut user)
    .expect("Failed to read line");

    println!("Enter password:");
    let mut password = String::new();
    io::stdin()
    .read_line(&mut password)
    .expect("Failed to read line");

    let name = user.trim().len() >= 4 && user.trim().len() <= 8 && user.trim() == username;
    let pass = password.trim().chars().all(|pw| pw.is_ascii_lowercase() || pw.is_numeric()) && !password.contains(|pw: char| pw == '$' || pw == '#' || pw == '@');

    name && pass
}

fn five_percent(percentage_leverage: f64, liabilities: f64) -> f64 {
    if liabilities < 10_000_000.0 {
        percentage_leverage * 0.05
    } 
    else {
        0.0
    }
}

fn main() {
    println!("Enter company:");
    let mut input = String::new();
    io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line");

    let company_name = input.trim();

    if login(company_name) {
        let companies = vec![
            Company {
                name: String::from("Cadbury Nigeria Plc."),
                year: 1965,
                shares: 15_000_000.0,
                liabilities: 5_500_000.0,
            },
            Company {
                name: String::from("Champion Breweries Plc."),
                year: 1974,
                shares: 25_000_000.0,
                liabilities: 8_000_000.0,
            },
            Company {
                name: String::from("Dangote Sugar Refinery Plc."),
                year: 1970,
                shares: 18_000_000.0,
                liabilities: 10_000_000.0,
            },
            Company {
                name: String::from("Flour Mills Nigeria Plc."),
                year: 1960,
                shares: 32_000_000.0,
                liabilities: 4_000_000.0,
            },
            Company {
                name: String::from("Nestle Nigeria Plc."),
                year: 1961,
                shares: 8_000_000.0,
                liabilities: 1_500_000.0,
            },
            Company {
                name: String::from("Unilever Nigeria Plc."),
                year: 1923,
                shares: 37_000_000.0,
                liabilities: 11_000_000.0,
            },
            Company {
                name: String::from("Honeywell Nigeria Plc."),
                year: 1906,
                shares: 34_000_000.0,
                liabilities: 9_000_000.0,
            },
            Company {
                name: String::from("Nigeria Breweries Plc."),
                year: 1946,
                shares: 30_000_000.0,
                liabilities: 12_000_000.0,
            },
            
        ];

        let mut file = File::create("data.txt").expect("Unable to create file");

        file.write_all(format!("{:<50}\n", "                                                  NIGERIAN MARKET\n")
        .as_bytes())
        .expect("Error writing to file");

        file.write_all(format!("| {:<28} | {:<12} | {:<10} | {:<10} | {:<8} | {:<10} | {:<9} | {:<7} |\n", "Name", "Year Founded", "Shares", "Liabilities", "Leverage", "% Leverage", "P. L. M", "5% of %Lev")
        .as_bytes())
        .expect("Unable to write to file");

        for company in &companies {
            if company.name == company_name {
                let (leverage, percentage_leverage, multiple_lev) = company.lev_percent();
                let fp = five_percent(percentage_leverage, company.liabilities);

                file.write_all(format!("| {:<28} | {:<12} | {:<10} | {:<11} | {:.6} | {:.7} | {:.6} | {:.8} |\n", company.name, company.year, company.shares, company.liabilities, leverage, percentage_leverage, multiple_lev, fp)
                .as_bytes())
                .expect("Unable to write to file");

                if company.shares > 20_000_000.0{
                    let mut multi = File::create("multiple.txt").expect("Unable to create file");

                    multi.write_all(format!("{:<50}\n", "MULTIPLE OF PERCENTAGE LEVERAGE  WITH SHARES GREATER THAN 20,000,000.00\n")
                    .as_bytes())
                    .expect("Error writing to file");

                    multi.write_all(format!("{:<28} - {:.6}\n", company.name, multiple_lev)
                    .as_bytes())
                    .expect("Error writing to file");
                }

                println!("Data has been pushed to the .txt file.");
                return;
            }
        }

        println!("Oops! The company you entered is not on the list.");
    } 
    else {
        println!("No cheats... You have no access.");
    }
}