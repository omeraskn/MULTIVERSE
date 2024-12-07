use std::io::{self, Write};

mod ownership;
mod calculator;
mod banking;
mod filtering;
mod repository;


fn main() {
    loop {
        println!("\n==== Welcome to Rust Utility Program ====");
        println!("Choose an option:");
        println!("1. Ownership Concatenater");
        println!("2. Calculator");
        println!("3. Banking Operations");
        println!("4. Filtering Games");
        println!("5. Exit");

        print!("Enter your choice (1-5): ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                println!("\nRunning Ownership Concatenater...");
                ownership::concatenater();
            }
            "2" => {
                println!("\nRunning Calculator...");
                calculator::calculator();
            }
            "3" => {
                println!("\nRunning Banking Operations...");
                banking::banking();
            }
            "4" => {
                println!("\nRunning Filtering Games...");
                filtering::main();
            }
            "5" => {
                println!("\nExiting the program. Goodbye!");
                break;
            }
            _ => {
                println!("\nInvalid choice. Please try again.");
            }
        }
    }
}
