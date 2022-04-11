use std::fmt::format;
use std::str::FromStr;
use crypto::ed25519::keypair;
use crypto::util::secure_memset;
use ed25519_dalek::{Keypair, PublicKey};
use rand::Rng;
use rustc_serialize::hex;
use rustc_serialize::hex::{ToHex, FromHex};

pub mod blockchain;
use louis_dor::{current_time};
use blockchain::{Block, Blockchain, Wallet, Transaction};

fn input_user() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    let mut wallet = Wallet::new();
    println!("Your secret key is: {:?}", wallet.secret_key.to_bytes().to_hex());
    println!("Your public key is: {:?}", wallet.public_key.to_bytes().to_hex());

    let mut wallet2 = Wallet::new();
    println!("Your secret key is: {:?}", wallet2.secret_key.to_bytes().to_hex());
    println!("Your public key is: {:?}", wallet2.public_key.to_bytes().to_hex());


    let mut blockchain = Blockchain::new();
    blockchain.genesis_block();
    println!("Welcome to the blockchain!");

    loop {
        println!("Enter a command: ");
        let mut command = String::new();
        std::io::stdin().read_line(&mut command).expect("Failed to read line");
        let command = command.trim();
        if command == "exit" {
            break;
        }
        if command == "mine" {
            blockchain.mine_block(2);
        }
        if command == "print" {
            println!("{:#?}", blockchain.chain);
        }
        if command == "transaction" {
            println!("Enter the sender's public key: ");
            let sender_public_key = input_user();
            println!("Enter the sender's private key: ");
            let sender_private_key = input_user();
            println!("Enter the recipient's public key: ");
            let receiver = input_user();
            println!("Enter the amount: ");
            let amount = input_user().trim().parse::<u64>().unwrap();


            let keypair = Keypair::from_bytes(&[sender_private_key.from_hex().unwrap(), sender_public_key.from_hex().unwrap()].concat()).expect("Failed to create keypair");
            let receiver_public_key = PublicKey::from_bytes(&receiver.from_hex().unwrap()).unwrap();
            let mut transaction = Transaction {
                sender: Some(keypair.public),
                receiver: Some(receiver_public_key),
                amount,
                timestamp: current_time(),
                signature: None
            };
            transaction.sign(keypair);
            blockchain.new_transaction(transaction.clone());
            println!("{:#?}", transaction);
            println!("{:#?}", transaction.verify());
        }
    }
}

