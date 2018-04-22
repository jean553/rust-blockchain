//! Peers routines.

use std::net::SocketAddr;
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

    match SocketAddr::from_str(&full_address) {
        Ok(socket_address) => {
            peers.push(socket_address);
            println!("Address {} added to peers list.", address);
        },
        Err(_) => {
            println!("Incorrect address format.");
        }
    };
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
