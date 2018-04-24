extern crate time;
extern crate sha1;
extern crate bincode;
extern crate termion;

extern crate serde;
#[macro_use] extern crate serde_derive;

mod hash_content;
mod block;
mod blocks;
mod peers;
mod help;
mod display;

use std::io::Read;
use std::net::{
    TcpListener,
    SocketAddr,
};
use std::thread::spawn;
use std::sync::{
    Arc,
    Mutex,
};

use bincode::deserialize;

use block::Block;

use blocks::{
    list_blocks,
    broadcast_block,
};

use peers::{
    create_peer,
    list_peers,
};

use help::list_commands;

use display::{
    clear_screen,
    get_input,
    set_cursor_into_logs,
    set_cursor_into_input,
};

/// Handle incoming TCP connections with other nodes.
///
/// Args:
///
/// `chain` - the chain to manipulate
fn handle_incoming_connections(chain: Arc<Mutex<Vec<Block>>>) {

    let listener = TcpListener::bind("0.0.0.0:10000").unwrap();

    for income in listener.incoming() {

        /* TODO: display message when receive a connection;
           should use mutex as it must modify the content
           of the main text area (so the cursor position
           must not be modified) */

        set_cursor_into_logs();

        let mut stream = income.unwrap();

        let mut buffer: Vec<u8> = Vec::new();
        stream.read_to_end(&mut buffer).unwrap();

        let block: Block = deserialize(&buffer).unwrap();

        let mut chain = chain.lock().unwrap();
        chain.push(block);

        println!(
            "Block from {} has been added to the chain.",
            stream.peer_addr().unwrap(),
        );

        set_cursor_into_input();
    }
}

fn main() {

    clear_screen();

    println!("Type 'help' to list commands.");

    let chain: Arc<Mutex<Vec<Block>>> = Arc::new(Mutex::new(Vec::new()));
    let mut peers: Vec<SocketAddr> = Vec::new();

    let listener_chain = chain.clone();
    spawn(|| { handle_incoming_connections(listener_chain) });

    loop {

        let input = get_input();
        let splitted: Vec<&str> = input.split(' ').collect();

        /* get() returns &&str, so we mention result type &str
           and get it from a reference (*) */
        let command: &str = match splitted.get(0) {
            Some(value) => *value,
            None => { continue; }
        };

        const ADD_BLOCK: &str = "add_block";
        const SEE_BLOCKCHAIN: &str = "list_blocks";
        const ADD_PEER: &str = "add_peer";
        const LIST_PEERS: &str = "list_peers";
        const HELP: &str = "help";

        let option = match splitted.get(1) {
            Some(option) => option,
            None => {

                if command == ADD_BLOCK ||
                    command == ADD_PEER {
                    continue;
                }

                ""
            }
        };

        if command == ADD_BLOCK {

            let data: i32 = option.parse().unwrap();
            let mut chain = chain.lock().unwrap();

            let mut previous_digest = String::new();

            if !chain.is_empty() {

                previous_digest = chain.last()
                    .unwrap()
                    .get_current()
                    .to_string();
            }

            let block = Block::new(data, previous_digest);
            chain.push(block.clone());

            println!("New block added.");

            broadcast_block(&peers, block);
        }
        else if command == SEE_BLOCKCHAIN {
            list_blocks(&chain);
        }
        else if command == ADD_PEER {
            create_peer(&mut peers, option);
        }
        else if command == LIST_PEERS {
            list_peers(&peers);
        }
        else if command == HELP {
            list_commands();
        }
    }
}
