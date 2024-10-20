use sha2::{ Sha256, Digest };

use super::blockchain::Blockchain;

#[derive(Debug)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: f64,
    pub timestamp: u64,
}

#[derive(Debug)]
pub struct Blob {
    pub data: String,
    pub timestamp: u64,
}

#[derive(Debug)]
pub enum Data {
    Transaction(Transaction),
    Blob(Blob),
}

#[derive(Debug)]
pub struct Block {
    pub index: u64,
    pub timestamp: u64,
    pub hash: String,
    pub prev_hash: String,
    pub nonce: u64,
    pub data: Vec<Data>,
}

pub type Blocks = Vec<Block>;

impl Block {
    pub fn new(index: u64, timestamp: u64, prev_hash: String, nonce: u64, data: Vec<Data>) -> Self {
        Block {
            index,
            timestamp,
            hash: String::default(),
            prev_hash,
            nonce,
            data,
        }
    }

    // fallback block for when mining the genesis (first) block
    // the real genesis block will be the first block mined, when the blockchain starts up
    pub fn genesis_sample() -> Self {
        Block {
            index: 0,
            timestamp: Blockchain::get_current_timestamp(),
            hash: String::default(),
            prev_hash: String::default(),
            nonce: 0,
            data: vec![],
        }
    }

    pub fn hash(&self) -> String {
        let mut block_data = self.data.iter().fold(String::new(), |acc, data| {
            match data {
                Data::Transaction(transaction) => {
                    format!("{}{}{}{}{}", acc, transaction.sender, transaction.receiver, transaction.amount, transaction.timestamp)
                }
                Data::Blob(blob) => {
                    format!("{}{}{}{}", acc, blob.data, blob.timestamp, self.prev_hash)
                }
            }
        });
        block_data = format!("{}{}{}{}", block_data, self.index, self.timestamp, self.nonce);
        let mut hasher = Sha256::new();
        hasher.update(block_data.as_bytes());
        let result = hasher.finalize();
        format!("{:x}", result)
    }
}