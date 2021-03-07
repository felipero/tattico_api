use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

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

#[derive(Hash, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct Entry {
    pub value: Decimal,
    pub description: Option<String>,
    pub date: DateTime<Utc>,
    pub category: String,
    pub account_id: String,
}
