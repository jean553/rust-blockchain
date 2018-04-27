//! Displays help.

/// List available commands.
pub fn list_commands() {

    println!("add_block [data] - append a block into the local blockchain");
    println!("Example: add_block 10 \n");
    println!("list_blocks - list the local chain blocks\n");
    println!("add_peer - add one node as a peer");
    println!("Example: add_peer 172.17.0.10\n");
    println!("list_peers - list the peers\n");
    println!("exit - quit the program");
}
