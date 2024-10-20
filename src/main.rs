mod engine;

use crate::engine::{block, blockchain};

fn main() {
    let mut bc = blockchain::Blockchain::new();
    bc.mine_block(6, vec![
        block::Data::Blob(block::Blob {
            data: String::from("Wie geht's Welt???"),
            timestamp: blockchain::Blockchain::get_current_timestamp(),
        }),
        block::Data::Transaction(block::Transaction {
            sender: String::from("Eva"),
            receiver: String::from("Victor"),
            amount: 100.0,
            timestamp: blockchain::Blockchain::get_current_timestamp(),
        }),
    ]);

    bc.mine_block(6, vec![
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

    bc.mine_block(6, vec![
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

    bc.log_blockchain();
}
