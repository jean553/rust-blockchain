//! Displays help.

/// List available commands.
pub fn list_commands() {

    println!("add_block [data] - append a block into the local blockchain");
    println!("Example: add_block 10 \n");
    println!("send [ip] - send a copy of the blockchain to another node");
    println!("Example: send 172.17.0.10\n");
    println!("receive - receive a copy of the blockchain from another node\n");
    println!("list - list the local chain blocks\n");
    println!("add_peer - add one node as a peer");
    println!("Example: add_peer 172.17.0.10");
}
