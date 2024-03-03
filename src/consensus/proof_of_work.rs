use sha2::{Digest, Sha256};

pub struct ProofOfWork;

impl ProofOfWork {
    pub fn new() -> ProofOfWork {
        ProofOfWork
    }

    pub fn solve_proof(last_proof: u64) -> u64 {
        let mut proof = 0;
        while !Self::validate_proof(last_proof, proof) {
            proof += 1;
        }

        proof
    }

    pub fn validate_proof(last_proof: u64, current_proof: u64) -> bool {
        let guess = format!("{}{}", last_proof, current_proof);
        let guess_hash = format!("{:x}", Sha256::digest(guess.as_bytes()));

        // verify if the first 4 characters of the hash are equal to 0000
        guess_hash.starts_with("0000")
    }
}
