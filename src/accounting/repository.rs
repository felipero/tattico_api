use std::fs::File;
use std::io::BufReader;

use crate::accounting::Account;

pub trait Repository<T> {
    fn find(&self, id: String) -> &T;
    fn find_all(&self) -> &Vec<T>;
    fn add(&mut self, account: Account) -> bool;
}

pub struct AccountRepository {
    accounts: Vec<Account>,
}

const DATA_FILE: &str = "accounts.json";

impl Repository<Account> for AccountRepository {
    fn find(&self, id: String) -> &Account {
        self.accounts
            .iter()
            .filter(|&a| a.id == id)
            .next()
            .expect("Could not find an account with given id.")
    }

    fn find_all(&self) -> &Vec<Account> {
        &self.accounts
    }

    fn add(&mut self, account: Account) -> bool {
        self.accounts.push(account);
        ::serde_json::to_writer(
            &File::create(DATA_FILE.to_string()).expect("Data file not found"),
            &self.accounts,
        ).expect("Could not write to file");
        return true;
    }
}

impl AccountRepository {
    // pub fn new() -> Self {
    //     AccountRepository { accounts: vec![] }
    // }

    pub fn from_file() -> Self {
        let file = File::open(DATA_FILE.to_string()).expect("eita pega!");
        //     .unwrap_or(
        //     File::create(DATA_FILE.to_string()).expect("Could not create missing data file"),
        // );
        let reader = BufReader::new(file);

        // Read the JSON contents of the file as an instance of `User`.
        let accounts = serde_json::from_reader(reader)
            // .expect("tem que parsear");
            .unwrap_or(vec![]);

        AccountRepository { accounts: accounts }
    }

    // pub fn with_accounts(&mut self, accounts: Vec<Account>) -> &mut Self {
    //     self.accounts = accounts;
    //     self
    // }
}
