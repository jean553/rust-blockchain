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

#[derive(Serialize, Deserialize)]
struct HashContent {
    timestamp: i64,
    data: i32,
}

#[derive(Serialize, Deserialize)]
struct Block {
    content: HashContent,
    previous: String,
    current: String,
}

impl HashContent {

    /// Creates a brand new hash content.
    ///
    /// Args:
    ///
    /// `data` - the data to store into the block hash content
    ///
    /// Returns:
    ///
    /// hash content with current timestamp and given data
    fn new(data: i32) -> HashContent {
        HashContent {
            timestamp: time::now_utc().to_timespec().sec,
            data: data,
        }
    }

    /// Getter of the timestamp.
    ///
    /// Returns:
    ///
    /// block creation timestamp
    fn get_timestamp(&self) -> i64 {
        self.timestamp
    }

    /// Getter of the data.
    ///
    /// Returns:
    ///
    /// block data
    fn get_data(&self) -> i32 {
        self.data
    }
}

impl Block {

    /// One block constructor. Creates the block from the given data and previous digest. Calculates its own hash digest.
    ///
    /// Args:
    ///
    /// `data` - the data of the block
    /// `previous` - the digest of the previous block (empty if genesis)
    ///
    /// Returns:
    ///
    /// new block
    fn new(
        data: i32,
        previous: String,
    ) -> Block {

        let content = HashContent::new(data);
        let bytes = bincode::serialize(&content).unwrap();
        let digest = sha1::Sha1::from(bytes).hexdigest();

        Block {
            content: content,
            previous: previous,
            current: digest,
        }
    }

    /// Getter of the current block hash digest.
    ///
    /// Returns:
    ///
    /// current block digest as string
    fn get_current(&self) -> &str {
        &self.current
    }

    /// Getter of the hashed content.
    ///
    /// Returns:
    ///
    /// block hashed content
    fn get_content(&self) -> &HashContent {
        &self.content
    }
}

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

/// Returns an address:port string from the user input. Refactored as used multiple times.
///
/// Args:
///
/// `height` - the terminal height
///
/// Returns:
///
/// bind address in "address:port" format
fn get_bind_address_from_input(height: u16) -> String {

    let input = get_input(height);
    let address = input.trim();

    const PORT: &str = "10000";

    format!(
        "{}:{}",
        address,
        PORT,
    ).to_string()
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

    let genesis = Block::new(0, String::new());
    let mut chain: Vec<Block> = vec![genesis];

    println!("{}", Goto(0, 2));
    println!("Genesis block has been generated.");

    loop {

        let input = get_input(height);

        const ADD_BLOCK_CHOICE: &str = "add_block";
        const SEND_BLOCKCHAIN_CHOICE: &str = "send";
        const RECEIVE_BLOCKCHAIN_CHOICE: &str = "receive";
        const SEE_BLOCKCHAIN_CHOICE: &str = "list";
        const HELP_CHOICE: &str = "help";

        if input == ADD_BLOCK_CHOICE {

            println!("Data of the block:");

            let input = get_input(height);
            let data: i32 = input.trim().parse().unwrap();

            let current_digest = chain.last()
                .unwrap()
                .get_current()
                .to_string();

            let block = Block::new(data, current_digest.clone());

            println!("One block has been added to the ledger.");
            println!("Current block digest: {}", block.get_current());

            chain.push(block);
        }
        else if input == SEND_BLOCKCHAIN_CHOICE {

            println!("Send blockchain to node at IP:");

            let bind_address = get_bind_address_from_input(height);
            let mut stream = TcpStream::connect(bind_address).unwrap();

            let bytes = serialize(&chain).unwrap();
            stream.write(&bytes).unwrap();
        }
        else if input == RECEIVE_BLOCKCHAIN_CHOICE {

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
        else if input == SEE_BLOCKCHAIN_CHOICE {

            for block in chain.iter() {

                let content = block.get_content();
                println!("Hash: {}", block.get_current());
                println!("Timestamp: {}", content.get_timestamp());
                println!("Data: {} \n\n", content.get_data());
            }
        }
        else if input == HELP_CHOICE {

            /* TODO: should use command options */

            println!("add_block - append a block into the local blockchain");
            println!("send - send a copy of the blockchain to another node");
            println!("receive - receive a copy of the blockchain from another node");
            println!("list - list the local chain blocks");
        }
    }
}
