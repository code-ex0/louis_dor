use ed25519_dalek::Keypair;
use ed25519_dalek::PublicKey;
use ed25519_dalek::SecretKey;
use rand::rngs::OsRng;
use rustc_serialize::hex::{FromHex, ToHex};

#[derive(Debug)]
pub struct Wallet {
    pub secret_key: SecretKey,
    pub public_key: PublicKey,
}

impl Wallet {
    pub fn new() -> Self {
        let mut rng = OsRng {};
        let keypair: Keypair = Keypair::generate(&mut rng);
        Wallet {
            public_key: keypair.public,
            secret_key: keypair.secret,
        }
    }

    pub fn from_string(secret_key: String, public_key: String) -> Keypair {
        Keypair::from_bytes(&[secret_key.from_hex().unwrap(), public_key.from_hex().unwrap()].concat()).expect("Failed to create keypair")
    }

    pub fn from_public_string(public_key: &String) -> PublicKey {
        PublicKey::from_bytes(&public_key.from_hex().unwrap()).expect("Failed to create public key")
    }

    pub fn from_secret_string(secret_key: String) -> SecretKey {
        SecretKey::from_bytes(&secret_key.from_hex().unwrap()).expect("Failed to create secret key")
    }

    pub fn from_secret_key(secret_key: SecretKey) -> Self {
        let public_key = PublicKey::from(&secret_key);
        Wallet {
            public_key,
            secret_key,
        }
    }

    pub fn to_keypair(&self) -> Keypair {
        Keypair::from_bytes(&[self.secret_key.to_bytes(), self.public_key.to_bytes()].concat()).expect("Failed to create keypair")
    }

    pub fn public_to_string(&self) -> String {
        self.public_key.to_bytes().to_hex()
    }

    pub fn secret_to_string(&self) -> String {
        self.secret_key.to_bytes().to_hex()
    }
}