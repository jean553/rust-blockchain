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

/// Creates a new peer.
///
/// Args:
///
/// `peers` - the peers array to modify
/// `address` - the new ip address (text format) to add
pub fn create_peer(peers: &mut Vec<SocketAddr>, address: &str) {

    const PORT: &str = "10000";
    let full_address = format!("{}:{}", address, PORT);

    let socket_address = match SocketAddr::from_str(&full_address) {
        Ok(socket_address) => socket_address,
        Err(_) => {
            println!("Incorrect address format.");
            return;
        }
    };

    peers.push(socket_address.clone());

    println!("Address {} added to peers list.", address);

    println!("Connecting to {}...", address);

    let mut stream = match TcpStream::connect_timeout(
        &socket_address,
        Duration::from_secs(5),
    ) {
        Ok(stream) => stream,
        Err(_) => {
            println!("The peer {} has been added but cannot be joined right now.", address);
            return;
        }
    };

    println!("Connected to {}.", address);

    let message = Message::new(
        Vec::new(),
        MessageLabel::AskLastBlock,
    );

    let bytes = serialize(&message).unwrap();

    stream.write(&bytes).unwrap();

    println!("Last block asked to {}.", address);
    println!("Waiting for reply...");

    /* TODO: explain why a message maximum size is 80 bytes long */
    const ONE_BLOCK_MESSAGE_MAX_LENGTH: usize = 80;

    let mut buffer: Vec<u8> = vec![0; ONE_BLOCK_MESSAGE_MAX_LENGTH];
    stream.read(&mut buffer).expect("Received message is too long.");

    let message: Message = deserialize(&buffer).unwrap();

    if message.get_blocks().is_empty() {
        println!("No block returned.");
        return;
    }

    println!("One block has been received.");

    /* TODO: compare blocks in order to know
       if the local one is the same as the received one */
}

/// Displays all the peers.
///
/// Args:
///
/// `peers` - the list of peers to display
pub fn list_peers(peers: &Vec<SocketAddr>) {

    for peer in peers.iter() {
        println!("{}", peer.to_string());
    }
}
