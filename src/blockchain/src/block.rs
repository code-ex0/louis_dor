use crate::{current_time, Transaction};
use crypto::digest::Digest;
use crypto::sha2::{Sha256};

#[derive(Debug, Clone)]
pub struct Block {
    pub index: u32,
    pub hash: String,
    pub previous_hash: String,
    pub timestamp: u64,
    pub transactions: Vec<Transaction>,
    pub nonce: u64,
}

impl Block {
    pub fn new(
        index: u32,
        previous_hash: String,
        timestamp: u64,
        transactions: Vec<Transaction>,
        nonce: u64,
    ) -> Block {
        Block {
            index,
            hash: String::new(),
            previous_hash,
            timestamp,
            transactions,
            nonce,
        }
    }

    pub fn hash(&mut self) -> String {
        let mut sha = Sha256::new();
        sha.input_str(&self.index.to_string());
        sha.input_str(&self.previous_hash);
        sha.input_str(&self.timestamp.to_string());
        sha.input_str(format!("{:?}", &self.transactions).to_string().as_str());
        sha.input_str(&self.nonce.to_string());
        self.hash = sha.result_str();
        self.hash.clone()
    }

    pub fn new_genesis_block() -> Block {
        let transactions = vec![Transaction::new_genesis_transaction()];
        let mut block = Block::new(
            0,
            String::new(),
            current_time(),
            transactions,
            0,
        );
        block.mine_block(1);
        block
    }

    pub fn mine_block(&mut self, difficulty: u64) {
        let mut nonce = 0;
        let difficulty_string = String::from_utf8(vec![b'0'; difficulty as usize]).unwrap();
        loop {
            self.hash();
            if self.hash.starts_with(&difficulty_string) {
                break;
            }
            nonce += 1;
            self.nonce = nonce;
        }
    }
}