//! Blocks routines.

use std::net::TcpStream;
use std::io::Write;
use std::sync::{
    Arc,
    Mutex,
};

use bincode::serialize;

use block::Block;

use message::{
    Message,
    MessageLabel,
};

use peers::create_stream;

/// Displays the blockchain blocks.
///
/// Args:
///
/// `chain` - the chain to modify
pub fn list_blocks(chain: &Arc<Mutex<Vec<Block>>>) {

    let chain = chain.lock().unwrap();

    for block in chain.iter() {

        let content = block.get_content();
        println!("Hash: {}", block.get_current());
        println!("Timestamp: {}", content.get_timestamp());
        println!("Data: {} \n\n", content.get_data());
    }
}

/// Tries to send the given block to all the given peers. Skip peer if timeout.
///
/// Args:
///
/// `peers` - list of peers
/// `block` - the block object to send
pub fn broadcast_block(peers: &Vec<String>, block: Block) {

    /* we voluntary halt the program if serialization and stream buffer write fails;
       in fact, if these problem happen, that means something is clearly wrong */

    let message = Message::new(
        vec![block],
        MessageLabel::SendBlock,
    );

    let bytes = serialize(&message).unwrap();

    for peer in peers.iter() {

        let address_part: Vec<&str> = peer.split(':').collect();
        let address = address_part.get(0).unwrap();

        let mut stream = match create_stream(&peer) {
            Some(stream) => stream,
            None => {
                println!("Cannot connect to {}.", address);
                continue;
            }
        };

        stream.write(&bytes).unwrap();
        println!("Block sent to {}.", address);
    }

    println!("Block creation broadcast terminated.");
}

/// Adds one block to the chain from a received message. Takes the first block of the chain. Panics if an error occurs.
///
/// Args:
///
/// `chain` - the chain to update
/// `message` - the message from where extract the unique block
pub fn add_block_from_message(
    chain: &Arc<Mutex<Vec<Block>>>,
    message: &Message,
) {

    let block = message.get_blocks().first().unwrap();

    let mut chain = chain.lock().unwrap();
    chain.push((*block).clone());

    println!("Received block added into the chain.");
}

/// Sends the local chain to another node through the given stream.
///
/// Args:
///
/// `stream` - the stream where data must be written
/// `chain` - the chain to use
pub fn send_last_block_to_stream(
    mut stream: TcpStream,
    chain: &Arc<Mutex<Vec<Block>>>,
) {

    println!("Last block requested.");

    let mut message = Message::new(
        Vec::new(),
        MessageLabel::SendBlock,
    );

    let chain = chain.lock().unwrap();

    if !chain.is_empty() {
        message.set_blocks(chain.clone());
    }

    let bytes = serialize(&message).unwrap();
    stream.write(&bytes).unwrap();

    println!("Last block sent.");
}
