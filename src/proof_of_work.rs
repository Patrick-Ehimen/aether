use crate::block::Block;
use data_encoding::HEXLOWER;
use num_bigint::{BigInt, Sign};
use std::borrow::Borrow;
use std::ops::ShlAssign;

/// # Proof of Work
///
/// This struct represents the proof-of-work algorithm.
/// It is used to find a hash that satisfies a certain condition (the target).
pub struct ProofOfWork {
    block: Block,
    target: BigInt,
}

/// The number of bits for the target.
/// The smaller the number, the more difficult the proof-of-work.
const TARGET_BITS: i32 = 8;

/// The maximum value for the nonce.
const MAX_NONCE: i64 = i64::MAX;

/// @title ProofOfWork Implementation
/// @notice Provides methods for creating and running a proof-of-work algorithm for a block.
/// @dev This struct and its methods are used to perform proof-of-work mining in a blockchain context.
impl ProofOfWork {
    /// - `new_proof_of_work(block: Block) -> ProofOfWork`
    ///   @notice Creates a new ProofOfWork instance for the given block.
    ///   @param block The block for which to compute the proof-of-work.
    ///   @return A new ProofOfWork instance.
    pub fn new_proof_of_work(block: Block) -> ProofOfWork {
        let mut target = BigInt::from(1);

        target.shl_assign(256 - TARGET_BITS);
        ProofOfWork { block, target }
    }

    /// - `prepare_data(&self, nonce: i64) -> Vec<u8>`
    ///   @notice Prepares the data to be hashed for the proof-of-work algorithm.
    ///   @param nonce The nonce value to include in the data.
    ///   @return A vector of bytes representing the data to be hashed.
    fn prepare_data(&self, nonce: i64) -> Vec<u8> {
        let pre_block_hash = self.block.get_pre_block_hash();
        let transactions_hash = self.block.hash_transactions();
        let timestamp = self.block.get_timestamp();
        let mut data_bytes = vec![];
        data_bytes.extend(pre_block_hash.as_bytes());
        data_bytes.extend(transactions_hash);
        data_bytes.extend(timestamp.to_be_bytes());
        data_bytes.extend(TARGET_BITS.to_be_bytes());
        data_bytes.extend(nonce.to_be_bytes());
        return data_bytes;
    }

    /// - `run(&self) -> (i64, String)`
    ///   @notice Executes the proof-of-work algorithm, searching for a valid nonce.
    ///   @return A tuple containing the valid nonce and the corresponding hash as a hex string.
    pub fn run(&self) -> (i64, String) {
        let mut nonce = 0;
        let mut hash = Vec::new();
        println!("Mining the block");
        while nonce < MAX_NONCE {
            let data = self.prepare_data(nonce);
            hash = crate::sha256_digest(data.as_slice());
            let hash_int = BigInt::from_bytes_be(Sign::Plus, hash.as_slice());

            if hash_int.lt(self.target.borrow()) {
                println!("{}", HEXLOWER.encode(hash.as_slice()));
                break;
            } else {
                nonce += 1;
            }
        }
        println!();
        return (nonce, HEXLOWER.encode(hash.as_slice()));
    }
}
