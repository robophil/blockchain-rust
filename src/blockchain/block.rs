use sha2::{ Sha256, Digest };

pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: f64,
    pub timestamp: u64,
}

pub struct Blob {
    pub data: String,
    pub timestamp: u64,
}

pub enum Data {
    Transaction(Transaction),
    Blob(Blob),
}

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

    pub fn set_hash(&mut self, hash: String) {
        self.hash = hash;
    }

    pub fn set_nonce(&mut self, nonce: u64) {
        self.nonce = nonce;
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