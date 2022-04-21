use louis_dor::current_time;
use ed25519_dalek::{Keypair, SignatureError, Signer, Verifier};
use ed25519_dalek::PublicKey;
use ed25519_dalek::Signature;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use rustc_serialize::hex::{ToHex, FromHex};
use crate::Wallet;

#[derive(Debug, Clone)]
pub struct Transaction {
    pub sender: Option<String>,
    pub receiver: Option<String>,
    pub amount: u64,
    pub timestamp: u64,
    pub signature: Option<String>,
}

impl Transaction {

    pub fn new(sender: Option<String>, receiver: Option<String>, amount: u64) -> Transaction {
        Transaction {
            sender,
            receiver,
            amount,
            timestamp: current_time(),
            signature: None,
        }
    }

    pub fn bytes (&self) -> Vec<u8> {

        let mut bytes = vec![];
        if let Some(sender) = &self.sender {
            bytes.extend_from_slice(sender.as_bytes());
        }
        if let Some(receiver) = &self.receiver {
            bytes.extend_from_slice(receiver.as_bytes());
        }
        bytes.extend_from_slice(&self.amount.to_le_bytes());
        bytes.extend_from_slice(&self.timestamp.to_le_bytes());
        bytes
    }

    pub fn hash (&self) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.input(&self.bytes());
        hasher.result_str().into_bytes()
    }

    pub fn new_genesis_transaction() -> Transaction {
        // let mut wallet = Wallet::new();
        // println!("Secret key : {}", wallet.secret_to_string());
        // println!("Public Key : {}", wallet.public_to_string());
        let mut transaction = Transaction::new(None, Some("f30bcd6738f87172ec16e23e4bb8944275e568b04e118732099b4185b925e01a".to_string()), 21_000_000_000_000_000);
        transaction.signature = None;
        transaction
    }

    pub fn sign(&mut self, key: Keypair) {
        if self.sender == self.receiver {
            println!("You can't send money to yourself");
            return;
        }
        self.signature = Some(key.sign(&self.hash()).to_bytes().to_hex());
    }

    pub fn verify(&self) -> bool {
        match (&self.signature, &self.sender) {
            (Some(signature), Some(sender)) => {
                let public_key = PublicKey::from_bytes(&sender.from_hex().unwrap()).unwrap();
                let signature = Signature::from_bytes(&signature.from_hex().unwrap()).unwrap();
                public_key.verify(&self.hash(), &signature).is_ok()
            },
            (None, _) => {
                println!("You have to sign the transaction before verifying it");
                true
            },
            _ => false
        }
    }
}