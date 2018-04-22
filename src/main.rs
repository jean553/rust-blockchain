extern crate time;
extern crate sha1;
extern crate bincode;
extern crate termion;

extern crate serde;
#[macro_use] extern crate serde_derive;

use std::io::{
    stdin,
    Write,
    Read,
};
use std::net::{
    TcpListener,
    TcpStream,
    SocketAddr,
};
use std::time::Duration;
use std::str::FromStr;
use std::thread::spawn;
use bincode::{
    serialize,
    deserialize,
};
use termion::{
    color,
    terminal_size,
};
use termion::cursor::Goto;

mod hash_content;
mod block;
mod blocks;
mod peers;

use block::Block;

use blocks::{
    add_genesis_block,
    add_block,
    list_blocks,
};

use peers::{
    create_peer,
    list_peers,
};

const DEFAULT_STATUS: &str = "Waiting. Type 'help' to get the commands list.";

/// Handles user input and returns that input as a string.
///
/// Args:
///
/// `height` - the terminal height
///
/// Returns:
///
/// user input as string
fn get_input(height: u16) -> String {

    println!("{}", Goto(0, height - 3));

    let mut input = String::new();
    stdin().read_line(&mut input).expect("cannot read input");

    clear_screen();
    println!("{}", Goto(0, 2));

    input.trim().to_string()
}

/// Display the given text into an horizontal bar.
///
/// Args:
///
/// `text` - the text to display into the text bar
fn display_text_bar(text: &str) {

    println!(
        "{}{}{}{}{}{}",
        color::Bg(color::Blue),
        color::Fg(color::White),
        text,
        std::iter::repeat(' ')
            .take(terminal_size().unwrap().0 as usize - text.len())
            .collect::<String>(),
        color::Bg(color::Reset),
        color::Fg(color::Reset),
    );
}

/// Update the content of the status text bar.
///
/// Args:
///
/// `text` - the text to display into the text bar
/// `height` - the height of the terminal screen
fn set_status_text(text: &str, height: u16) {

    println!("{}", Goto(0, height - 2));
    display_text_bar(text);
    println!("{}", Goto(0, 2));
}

/// Clear the whole terminal content and generate the default content (bars and titles). Refactored as used multiple times and definition might not be clear.
fn clear_screen() {

    /* send a control character to the terminal */
    print!("{}[2J", 27 as char);

    println!("{}", Goto(1, 1));
    const TITLE: &str = "rust-blockchain";
    display_text_bar(TITLE);
}

/// Handle incoming TCP connections with other nodes.
fn handle_incoming_connections() {

    let listener = TcpListener::bind("0.0.0.0:10000").unwrap();

    for income in listener.incoming() {

        /* TODO: display message when receive a connection;
           should use mutex as it must modify the content
           of the main text area (so the cursor position
           must not be modified) */
    }
}

fn main() {

    let (_, height) = terminal_size().unwrap();
    let height = height as u16;

    clear_screen();

    let mut chain: Vec<Block> = Vec::new();
    let mut peers: Vec<SocketAddr> = Vec::new();

    spawn(|| { handle_incoming_connections() });

    loop {

        set_status_text(DEFAULT_STATUS, height);

        let input = get_input(height);
        let splitted: Vec<&str> = input.split(' ').collect();

        /* get() returns &&str, so we mention result type &str
           and get it from a reference (*) */
        let command: &str = match splitted.get(0) {
            Some(value) => *value,
            None => { continue; }
        };

        const ADD_BLOCK: &str = "add_block";
        const SEND_BLOCKCHAIN: &str = "send";
        const RECEIVE_BLOCKCHAIN: &str = "receive";
        const SEE_BLOCKCHAIN: &str = "list";
        const ADD_PEER: &str = "add_peer";
        const LIST_PEERS: &str = "list_peers";
        const HELP: &str = "help";

        let option = match splitted.get(1) {
            Some(option) => option,
            None => {

                if command == ADD_BLOCK ||
                    command == SEND_BLOCKCHAIN ||
                    command == ADD_PEER {
                    continue;
                }

                ""
            }
        };

        if command == ADD_BLOCK {

            let data: i32 = option.parse().unwrap();
            let chain = &mut chain;

            if chain.is_empty() {
                add_genesis_block(chain, data);
                continue;
            }

            add_block(chain, data);
        }
        else if command == SEND_BLOCKCHAIN {

            /* TODO: should be done automatically when add a new block */

            let full_address = format!("{}:10000", option);
            let bind_address = match SocketAddr::from_str(&full_address) {
                Ok(address) => address,
                Err(_) => {
                    println!("Incorrect address format.");
                    continue;
                }
            };

            set_status_text(&format!("Trying to connect to {}...", option), height);

            let mut stream = match TcpStream::connect_timeout(
                &bind_address,
                Duration::from_secs(5),
            ) {
                Ok(stream) => stream,
                Err(_) => {
                    println!("Cannot connect to the given node.");
                    continue;
                }
            };

            /* halt the program if serialization fails or socket write fails;
               this is not something the user can solve, and something is clearly wrong... */

            let bytes = serialize(&chain).unwrap();
            stream.write(&bytes).unwrap();
        }
        else if command == RECEIVE_BLOCKCHAIN {

            /* TODO: #33 not refactored by now, this should be handled by a separated thread */

            let listener = TcpListener::bind("0.0.0.0:10000").unwrap();

            println!("Waiting for connection...");

            let connection = listener.accept().unwrap();

            println!("Connection received.");

            let mut buffer: Vec<u8> = Vec::new();
            let mut stream = connection.0;

            stream.read_to_end(&mut buffer).unwrap();

            /* TODO: check integrity of the received chain */

            let received_chain: Vec<Block> = deserialize(&buffer).unwrap();
            if received_chain.len() > chain.len() {
                chain = received_chain;
            }
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

            /* TODO: should use command options */

            println!("add_block [data] - append a block into the local blockchain");
            println!("Example: add_block 10 \n");
            println!("send [ip] - send a copy of the blockchain to another node");
            println!("Example: send 172.17.0.10\n");
            println!("receive - receive a copy of the blockchain from another node\n");
            println!("list - list the local chain blocks\n");
            println!("add_peer - add one node as a peer");
            println!("Example: add_peer 172.17.0.10");
        }
    }
}
