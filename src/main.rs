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
mod message;

use std::io::Read;
use std::net::TcpListener;
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
    add_block_from_message,
    send_last_block_to_stream,
};

use peers::{
    get_chain_from_stream,
    list_peers,
    create_stream,
};

use help::list_commands;

use display::{
    clear_screen,
    get_input,
    set_cursor_into_logs,
    set_cursor_into_input,
};

use message::{
    Message,
    MessageLabel,
};

const LISTENING_PORT: &str = "10000";

/// Handle incoming TCP connections with other nodes.
///
/// Args:
///
/// `chain` - the chain to manipulate
fn handle_incoming_connections(chain: Arc<Mutex<Vec<Block>>>) {

    let address = format!("0.0.0.0:{}", LISTENING_PORT);
    let listener = TcpListener::bind(address).unwrap();

    /* blocks until data is received */
    for income in listener.incoming() {

        /* TODO: display message when receive a connection;
           should use mutex as it must modify the content
           of the main text area (so the cursor position
           must not be modified) */

        clear_screen();
        set_cursor_into_logs();

        let mut stream = income.unwrap();

        const MESSAGE_MAX_LENGTH: usize = 20;
        let mut buffer: Vec<u8> = vec![0; MESSAGE_MAX_LENGTH];

        /* blocks until data is received  */
        stream.read(&mut buffer).expect("Received message is too long.");

        let message: Message = deserialize(&buffer).unwrap();
        let label = message.get_label();

        if label == &MessageLabel::AskForAllBlocks {
            send_last_block_to_stream(
                stream,
                &chain,
            );
        }
        else if label == &MessageLabel::SendBlock {
            add_block_from_message(
                &chain,
                &message,
            );
        }

        set_cursor_into_input();
    }
}

fn main() {

    clear_screen();

    println!("Type 'help' to list commands.");

    let chain: Arc<Mutex<Vec<Block>>> = Arc::new(Mutex::new(Vec::new()));
    let mut peers: Vec<String> = Vec::new();

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
        const EXIT: &str = "exit";
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

            let full_address = format!("{}:{}", option, LISTENING_PORT);
            peers.push(full_address.clone());

            println!("Address {} added to peers list.", option);

            let stream = create_stream(&full_address);
            if stream.is_some() {
                let remote_chain = get_chain_from_stream(stream.unwrap());

                let mut chain = chain.lock().unwrap();

                if remote_chain.len() > chain.len() {
                    *chain = remote_chain.clone();
                    println!("The local chain is outdated compared to the remote one, replaced.");
                } else {
                    println!("The local chain is up-to-date compared to the remote one.");
                }
            }
        }
        else if command == LIST_PEERS {
            list_peers(&peers);
        }
        else if command == HELP {
            list_commands();
        }
        else if command == EXIT {
            break;
        } else {
            println!("Command not found. Type 'help' to list commands.");
        }
    }
}
