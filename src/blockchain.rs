use crate::Transaction;
use crate::block::Block;
// use crate::transaction::TXOutput;

use sled::{Db, Tree};
// use std::collections::HashMap;
use sled::transaction::TransactionResult;
use std::env::current_dir;
use std::sync::{Arc, RwLock};

const TIP_BLOCK_HASH_KEY: &str = "tip_block_hash";
const BLOCKS_TREE: &str = "blocks";

#[derive(Clone)]
pub struct Blockchain {
    tip_hash: Arc<RwLock<String>>, // hash of last block
    db: Db,
}

impl Blockchain {
    pub fn create_blockchain(genesis_address: &str) -> Blockchain {
        let db = sled::open(current_dir().unwrap().join("data")).unwrap();
        let blocks_tree = db.open_tree(BLOCKS_TREE).unwrap();

        let data = blocks_tree.get(TIP_BLOCK_HASH_KEY).unwrap();
        let tip_hash;
        if data.is_none() {
            let coinbase_tx = Transaction::new_coinbase_tx(genesis_address);
            let block = Block::generate_genesis_block(&coinbase_tx);
            Self::update_blocks_tree(&blocks_tree, &block);
            tip_hash = String::from(block.get_hash());
        } else {
            tip_hash = String::from_utf8(data.unwrap().to_vec()).unwrap();
        }
        Blockchain {
            tip_hash: Arc::new(RwLock::new(tip_hash)),
            db,
        }
    }

    fn update_blocks_tree(blocks_tree: &Tree, block: &Block) {
        let block_hash = block.get_hash();
        let _: TransactionResult<(), ()> = blocks_tree.transaction(|tx_db| {
            let _ = tx_db.insert(block_hash, block.clone());
            let _ = tx_db.insert(TIP_BLOCK_HASH_KEY, block_hash);
            Ok(())
        });
    }

    pub fn new_blockchain() -> Blockchain {
        let db = sled::open(current_dir().unwrap().join("data")).unwrap();
        let blocks_tree = db.open_tree(BLOCKS_TREE).unwrap();
        let tip_bytes = blocks_tree
            .get(TIP_BLOCK_HASH_KEY)
            .unwrap()
            .expect("No existing blockchain found. Create one first.");
        let tip_hash = String::from_utf8(tip_bytes.to_vec()).unwrap();
        Blockchain {
            tip_hash: Arc::new(RwLock::new(tip_hash)),
            db,
        }
    }

    pub fn get_db(&self) -> &Db {
        &self.db
    }

    pub fn get_tip_hash(&self) -> String {
        self.tip_hash.read().unwrap().clone()
    }

    pub fn set_tip_hash(&self, new_tip_hash: &str) {
        let mut tip_hash = self.tip_hash.write().unwrap();
        *tip_hash = String::from(new_tip_hash)
    }

    pub fn mine_block(&self, transactions: &[Transaction]) -> Block {
        for trasaction in transactions {
            if trasaction.verify(self) == false {
                panic!("ERROR: Invalid transaction")
            }
        }
        let best_height = self.get_best_height();

        let block = Block::new_block(self.get_tip_hash(), transactions, best_height + 1);
        let block_hash = block.get_hash();

        let blocks_tree = self.db.open_tree(BLOCKS_TREE).unwrap();
        Self::update_blocks_tree(&blocks_tree, &block);
        self.set_tip_hash(block_hash);
        block
    }

    pub fn get_best_height(&self) -> usize {
        let block_tree = self.db.open_tree(BLOCKS_TREE).unwrap();
        let tip_block_bytes = block_tree
            .get(self.get_tip_hash())
            .unwrap()
            .expect("The tip hash is valid");
        let tip_block = Block::deserialize(tip_block_bytes.as_ref());
        tip_block.get_height()
    }

    pub fn get_block(&self, block_hash: &[u8]) -> Option<Block> {
        let block_tree = self.db.open_tree(BLOCKS_TREE).unwrap();
        if let Some(block_bytes) = block_tree.get(block_hash).unwrap() {
            let block = Block::deserialize(block_bytes.as_ref());
            return Some(block);
        }
        return None;
    }

    pub fn get_block_hashes(&self) -> Vec<Vec<u8>> {
        let mut iterator = self.iterator();
        let mut blocks = vec![];
        loop {
            let option = iterator.next();
            if option.is_none() {
                break;
            }
            let block = option.unwrap();
            blocks.push(block.get_hash_bytes());
        }
        return blocks;
    }

    pub fn iterator(&self) -> BlockchainIterator {
        BlockchainIterator::new(self.get_tip_hash(), self.db.clone())
    }

    pub fn add_block(&self, block: &Block) {
        let block_tree = self.db.open_tree(BLOCKS_TREE).unwrap();
        if let Some(_) = block_tree.get(block.get_hash()).unwrap() {
            return;
        }
        let _: TransactionResult<(), ()> = block_tree.transaction(|tx_db| {
            let _ = tx_db.insert(block.get_hash(), block.serialize()).unwrap();

            let tip_block_bytes = tx_db
                .get(self.get_tip_hash())
                .unwrap()
                .expect("The tip hash is not valid");
            let tip_block = Block::deserialize(tip_block_bytes.as_ref());
            if block.get_height() > tip_block.get_height() {
                let _ = tx_db.insert(TIP_BLOCK_HASH_KEY, block.get_hash()).unwrap();
                self.set_tip_hash(block.get_hash());
            }
            Ok(())
        });
    }
}

pub struct BlockchainIterator {
    db: Db,
    current_hash: String,
}

impl BlockchainIterator {
    fn new(tip_hash: String, db: Db) -> BlockchainIterator {
        BlockchainIterator {
            current_hash: tip_hash,
            db,
        }
    }

    pub fn next(&mut self) -> Option<Block> {
        let block_tree = self.db.open_tree(BLOCKS_TREE).unwrap();
        let data = block_tree.get(self.current_hash.clone()).unwrap();
        if data.is_none() {
            return None;
        }
        let block = Block::deserialize(data.unwrap().to_vec().as_slice());
        self.current_hash = block.get_pre_block_hash().clone();
        return Some(block);
    }
}
