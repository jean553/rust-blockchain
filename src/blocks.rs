//! Blocks routines.

use block::Block;

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
