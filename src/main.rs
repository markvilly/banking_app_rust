// bank app features:

// create account
// balance
// withdraw balance
// deposit amount
// security
// account history (Track transactions)
// Implement account transferred

use std::io;
use std::{pin, time::Duration};

// struct Transactions {
//     account_number_to: u32,
//     account_number_from: u32,
//     amount_sent: u32,
//     date: Duration
// }

// enum AccountType {
//     SavingsAccount,
//     CurrentAccount,
//     SalaryAccount,
//     PersonalAccount
// }

#[derive(Debug, Clone, PartialEq, Eq)]
struct Bank {
    account_number: u32,
    account_name: String,
    balance: i32,
    pin: String,
    // transaction_history: Vec<Transactions>
}

trait BankTrait {
    fn create_account(account_name: String, account_number: u32, balance: i32, pin: String)
    -> Self;
    fn deposit(&mut self, amount: i32);
    fn withdraw(&mut self, amount: i32) -> Result<(), &'static str>;
    fn balance(&self) -> i32;
    fn account_details(self) -> (u32, String);
    fn transfer(
        &mut self,
        amount: i32,
        recipient: &mut Bank,
        
    ) -> Result<(), &'static str>;
}

impl BankTrait for Bank {
    fn create_account(
        account_name: String,
        account_number: u32,
        balance: i32,
        pin: String,
    ) -> Self {
        Bank {
            account_number,
            account_name,
            balance: balance,
            pin: pin,
        }
    }

    fn deposit(&mut self, amount: i32) {
        self.balance += amount
    }

    fn withdraw(&mut self, amount: i32) -> Result<(), &'static str> {
        if amount > self.balance {
            Err("Insufficient Funds")
        } else {
            Ok(self.balance -= amount)
        }
    }

    fn balance(&self) -> i32 {
        self.balance
    }

    fn account_details(self) -> (u32, String) {
        todo!()
    }

    fn transfer(
        &mut self,
        amount: i32,
        recipient: &mut Bank,
    ) -> Result<(), &'static str> {
        let mut pin = String::new();

        io::stdin()
            .read_line(&mut pin)
            .expect("Failed to read line");

        println!(
            "You are about to transfer {} to {}",
            amount, recipient.account_name
        );
        println!("Enter your pin: ");

        match &pin.trim().parse::<u16>() {
            Ok(num) => println!("You have entered: {}", num),
            Err(_) => println!("Invalid input! add numbers pls!"),
        }

        if pin != self.pin {
            return Err("Invalid Pint, try again");
        } else {
            match self.withdraw(amount) {
                Ok(()) => {
                    recipient.deposit(amount);
                    Ok(())
                }
                Err(err) => Err(err),
            }
        }
    }
}

fn main() {
    let mut yvan_account =
        &mut Bank::create_account(String::from("yvan"), 1234, 6000, String::from("999"));

    let yvan_clone = yvan_account.clone();
    let mut alice = Bank {
        balance: 600,
        account_name: String::from("Alice"),
        account_number: 5454,
        pin: String::from("6996"),
    };
    let mut bob = Bank {
        balance: 360,
        account_name: String::from("Bob"),
        pin: String::from("1212"),
        account_number: 7887,
    };

    let mut juan_account =
        Bank::create_account(String::from("Juan"), 4321, 300, String::from("9999"));

    println!("New account created details is {:#?}", yvan_account);

    yvan_account.deposit(200);

    println!("Yvan's deposited!");
    println!("Yvan's new balance is ${}", yvan_account.balance());
    // println!("");

    // input pin variable:

    match yvan_account.transfer(3000, &mut juan_account) {
        Ok(()) => println!("Transfer successful"),
        Err(err) => println!("Transfer failed: {}", err),
    }

    println!("Juan's balance is {}", juan_account.balance());

    println!("Yvan's balance is {}", yvan_account.balance());
}
