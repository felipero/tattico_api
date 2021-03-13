use serde::{Deserialize, Serialize};
use crate::accounting::Entry;

#[derive(Hash, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum AccountType {
    CreditCard,
    Checking,
    Savings,
    Investment,
}

#[derive(Hash, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct Account {
    pub id: String,
    pub name: String,
    pub kind: AccountType,
    pub categories: Vec<Category>,
}

#[derive(Hash, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct Category {
    pub name: String,
    pub entries: Vec<Entry>,
}