extern crate time;
extern crate sha1;
extern crate bincode;

extern crate serde;
#[macro_use] extern crate serde_derive;

use std::io::stdin;

#[derive(Serialize)]
struct Block {
    timestamp: i64,
    data: i32,
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

        let mut chain = Block {
            timestamp: get_current_timestamp(),
            data: 0,
            previous: String::new(),
            current: String::new(),
        };

        /* current block hash generation must be done after block creation */
        chain.current = chain.get_digest();

        chain
    }

    /// Adds a block into the blockchain. Encrypt the current block, stores it as the previous block, update the timestamp and the data.
    ///
    /// Args:
    ///
    /// `data` - data to insert into the new block.
    fn add_block(
        &mut self,
        data: i32,
    ) {

        self.previous = self.get_digest();
        self.timestamp = get_current_timestamp();
        self.data = data;
        self.current = self.get_digest();
    }

    /// Returns the hash digest of the current block.
    ///
    /// Returns:
    ///
    /// sha1 digest of the current block
    fn get_digest(&self) -> String {

        let bytes = bincode::serialize(&self).unwrap();
        sha1::Sha1::from(bytes).hexdigest()
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

    let mut block = Block::new();

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
            block.add_block(data);

            println!("One block has been added to the ledger.");
            println!("Current block digest: {}", block.get_current_digest());
        }
    }
}
