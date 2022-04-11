use louis_dor::current_time;
use ed25519_dalek::{Keypair, Signer, Verifier};
use ed25519_dalek::PublicKey;
use ed25519_dalek::Signature;
use crypto::digest::Digest;
use crypto::sha2::Sha256;

#[derive(Debug, Clone)]
pub struct Transaction {
    pub sender: Option<PublicKey>,
    pub receiver: Option<PublicKey>,
    pub amount: u64,
    pub timestamp: u64,
    pub signature: Option<Signature>,
}

impl Transaction {

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

    pub fn sign(&mut self, key: Keypair) {
        if let Some(p) = self.sender {
            if p != key.public {
                panic!("you can't sign for a other account")
            } else {
                self.signature = Some(key.sign(&self.hash()));
            }
        }
    }

    pub fn verify(&self) -> bool {
        match (self.sender, self.signature) {
            (Some(p), Some(s)) if p.verify(&self.hash(), &s).is_ok() => true,
            (None, _) => true, // for miner
            _ => false
        }
    }
}