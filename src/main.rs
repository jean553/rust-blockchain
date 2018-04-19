extern crate time;
extern crate sha1;
extern crate bincode;

extern crate serde;
#[macro_use] extern crate serde_derive;

use std::io::stdin;

#[derive(Serialize)]
struct HashContent {
    timestamp: i64,
    data: i32,
}

#[derive(Serialize)]
struct Block {
    content: HashContent,
    previous: String,
    current: String,
}

impl Block {

    /// Constructor of the blockchain, creates the genesis block with an empty previous block digest.
    ///
    /// Returns:
    ///
    /// genesis block
    fn new() -> Block {

        let content = HashContent {
            timestamp: get_current_timestamp(),
            data: 0,
        };

        let hash = generate_hash(&content);

        let chain = Block {
            content: content,
            previous: String::new(),
            current: hash,
        };

        chain
    }

    /// Getter of the current block hash digest.
    ///
    /// Returns:
    ///
    /// current block digest as string
    fn get_current_digest(&self) -> &str {
        &self.current
    }
}

/// Generates the digest of a given hash content.
///
/// Args:
///
/// `content` - the content to process
///
/// Returns:
///
/// the hash digest as a string
fn generate_hash(content: &HashContent) -> String {

    let bytes = bincode::serialize(&content).unwrap();
    sha1::Sha1::from(bytes).hexdigest()
}

/// Refactor the current timestamp generation.
///
/// Returns:
///
/// the current timestamp
fn get_current_timestamp() -> i64 {
    time::now_utc().to_timespec().sec
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

fn main() {

    let mut chain: Vec<Block> = vec![Block::new()];

    println!("Genesis block has been generated.");

    loop {

        println!("\nChoices:");
        println!("1. Add a block");
        println!("2. Update blockchain");

        let input = get_input();
        let choice = input.as_bytes()[0];

        const ADD_BLOCK_CHOICE: u8 = 0x31;

        if choice == ADD_BLOCK_CHOICE {

            println!("Data of the block:");

            let input = get_input();
            let data: i32 = input.trim().parse().unwrap();

            let current_digest = chain.last()
                .unwrap()
                .get_current_digest()
                .to_string();

            let content = HashContent {
                timestamp: get_current_timestamp(),
                data: data,
            };

            let hash = generate_hash(&content);

            let block = Block {
                content: content,
                previous: current_digest.clone(),
                current: hash,
            };

            println!("One block has been added to the ledger.");
            println!("Current block digest: {}", current_digest);

            chain.push(block);
        }
    }
}
