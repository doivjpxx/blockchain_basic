use crate::blockchain::block::Block;
use crate::blockchain::transaction::Transaction;
use crate::consensus::ProofOfWork;

pub struct BlockChain {
    pub chain: Vec<Block>,
}

impl BlockChain {
    pub fn new() -> BlockChain {
        let genesis_block = Block::new(0, vec![], 0, String::from("0"));
        BlockChain {
            chain: vec![genesis_block],
        }
    }

    pub fn add_block(&mut self, transactions: Vec<Transaction>, proof: u64) {
        let last_block = self.chain.last().unwrap();
        let last_proof = last_block.proof;
        let valid_proof = ProofOfWork::solve_proof(last_proof);

        if valid_proof != proof {
            panic!("Invalid proof");
        }

        let previous_hash = last_block.calculate_hash();
        let new_index = last_block.index + 1;
        let new_block = Block::new(new_index, transactions, proof, previous_hash);

        self.chain.push(new_block);
    }

    // Mine a new block with the given transactions
    pub fn mine(&mut self, transactions: Vec<Transaction>) {
        // Get the last block to get the last proof and hash
        let last_block = self
            .chain
            .last()
            .expect("The chain should have at least one block.");
        let last_proof = last_block.proof;

        // You would have a ProofOfWork instance to utilize, adjust as your implementation details

        // Execute proof of work to find the new proof
        let proof = ProofOfWork::solve_proof(last_proof);

        // We must ensure that the proof is correct by calling a method to validate it.
        // This part might be handled within the `solve_proof` itself, or you might call another method.

        // In case there's a need to validate, you could uncomment the following lines:
        // if !proof_of_work.validate_proof(last_proof, proof) {
        //     panic!("Failed to mine the block: Invalid proof of work.");
        // }

        let previous_hash = last_block.calculate_hash();
        let new_block = Block::new(
            self.chain.len() as u64, // index of the new block is length of the chain
            transactions,
            proof,
            previous_hash,
        );

        // Push the new, valid block to the chain
        self.chain.push(new_block);
    }
}
