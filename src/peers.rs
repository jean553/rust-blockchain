//! Peers routines.

use std::net::{
    SocketAddr,
    TcpStream,
};
use std::io::{
    Write,
    Read,
};
use std::time::Duration;
use std::str::FromStr;

use bincode::{
    deserialize,
    serialize,
};

use message::{
    Message,
    MessageLabel,
};

use block::Block;

/// Check the given address and returns a stream to communicate with the specified node. Handles errors with output messages.
///
/// Args:
///
/// `address` - the node address in format IP:PORT
///
/// Returns:
///
/// the created TCP stream
pub fn create_stream(address: &str) -> Option<TcpStream> {

    println!("Connecting to {}...", address);

    let socket_address = match SocketAddr::from_str(&address) {
        Ok(socket_address) => socket_address,
        Err(_) => {
            println!("Incorrect address format.");
            return None;
        }
    };

    let stream = match TcpStream::connect_timeout(
        &socket_address,
        Duration::from_secs(5),
    ) {
        Ok(stream) => stream,
        Err(_) => {
            println!("The peer cannot be joined.");
            return None;
        }
    };

    println!("Connected to {}.", address);

    Some(stream)
}

/// Creates a new peer.
///
/// Args:
///
/// `stream` - the stream opened to the added peer
///
/// Returns:
///
/// the received remote chain
pub fn get_chain_from_stream(mut stream: TcpStream) -> Vec<Block> {

    let message = Message::new(
        Vec::new(),
        MessageLabel::AskForAllBlocks,
    );

    let bytes = serialize(&message).unwrap();

    stream.write(&bytes).unwrap();

    println!("Waiting for reply...");

    /* TODO: explain why a message maximum size is 80 bytes long */
    const ONE_BLOCK_MESSAGE_MAX_LENGTH: usize = 80;

    let mut buffer: Vec<u8> = vec![0; ONE_BLOCK_MESSAGE_MAX_LENGTH];
    stream.read(&mut buffer).expect("Received message is too long.");

    let message: Message = deserialize(&buffer).unwrap();
    message.get_blocks().clone()
}

/// Displays all the peers.
///
/// Args:
///
/// `peers` - the list of peers to display
pub fn list_peers(peers: &Vec<String>) {

    for peer in peers.iter() {
        println!("{}", peer);
    }
}
