use crate::{Block, current_time, Transaction};
#[derive(Debug, Clone)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub temp_transactions: Vec<Transaction>
}

impl Blockchain {

    pub fn new() -> Blockchain {
        Blockchain {
            chain: Vec::new(),
            temp_transactions: Vec::new(),
        }
    }

    pub fn new_block(&self, proof: u64, previous_hash: String) -> Block {
        let last_block = self.chain.last().unwrap();
        let block = Block::new(last_block.index + 1, previous_hash, current_time(), self.temp_transactions.clone(), 0);
        block
    }

    pub fn add_block(&mut self, block: Block) {
        self.chain.push(block);
    }

    pub fn genesis_block(&mut self) {
        let block = Block::new_genesis_block();
        self.chain.push(block);
    }

    pub fn new_transaction(&mut self, transaction: Transaction) {
        if let Some(sender) = &transaction.sender {
            let sender_balance = self.calculate_balance(sender.clone());
            let sender_temp_balance = self.calculate_temps_balance(sender.clone());
            if sender_balance < transaction.amount + sender_temp_balance {
                println!("Sender does not have enough balance to send transaction");
                return;
            }
        }
        if transaction.verify() {
            self.temp_transactions.push(transaction);
        }
    }

    pub fn last_block(&self) -> &Block {
        self.chain.last().unwrap()
    }

    pub fn mine_block(&mut self, difficulty: u64, _address: String, _reward: u64) {
        let mut block = Block::new(
            self.last_block().index + 1,
            self.last_block().hash.clone(),
            current_time(),
            self.temp_transactions.clone(),
            0,
        );
        block.mine_block(difficulty);
        self.chain.push(block);
        self.temp_transactions = Vec::new();
    }

    pub fn calculate_balance(&self, address: String) -> u64 {
        let mut balance = 0;
        for block in self.chain.iter() {
            for transaction in block.transactions.iter() {
                if transaction.sender == Some(address.clone()) {
                    balance -= transaction.amount;
                }
                if transaction.receiver == Some(address.clone()) {
                    balance += transaction.amount;
                }
            }
        }
        balance
    }

    pub fn calculate_temps_balance(&self, address: String) -> u64 {
        let mut balance = 0;
        for transaction in self.temp_transactions.iter() {
            if transaction.sender == Some(address.clone()) {
                balance += transaction.amount;
            }
        }
        balance
    }

    pub fn get_wallets(&self) -> Vec<String> {
        let mut wallets = Vec::new();
        for block in self.chain.iter() {
            for transaction in block.transactions.iter() {
                if let Some(sender) = transaction.sender.clone() {
                    if !wallets.contains(&sender) {
                        wallets.push(sender);
                    }
                }
                if let Some(receiver) = transaction.receiver.clone() {
                    if !wallets.contains(&receiver) {
                        wallets.push(receiver);
                    }
                }
            }
        }
        wallets
    }
}