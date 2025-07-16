// use aether::block::Block;
// use aether::proof_of_work::ProofOfWork;
// use aether::transactions::Transaction;

// #[test]
// fn test_new_block() {
//     let transaction = Transaction::new_coinbase_tx("Alice".to_string(), "".to_string());
//     let transactions = vec![transaction];
//     let block = Block::new_block("0".to_string(), &transactions, 0);

//     assert_eq!(block.get_pre_block_hash(), "0");
//     assert_eq!(block.get_transactions().len(), 1);
//     assert_eq!(block.get_height(), 0);
// }

// #[test]
// fn test_genesis_block() {
//     let transaction = Transaction::new_coinbase_tx("Alice".to_string(), "".to_string());
//     let genesis_block = Block::generate_genesis_block(&transaction);

//     assert_eq!(genesis_block.get_pre_block_hash(), "None");
//     assert_eq!(genesis_block.get_transactions().len(), 1);
//     assert_eq!(genesis_block.get_height(), 0);
// }

// #[test]
// fn test_block_serialization() {
//     let transaction = Transaction::new_coinbase_tx("Alice".to_string(), "".to_string());
//     let transactions = vec![transaction];
//     let block = Block::new_block("0".to_string(), &transactions, 0);
//     let serialized_block = block.serialize();
//     let deserialized_block = Block::deserialize(&serialized_block);

//     assert_eq!(block.get_hash(), deserialized_block.get_hash());
// }

// #[test]
// fn test_proof_of_work() {
//     let transaction = Transaction::new_coinbase_tx("Alice".to_string(), "".to_string());
//     let transactions = vec![transaction];
//     let block = Block::new_block("0".to_string(), &transactions, 0);
//     let pow = ProofOfWork::new_proof_of_work(block.clone());
//     let (nonce, hash) = pow.run();

//     assert!(nonce > 0);
//     assert!(!hash.is_empty());
// }
