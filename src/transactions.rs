// use data_encoding::HEXLOWER;
use crate::Blockchain;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

const SUBSIDY: i32 = 10;

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TXInput {
    txid: Vec<u8>,
    vout: usize,
    signature: Vec<u8>,
    pub_key: Vec<u8>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TXOutput {
    value: i32,
    pub_key_hash: Vec<u8>,
}

impl TXOutput {
    pub fn new(value: i32, address: &str) -> TXOutput {
        let mut output = TXOutput {
            value,
            pub_key_hash: vec![],
        };
        output.lock(address);
        return output;
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Transaction {
    id: Vec<u8>,
    vin: Vec<TXInput>,
    vout: Vec<TXOutput>,
}

impl Transaction {
    pub fn new_coinbase_tx(to: &str) -> Transaction {
        let txout = TXOutput::new(SUBSIDY, to);
        let mut tx_input = TXInput::default();
        tx_input.signature = Uuid::new_v4().as_bytes().to_vec();

        let mut tx = Transaction {
            id: vec![],
            vin: vec![tx_input],
            vout: vec![txout],
        };

        tx.id = tx.hash();
        return tx;
    }

    pub fn verify(&self, blockchain: &Blockchain) -> bool {
        if self.is_coinbase() {
            return true;
        }
        let mut tx_copy = self.trimmed_copy();
        for (idx, vin) in self.vin.iter().enumerate() {
            let prev_tx_option = blockchain.find_transaction(vin.get_txid());
            if prev_tx_option.is_none() {
                panic!("ERROR: Previous transaction is not correct")
            }
            let prev_tx = prev_tx_option.unwrap();
            tx_copy.vin[idx].signature = vec![];
            tx_copy.vin[idx].pub_key = prev_tx.vout[vin.vout].pub_key_hash.clone();
            tx_copy.id = tx_copy.hash();
            tx_copy.vin[idx].pub_key = vec![];

            let verify = crate::ecdsa_p256_sha256_sign_verify(
                vin.pub_key.as_slice(),
                vin.signature.as_slice(),
                tx_copy.get_id(),
            );
            if !verify {
                return false;
            }
        }
        true
    }

    pub fn get_id(&self) -> &[u8] {
        self.id.as_slice()
    }
}
