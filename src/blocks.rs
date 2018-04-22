//! Blocks routines.

use block::Block;

/// Creates a genesis block and appends it to the chain.
///
/// Args:
///
/// `chain` - the chain to modify
/// `data` - the data to add
pub fn add_genesis_block(chain: &mut Vec<Block>, data: i32) {

    let genesis = Block::new(data, String::new());

    println!("Genesis block has been generated.");
    println!("Current block digest: {}", genesis.get_current());

    chain.push(genesis);
}

/// Creates a standard block and appends it into the chain.
///
/// Args:
///
/// `chain` - the chain to modify
/// `data` - the data to add
pub fn add_block(chain: &mut Vec<Block>, data: i32) {

    let current_digest = chain.last()
        .unwrap()
        .get_current()
        .to_string();

    let block = Block::new(data, current_digest.clone());

    println!("One block has been added to the ledger.");
    println!("Current block digest: {}", block.get_current());

    chain.push(block);
}

/// Displays the blockchain blocks.
///
/// Args:
///
/// `chain` - the chain to modify
pub fn list_blocks(chain: &Vec<Block>) {

    for block in chain.iter() {

        let content = block.get_content();
        println!("Hash: {}", block.get_current());
        println!("Timestamp: {}", content.get_timestamp());
        println!("Data: {} \n\n", content.get_data());
    }
}
