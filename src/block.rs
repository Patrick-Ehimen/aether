use crate::ProofOfWork;
use crate::Transaction;

extern crate bincode;
use serde::{Deserialize, Serialize};
use sled::IVec;

/// Represents a block in the blockchain, containing metadata and a list of transactions.
///
/// # Fields
/// - `timestamp`: The time at which the block was created (Unix timestamp).
/// - `pre_block_hash`: The hash of the previous block in the chain.
/// - `hash`: The hash of this block.
/// - `transactions`: The list of transactions included in this block.
/// - `nonce`: The nonce value found by the proof-of-work algorithm.
/// - `height`: The position of the block in the blockchain.
#[derive(Clone, Serialize, Deserialize)]
pub struct Block {
    timestamp: i64,
    pre_block_hash: String,
    hash: String,
    transactions: Vec<Transaction>,
    nonce: i64,
    height: usize,
}

/// Implements methods for the `Block` struct, providing functionality for block creation,
/// serialization, deserialization, transaction hashing, and accessors.
impl Block {
    /// - `new_block`: Creates a new block with the given previous block hash, transactions, and height.
    ///   Runs the proof-of-work algorithm to compute the block's nonce and hash.
    ///   * `pre_block_hash` - The hash of the previous block.
    ///   * `transactions` - A slice of transactions to include in the block.
    ///   * `height` - The height of the block in the blockchain.
    ///   * Returns: A new `Block` instance.
    pub fn new_block(pre_block_hash: String, transactions: &[Transaction], height: usize) -> Block {
        let mut block = Block {
            timestamp: crate::current_timestamp(),
            pre_block_hash,
            hash: String::new(),
            transactions: transactions.to_vec(),
            nonce: 0,
            height,
        };

        let pow = ProofOfWork::new_proof_of_work(block.clone());
        let (nonce, hash) = pow.run();
        block.nonce = nonce;
        block.hash = hash;
        return block;
    }

    /// - `deserialize`: Deserializes a block from a byte slice using `bincode`.
    ///   * `bytes` - The byte slice to deserialize.
    ///   * Returns: The deserialized `Block`.
    pub fn deserialize(bytes: &[u8]) -> Block {
        bincode::deserialize(bytes).unwrap()
    }

    /// - `serialize`: Serializes the block into a vector of bytes using `bincode`.
    ///   * Returns: A `Vec<u8>` containing the serialized block.
    pub fn serialize(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap().to_vec()
    }

    /// - `generate_genesis_block`: Generates the genesis (first) block with a single transaction.
    ///   * `transaction` - The transaction to include in the genesis block.
    ///   * Returns: The genesis `Block`.
    pub fn generate_genesis_block(transaction: &Transaction) -> Block {
        let transactions = vec![transaction.clone()];
        return Block::new_block(String::from("None"), &transactions, 0);
    }

    /// - `hash_transactions`: Computes the SHA-256 hash of all transactions in the block.
    ///   * Returns: A `Vec<u8>` containing the hash of the transactions.
    pub fn hash_transactions(&self) -> Vec<u8> {
        let mut txhashs = vec![];
        for transaction in &self.transactions {
            txhashs.extend(transaction.get_id());
        }
        crate::sha256_digest(txhashs.as_slice())
    }

    /// - `get_transactions`: Returns a slice of the transactions contained in the block.
    ///   * Returns: A slice of `Transaction`.
    pub fn get_transactions(&self) -> &[Transaction] {
        self.transactions.as_slice()
    }

    /// - `get_pre_block_hash`: Returns a clone of the previous block's hash.
    ///   * Returns: A `String` containing the previous block hash.
    pub fn get_pre_block_hash(&self) -> String {
        self.pre_block_hash.clone()
    }

    /// - `get_hash`: Returns a reference to the block's hash as a string slice.
    ///   * Returns: A `&str` representing the block hash.
    pub fn get_hash(&self) -> &str {
        self.hash.as_str()
    }

    /// - `get_hash_bytes`: Returns the block's hash as a vector of bytes.
    ///   * Returns: A `Vec<u8>` containing the hash bytes.
    pub fn get_hash_bytes(&self) -> Vec<u8> {
        self.hash.as_bytes().to_vec()
    }

    /// - `get_timestamp`: Returns the timestamp of the block.
    ///   * Returns: An `i64` representing the block's timestamp.
    pub fn get_timestamp(&self) -> i64 {
        self.timestamp
    }

    /// - `get_height`: Returns the height of the block in the blockchain.
    ///   * Returns: A `usize` representing the block's height.
    pub fn get_height(&self) -> usize {
        self.height
    }
}

/// @notice Converts a `Block` into an `IVec` by serializing the block using `bincode`.
/// @dev This implementation uses `bincode::serialize` to convert the `Block` into a byte vector,
///      then constructs an `IVec` from those bytes. Panics if serialization fails.
/// @param b The `Block` instance to be converted.
/// @return An `IVec` containing the serialized bytes of the `Block`.
impl From<Block> for IVec {
    fn from(b: Block) -> Self {
        let bytes = bincode::serialize(&b).unwrap();
        Self::from(bytes)
    }
}
