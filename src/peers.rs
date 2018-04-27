//! Peers routines.

use std::net::{
    SocketAddr,
    TcpStream,
};
use std::time::Duration;
use std::str::FromStr;

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
        Ok(socket_address) => {
            println!("Address {} added to peers list.", address);
            socket_address
        },
        Err(_) => {
            println!("Incorrect address format.");
            return;
        }
    };

    peers.push(socket_address.clone());

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

    /* TODO: the peer sends its last block: if the current local chain is empty
       or if the received last block from the peer is not the same one
       as the local one, then, the whole local chain is updated */
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
