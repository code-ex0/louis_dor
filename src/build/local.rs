use rand::Rng;
use blockchain::{Wallet, Blockchain, Transaction};

fn input_user() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    generate_fake_data();
    let mut blockchain = Blockchain::new();
    blockchain.genesis_block();
    println!("Welcome to the blockchain!");

    let wallet = Wallet::new();
    println!("Your secret key is: {:?}", wallet.secret_to_string());
    println!("Your public key is: {:?}", wallet.public_to_string());

    let wallet2 = Wallet::new();
    println!("Your secret key is: {:?}", wallet2.secret_to_string());
    println!("Your public key is: {:?}", wallet2.public_to_string());

    loop {
        println!("Enter a command: ");
        let mut command = String::new();
        std::io::stdin().read_line(&mut command).expect("Failed to read line");
        let command = command.trim();
        if command == "exit" {
            break;
        }
        if command == "mine" {
            blockchain.mine_block(2, wallet.public_to_string(), 5);
        }
        if command == "print" {
            println!("{:#?}", blockchain.chain);
        }
        if command == "balance" {
            let address = input_user();
            println!("{:#?}", blockchain.calculate_balance(address));
        }
        if command == "balances" {
            for i in blockchain.get_wallets() {
                println!("{} : {}", i.clone(),  blockchain.calculate_balance(i));
            }
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

            let mut transaction = Transaction::new(Some(sender_public_key.clone()), Some(receiver), amount);
            transaction.sign(Wallet::from_string(sender_private_key, sender_public_key));
            blockchain.new_transaction(transaction);
        }
    }
}

fn generate_fake_data() {
    let mut rng = rand::thread_rng();
    let mut wallets: Vec<Wallet> = Vec::new();
    let mut blockchain = Blockchain::new();
    blockchain.genesis_block();
    let keypair = Wallet::from_string("12933f30846d732ee2a68d24d891d5620a19ead3bb465a2feba462276851fb90".to_string(), "f30bcd6738f87172ec16e23e4bb8944275e568b04e118732099b4185b925e01a".to_string());
    let genesis_wallet = Wallet {
        public_key: keypair.public,
        secret_key: keypair.secret,
    };

    for _ in 0..2000 {
        let wallet = Wallet::new();
        wallets.push(wallet);
    }

    let len = wallets.len();

    for i in &wallets {
        let mut transaction = Transaction::new(Some(genesis_wallet.public_to_string()), Some(i.public_to_string()), 100_000_000_000);
        transaction.sign(genesis_wallet.to_keypair());
        blockchain.new_transaction(transaction);
    }
    blockchain.mine_block(2, genesis_wallet.public_to_string(), 10);

    for _ in 0..100 {
        for _ in 0..100 {
            let sender = rng.gen_range(0, len);
            let mut receiver = rng.gen_range(0, len);
            while receiver == sender {
                receiver = rng.gen_range(0, len);
            }
            let amount = rng.gen_range(1_000_000_000, 100_000_000_000);
            let mut transaction = Transaction::new(Some(wallets[sender].public_to_string()), Some(wallets[receiver].public_to_string()), amount);
            transaction.sign(wallets[sender].to_keypair());
            blockchain.new_transaction(transaction);
        }
        blockchain.mine_block(2, genesis_wallet.public_to_string(), 10);
    }
    println!("------------------------------");
    println!("{:#?}", blockchain);
    for i in blockchain.get_wallets() {
        println!("{} : {}", i.clone(),  blockchain.calculate_balance(i) as f64 / 1_000_000_000.0);
    }
    let mut balances: Vec<(String, u64)> = Vec::new();
    for i in blockchain.get_wallets() {
        balances.push((i.clone(), blockchain.calculate_balance(i)));
    }
    println!("------------------------------");
    balances.sort_by(|a, b| b.1.cmp(&a.1));
    for i in 0..10 {
        // print the balance but the last 9 digits or under 0.1
        println!("{} : {:.9}", balances[i].0, balances[i].1 as f64 / 1_000_000_000.0);
    }
    println!("------------------------------");
    for i in balances.len() - 10..balances.len() {
        println!("{} : {:.9}", balances[i].0, balances[i].1 as f64 / 1_000_000_000.0);
    }
}
