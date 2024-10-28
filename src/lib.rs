// Imports and struct definition remain the same
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};
use sha3::{Digest, Keccak256};
use hex;

#[derive(Default, BorshSerialize, BorshDeserialize)]
#[near_bindgen]
pub struct PowVerifier {
    counter: u64, // Tracks the current number for hashing
    salt: u64,    // Fixed salt value
}

#[near_bindgen]
impl PowVerifier {
    #[init]
    pub fn new() -> Self {
        Self { counter: 0, salt: 123456 } // Fixed salt value
    }

    // Method to retrieve current counter and salt
    pub fn get_counter_and_salt(&self) -> (u64, u64) {
        (self.counter, self.salt)
    }

    // Combined calculation and proof submission method
    pub fn submit_direct_proof(&mut self, proof_hex: String) -> bool {
        let miner = env::signer_account_id();

        // Generate proof from current counter and salt
        let input = format!("{}{}", self.counter, self.salt);
        let calculated_proof = Keccak256::digest(input.as_bytes()).to_vec();

        // Decode the provided hex proof for comparison
        let provided_proof = match hex::decode(&proof_hex) {
            Ok(decoded) => decoded,
            Err(_) => {
                env::log_str("Failed to decode hex string.");
                return false;
            }
        };

        // Check if the provided proof matches the calculated proof
        if provided_proof == calculated_proof {
            self.counter += 1; // Increment counter if proof is valid
            env::log_str(&format!("Proof validated for miner: {}", miner));
            true
        } else {
            env::log_str("Proof validation failed.");
            false
        }
    }
}
