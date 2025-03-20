// bank app features:

// create account
// balance
// withdraw balance
// deposit amount
// security
// account history (Track transactions)
// Implement account transferred

use std::time::Duration;


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
    // pin: u16,
    // transaction_history: Vec<Transactions>
}

trait BankTrait {
    fn create_account(account_name:  String, account_number: u32, balance:i32) -> Self;
    fn deposit(&mut self, amount: i32);
    fn withdraw(&mut self, amount: i32) -> Result<(), &'static str>;
    fn balance(self) -> i32;
    fn account_details(self)-> (u32, String);
    fn transfer(&mut self, amount: i32, recipient: &mut Bank) -> Result<(), &'static str>;

}

impl BankTrait for Bank {

    fn create_account(account_name: String, account_number: u32, balance: i32)-> Self{
        Bank {
            account_number,
            account_name,
            balance: balance
        }
    }

    fn deposit(&mut self, amount: i32){
        self.balance += amount
    }

    fn withdraw(&mut self, amount: i32) -> Result<(), &'static str>{
        if amount > self.balance {
            Err("Insufficient Funds")
        }else {
            Ok(self.balance -= amount)
        }
    }

    fn balance(self) -> i32{
        self.balance
    }

    fn account_details(self)-> (u32, String){
        todo!()
    }

    fn transfer(&mut self, amount: i32, recipient: &mut Bank) -> Result<(), &'static str> {
        match self.withdraw(amount) {
            Ok(()) => {
                recipient.deposit(amount);
                Ok(())
            }
            Err(err)=> Err(err),
        }
    }

}

fn main() {
    let mut yvan_account = Bank::create_account(String::from("yvan"), 1234, 6000);

    let yvan_clone = yvan_account.clone();
    let mut alice = Bank{balance: 600, account_name: String::from("Alice"), account_number:5454};
    let mut bob = Bank{balance: 360, account_name: String::from("Bob"), account_number: 7887};


    let juan_account = Bank::create_account(String::from("Juan"), 4321, 300);

    println!("New account created details is {:#?}", yvan_account);

    yvan_account.deposit(200);

    println!("Yvan's deposited!");
    println!("Yvan's new balance is ${}", yvan_account.balance());
    println!("");

    // yvan_account.transfer(2300, 1234)


}


