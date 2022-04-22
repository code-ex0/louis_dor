mod blockchain;
mod block;
mod transaction;
mod wallet;

pub use blockchain::Blockchain;
pub use block::Block;
pub use transaction::Transaction;
pub use wallet::Wallet;

use std::time::{
    SystemTime,
};

pub fn current_time() -> u64
{
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(x) => {x.as_secs()}
        Err(_) => {0}
    }
}