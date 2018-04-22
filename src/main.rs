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
};
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

use block::Block;

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

    clear_screen(height);
    println!("{}", Goto(0, 2));

    input.trim().to_string()
}

/// Display text into a blue bar with a width that is as long as the terminal width. Refactored as it is used multiple times.
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

/// Clear the whole terminal content and generate the default content (bars and titles). Refactored as used multiple times and definition might not be clear.
///
/// Args:
///
/// `height` - the terminal height
fn clear_screen(height: u16) {

    /* send a control character to the terminal */
    print!("{}[2J", 27 as char);

    println!("{}", Goto(1, 1));
    display_text_bar("rust-blockchain");

    println!("{}", Goto(0, height - 1));
    display_text_bar("Waiting. Type 'help' to get the commands list.");
}

fn main() {

    let (_, height) = terminal_size().unwrap();
    let height = height as u16;

    clear_screen(height);

    let mut chain: Vec<Block> = Vec::new();

    println!("{}", Goto(0, 2));

    loop {

        let input = get_input(height);
        let splitted: Vec<&str> = input.split(' ').collect();

        let command = match splitted.get(0) {
            Some(value) => value.trim(),
            None => { continue; }
        };

        const ADD_BLOCK_CHOICE: &str = "add_block";
        const SEND_BLOCKCHAIN_CHOICE: &str = "send";
        const RECEIVE_BLOCKCHAIN_CHOICE: &str = "receive";
        const SEE_BLOCKCHAIN_CHOICE: &str = "list";
        const HELP_CHOICE: &str = "help";

        const PORT: &str = "10000";

        if command == ADD_BLOCK_CHOICE {

            let data: i32 = match splitted.get(1) {
                Some(value) => value.trim().parse().unwrap(),
                None => { continue; }
            };

            if chain.is_empty() {

                let genesis = Block::new(data, String::new());

                println!("Genesis block has been generated.");
                println!("Current block digest: {}", genesis.get_current());

                chain.push(genesis);

                continue;
            }

            let current_digest = chain.last()
                .unwrap()
                .get_current()
                .to_string();

            let block = Block::new(data, current_digest.clone());

            println!("One block has been added to the ledger.");
            println!("Current block digest: {}", block.get_current());

            chain.push(block);
        }
        else if command == SEND_BLOCKCHAIN_CHOICE {

            let address = match splitted.get(1) {
                Some(value) => value.trim(),
                None => { continue; }
            };

            let full_address = format!("{}:{}", address, PORT);
            let mut stream = TcpStream::connect(full_address).unwrap();

            let bytes = serialize(&chain).unwrap();
            stream.write(&bytes).unwrap();
        }
        else if command == RECEIVE_BLOCKCHAIN_CHOICE {

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
        else if command == SEE_BLOCKCHAIN_CHOICE {

            for block in chain.iter() {

                let content = block.get_content();
                println!("Hash: {}", block.get_current());
                println!("Timestamp: {}", content.get_timestamp());
                println!("Data: {} \n\n", content.get_data());
            }
        }
        else if command == HELP_CHOICE {

            /* TODO: should use command options */

            println!("add_block [data] - append a block into the local blockchain");
            println!("Example: add_block 10 \n");
            println!("send - send a copy of the blockchain to another node");
            println!("receive - receive a copy of the blockchain from another node");
            println!("list - list the local chain blocks");
        }
    }
}
