mod engine;

use crate::engine::{block, blockchain};

fn main() {
    let mut bc = blockchain::Blockchain::new(6);
    // mine genesis block
    bc.mine_block(vec![
        block::Data::Blob(block::Blob {
            data: String::from("This is the genesis block."),
            timestamp: blockchain::Blockchain::get_current_timestamp(),
        }),
        block::Data::Blob(block::Blob {
            data: String::from("Adding more data to the genesis block."),
            timestamp: blockchain::Blockchain::get_current_timestamp(),
        }),
        block::Data::Blob(block::Blob {
            data: String::from("That's all for the genesis block. More blocks to come :)"),
            timestamp: blockchain::Blockchain::get_current_timestamp(),
        }),
    ]);

    bc.mine_block(vec![
        block::Data::Blob(block::Blob {
            data: String::from("Hello, World!"),
            timestamp: blockchain::Blockchain::get_current_timestamp(),
        }),
        block::Data::Transaction(block::Transaction {
            sender: String::from("Eva"),
            receiver: String::from("Robo"),
            amount: 200.0,
            timestamp: blockchain::Blockchain::get_current_timestamp(),
        }),
    ]);

    bc.mine_block(vec![
        block::Data::Blob(block::Blob {
            data: String::from("Goodbye, World!"),
            timestamp: blockchain::Blockchain::get_current_timestamp(),
        }),
        block::Data::Transaction(block::Transaction {
            sender: String::from("Oluka"),
            receiver: String::from("Robo"),
            amount: 200.0,
            timestamp: blockchain::Blockchain::get_current_timestamp(),
        }),
        block::Data::Blob(block::Blob {
            data: String::from("Goodbye, World!"),
            timestamp: blockchain::Blockchain::get_current_timestamp(),
        }),
        block::Data::Blob(block::Blob {
            data: String::from("Hola"),
            timestamp: blockchain::Blockchain::get_current_timestamp(),
        }),
        block::Data::Blob(block::Blob {
            data: String::from("Ciao"),
            timestamp: blockchain::Blockchain::get_current_timestamp(),
        }),
    ]);

    println!("Blockchain length: {}", bc.len());
    bc.log_blockchain();
}
