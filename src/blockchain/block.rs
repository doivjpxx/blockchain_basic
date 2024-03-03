use crate::blockchain::transaction::Transaction;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,
    pub timestamp: u64,
    pub transactions: Vec<Transaction>,
    pub proof: u64,
    pub previous_hash: String,
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
            timestamp: current_timestamp(),
            transactions,
            proof,
            previous_hash,
        }
    }

    pub fn calculate_hash(&self) -> String {
        let record = format!(
            "{}{}{:?}{}{}",
            &self.index,
            &self.timestamp,
            &self.transactions, // Đảm bảo rằng Transaction struct có thể được serialized
            &self.proof,
            &self.previous_hash,
        );
        let mut hasher = Sha256::new();
        hasher.update(record);
        format!("{:x}", hasher.finalize())
    }
}

fn current_timestamp() -> u64 {
    use chrono::Utc;
    Utc::now().timestamp_millis() as u64
}
