use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    sender: String,
    recipient: String,
    amount: f32,
}

impl Transaction {
    pub fn new(sender: String, recipient: String, amount: f32) -> Transaction {
        Transaction {
            sender,
            recipient,
            amount,
        }
    }
}
