use crate::blockchain::block::Block;
use crate::blockchain::transaction::Transaction;

pub struct BlockChain {
    pub chain: Vec<Block>,
}

impl BlockChain {
    pub fn new() -> BlockChain {
        BlockChain {
            chain: vec![Block::new(0, vec![], 0, String::from("0"))],
        }
    }

    pub fn add_block(&mut self, transactions: Vec<Transaction>, proof: u64) {
        let previous_hash = self.chain.last().unwrap().calculate_hash();
        let block = Block::new(self.chain.len() as u64, transactions, proof, previous_hash);

        self.chain.push(block);
    }
}
