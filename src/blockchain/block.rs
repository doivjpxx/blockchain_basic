use serde::{Deserialize, Serialize};
use time::SystemTime;

#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,
    pub timestamp: u64,
    pub transactions: Vec<Transaction>,
    pub proof: u64,
    pub previous_hash: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    sender: String,
    recipient: String,
    amount: f32,
}

impl Block {
    pub fn new(
        index: u64,
        transactions: Vec<Transaction>,
        proof: u64,
        previous_hash: String,
    ) -> Block {
        Block {
            index,
            timestamp: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            transactions,
            proof,
            previous_hash,
        }
    }
}
