use chrono::Utc;
use crate::engine::block::{Block, Blocks};

use super::block::Data;

pub struct Blockchain {
    chain: Blocks,
    difficulty: usize
}

impl Blockchain {
    pub fn new(difficulty: usize) -> Self {
        Blockchain {
            chain: vec![],
            difficulty
        }
    }

    pub fn add_block(&mut self, block: Block) {
        self.chain.push(block);
    }

    pub fn get_last_block(&self) -> Option<&Block> {
        self.chain.last()
    }

    pub fn len(&self) -> usize {
        self.chain.len()
    }

    pub fn get_current_timestamp() -> u64 {
        let now = Utc::now();
        now.timestamp_millis() as u64
    }

    pub fn get_time_elasped(start_time: u64, end_time: u64) -> String {
        let ms_diff = end_time - start_time;
        let ms = ms_diff % 1000;
        let sec = (ms_diff / 1000) % 60;
        let min = (ms_diff / (1000 * 60)) % 60;
        let hour = (ms_diff / (1000 * 60 * 60)) % 24;
        format!("{:02}h:{:02}m:{:02}s.{:03}ms", hour, min, sec, ms)
    }

    pub fn mine_block(&mut self, data: Vec<Data>) {
        let start_time = Blockchain::get_current_timestamp();
        // fallback block for when mining the genesis (first) block
        let genesis_block_sample = Block::genesis_sample();
        let prev_block = self.get_last_block().unwrap_or(&genesis_block_sample);
        let mut block = Block::new(
            self.len() as u64,
            start_time,
            prev_block.hash.clone(),
            0,
            data
        );

        let mut hash = String::from("");
        while hash.starts_with(&"0".repeat(self.difficulty)) == false {
            block.nonce += 1;
            hash = block.hash();
        }
        // log time taken to mine block
        let end_time = Blockchain::get_current_timestamp();
        println!("Block {} mined in {} with hash: {}. It contains {} transaction data in it", block.index, Blockchain::get_time_elasped(start_time, end_time), hash, block.data.len());
        // update block's hash value and add to chain
        block.hash = hash;
        self.add_block(block);
    }

    pub fn log_blockchain(&self) {
        for block in &self.chain {
            println!("No #{}: {:?}", block.index, block);
        }
    }
}