use blockchain::{Blockchain, Block, current_time};
use libp2p::{
    floodsub::{Floodsub, FloodsubEvent, Topic},
    identity,
    mdns::{Mdns, MdnsEvent},
    swarm::{NetworkBehaviourEventProcess, Swarm},
    NetworkBehaviour, PeerId,
};
use log::{error, info};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use tokio::sync::mpsc;

pub static KEYS: Lazy<identity::Keypair> = Lazy::new(identity::Keypair::generate_ed25519);
pub static PEER_ID: Lazy<PeerId> = Lazy::new(|| PeerId::from(KEYS.public()));
pub static CHAIN_TOPIC: Lazy<Topic> = Lazy::new(|| Topic::new("chain"));
pub static BLOCK_TOPIC: Lazy<Topic> = Lazy::new(|| Topic::new("block"));


#[derive(Serialize, Deserialize, Debug)]
pub struct ChainResponse {
    pub blocks: Vec<Block>,
    received: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LocalChainRequest {
    pub from_peer_id: String,
}

pub enum EventType{
    LocalChainResponse(ChainResponse),
    Input(String),
    Init,
}

#[derive(NetworkBehaviour)]
pub struct BlockchainBehaviour {
    pub floodsub: Floodsub,
    pub mdns: Mdns,
    #[behaviour(ignore)]
    pub response_sender: mpsc::UnboundedSender<ChainResponse>,
    #[behaviour(ignore)]
    init_sender: mpsc::UnboundedSender<bool>,
    #[behaviour(ignore)]
    pub chain: Blockchain,
}

impl BlockchainBehaviour {
    pub async fn new(
        chain: Blockchain,
        response_sender: mpsc::UnboundedSender<ChainResponse>,
        init_sender: mpsc::UnboundedSender<bool>) -> Self {
        let mut behaviour = BlockchainBehaviour {
            floodsub: Floodsub::new(*PEER_ID),
            mdns: Mdns::new(Default::default()).await.expect("Failed to create mdns"),
            response_sender,
            init_sender,
            chain,
        };

        behaviour.floodsub.subscribe(CHAIN_TOPIC.clone());
        behaviour.floodsub.subscribe(BLOCK_TOPIC.clone());

        behaviour
    }
}



pub fn get_list_peers(swarm: &Swarm<BlockchainBehaviour>) -> Vec<String> {
    info!("Discovered peers");
    let nodes = swarm.behaviour().mdns.discovered_nodes();
    let mut unique_peers: HashSet<String> = HashSet::new();
    for node in nodes {
        let peer_id = node.id.to_base58();
        unique_peers.insert(peer_id);
    }
    unique_peers.iter().map(|x| x.to_string()).collect()
}

pub fn handle_print_peers(swarm: &Swarm<BlockchainBehaviour>) {
    let peers = get_list_peers(swarm);
    peers.iter().for_each(|x| info!("{}", x));
}

pub fn handle_print_chain(swarm: &Swarm<BlockchainBehaviour>) {
    info!("Local Blockchain:");
    let pretty_json = serde_json::to_string_pretty(&swarm.behaviour().chain).unwrap();
    info!("{}", pretty_json);
}

pub fn handle_create_block(cmd :&str, swarm: &mut Swarm<BlockchainBehaviour>) {
    if let Some(data) = cmd.strip_prefix("create b") {
        let behavior = swarm.behaviour_mut();
        let last_block = behavior.chain.last_block();
        let new_block = Block{
            index: 0,
            hash: "".to_string(),
            previous_hash: last_block.hash.clone(),
            timestamp: current_time(),
            transactions: vec![],
            nonce: 0
        }

    }
}

//https://github.com/zupzup/rust-blockchain-example/blob/main/src/p2p.rs