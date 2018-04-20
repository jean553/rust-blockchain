extern crate time;
extern crate sha1;
extern crate bincode;

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

        let content = HashContent {
            timestamp: time::now_utc().to_timespec().sec,
            data: data,
        };

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
}

/// Handles user input and returns that input as a string.
///
/// Returns:
///
/// user input as string
fn get_input() -> String {

    let mut input = String::new();
    stdin().read_line(&mut input).expect("cannot read input");

    input
}

/// Returns an address:port string from the user input. Refactored as used multiple times.
///
/// Returns:
///
/// bind address in "address:port" format
fn get_bind_address_from_input() -> String {

    let input = get_input();
    let port = input.trim();

    const LOCALHOST: &str = "127.0.0.1";

    format!(
        "{}:{}",
        LOCALHOST,
        port,
    ).to_string()
}

fn main() {

    let genesis = Block::new(0, String::new());
    let mut chain: Vec<Block> = vec![genesis];

    println!("Genesis block has been generated.");

    loop {

        println!("\nChoices:");
        println!("1. Add a block");
        println!("2. Send blockchain");
        println!("3. Receive blockchain");

        let input = get_input();
        let choice = input.as_bytes()[0];

        const ADD_BLOCK_CHOICE: u8 = 0x31;
        const SEND_BLOCKCHAIN_CHOICE: u8 = 0x32;
        const RECEIVE_BLOCKCHAIN_CHOICE: u8 = 0x33;

        if choice == ADD_BLOCK_CHOICE {

            println!("Data of the block:");

            let input = get_input();
            let data: i32 = input.trim().parse().unwrap();

            let current_digest = chain.last()
                .unwrap()
                .get_current()
                .to_string();

            let block = Block::new(data, current_digest.clone());

            println!("One block has been added to the ledger.");
            println!("Current block digest: {}", current_digest);

            chain.push(block);
        }
        else if choice == SEND_BLOCKCHAIN_CHOICE {

            println!("Send blockchain to local instance at port:");

            let bind_address = get_bind_address_from_input();
            let mut stream = TcpStream::connect(bind_address).unwrap();

            let bytes = serialize(&chain).unwrap();
            stream.write(&bytes);
        }
        else if choice == RECEIVE_BLOCKCHAIN_CHOICE {

            println!("Receive blockchain on port:");

            let bind_address = get_bind_address_from_input();
            let listener = TcpListener::bind(bind_address).unwrap();

            println!("Waiting for connection...");

            let mut connection = listener.accept().unwrap();

            println!("Connection received.");

            let mut buffer: Vec<u8> = Vec::new();
            let mut stream = connection.0;

            stream.read_to_end(&mut buffer);

            let blockchain: Vec<Block> = deserialize(&buffer).unwrap();

            /* TODO: compare chains in order to replace it or not... */
        }
    }
}
