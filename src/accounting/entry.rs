use rust_decimal::Decimal;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Hash, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct Entry {
    pub value: Decimal,
    pub description: Option<String>,
    pub date: DateTime<Utc>,
    pub category: String,
    pub account_id: String,
}
