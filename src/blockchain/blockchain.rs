use crate::blockchain::{Block, Transaction};
use crate::current_time;

#[derive(Debug, Clone)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub temp_transactions: Vec<Transaction>,
}

impl Blockchain {

    pub fn new() -> Blockchain {
        Blockchain {
            chain: Vec::new(),
            temp_transactions: Vec::new(),
        }
    }
    pub fn genesis_block(&mut self) {
        let mut block = Block::new_genesis_block();
        self.chain.push(block);
    }

    pub fn new_transaction(&mut self, transaction: Transaction) {
        if transaction.verify() {
            self.temp_transactions.push(transaction);
        }
    }

    pub fn last_block(&self) -> &Block {
        self.chain.last().unwrap()
    }

    pub fn mine_block(&mut self, difficulty: u64) {
        let mut block = Block::new(
            self.last_block().hash.clone(),
            current_time(),
            self.temp_transactions.clone(),
            0,
        );
        block.mine_block(difficulty);
        self.chain.push(block);
        self.temp_transactions = Vec::new();
    }
}