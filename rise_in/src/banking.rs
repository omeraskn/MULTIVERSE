/*  
In this task, students will create a basic banking system using Traits in Rust. 
The program will allow users to create accounts, deposit and withdraw money, and view their account balance.
*/

 trait Account {
    fn deposit(&mut self, amount: f64) -> Result<(), String>;
    fn withdraw(&mut self, amount: f64)  -> Result<(), String>;
    fn balance(&self) -> f64;
}

struct BankAccount {
    account_number: u32,
    holder_name: String,
    balance: f64
}

impl Account for BankAccount {
    fn deposit(&mut self, amount: f64) -> Result<(), String> {
        if amount <= 0.0 {
            return  Err("Deposit amount must be greater than zero.".to_string());
        }
        self.balance += amount;
        println!(
            "Deposited ${:.2} into account {}. New balance: ${:.2}",
            amount, self.account_number, self.balance
        );
        Ok(())
    }

    fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        if amount <= 0.0 {
            return Err("Withdrawal amount must be greater than zero.".to_string());
        }
        if amount > self.balance {
            return Err("Insufficient balance in account.".to_string());

        } else {
            self.balance -= amount;
            println!(
                "Withdrew ${:.2} from account {}. New balance: ${:.2}",
                amount, self.account_number, self.balance
            );
        }
        Ok(())
    }

    fn balance(&self) -> f64 {
        self.balance
    }
}

pub fn banking() {
    // Create two BankAccount instances
    let mut account1 = BankAccount {
        account_number: 101,
        holder_name: "Alice".to_string(),
        balance: 1000.0,
    };

    let mut account2 = BankAccount {
        account_number: 102,
        holder_name: "Bob".to_string(),
        balance: 500.0,
    };

    match account1.deposit(75.0) {
        Ok(_) => println!("Deposit successful."),
        Err(e) => println!("Error during deposit: {}", e),
    }

    // Withdraw from account2
    match account2.withdraw(25.0) {
        Ok(_) => println!("Withdrawal successful."),
        Err(e) => println!("Error during withdrawal: {}", e),
    }

    // Check and print balances
    println!(
        "Balance of account {} ({}): ${:.2}",
        account1.account_number,
        account1.holder_name,
        account1.balance()
    );
    println!(
        "Balance of account {} ({}): ${:.2}",
        account2.account_number,
        account2.holder_name,
        account2.balance()
    );
}