use chrono::Utc;
use crate::engine::block::{Block, Blocks};

use super::block::Data;

pub struct Blockchain {
    chain: Blocks
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain {
            chain: vec![]
        }
    }

    pub fn add_block(&mut self, block: Block) {
        self.chain.push(block);
    }

    pub fn last_block(&self) -> Option<&Block> {
        self.chain.last()
    }

    pub fn len(&self) -> usize {
        self.chain.len()
    }

    pub fn get_current_timestamp() -> u64 {
        let now = Utc::now();
        now.timestamp_millis() as u64
    }

    pub fn mine_block(&mut self, difficulty: u64, data: Vec<Data>) {
        let genesis_block = Block::genesis();
        let prev_block = self.last_block().unwrap_or(&genesis_block);
        let mut block = Block::new(
            self.len() as u64,
            Blockchain::get_current_timestamp(),
            prev_block.hash.clone(),
            0,
            data
        );

        let mut hash = String::from("");
        while hash.starts_with(&"0".repeat(difficulty as usize)) == false {
            block.nonce += 1;
            hash = block.hash();
        }
        block.hash = hash;
        self.add_block(block);
    }

    pub fn log(&self) {
        for block in &self.chain {
            println!("{:?}", block);
        }
    }
}