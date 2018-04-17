extern crate time;
extern crate sha1;
extern crate bincode;

extern crate serde;
#[macro_use] extern crate serde_derive;

use std::io::stdin;

const HASH_BYTES_SIZE: usize = 20;

#[derive(Serialize)]
struct Blockchain {
    timestamp: i64,
    data: i32,
    previous: [u8; HASH_BYTES_SIZE],
}

impl Blockchain {

    /// Adds a block into the blockchain. Encrypt the current block, stores it as the previous block, update the timestamp and the data.
    ///
    /// Args:
    ///
    /// `data` - data to insert into the new block.
    fn add_block(
        &mut self,
        data: i32,
    ) {

        let bytes = bincode::serialize(&self).unwrap();
        let digest = sha1::Sha1::from(bytes).digest().bytes();

        self.timestamp = get_current_timestamp();
        self.data = data;
        self.previous = digest;
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

    let mut chain = Blockchain {
        timestamp: get_current_timestamp(),
        data: 0,
        previous: [0; HASH_BYTES_SIZE],
    };

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
            chain.add_block(data);

            println!("One block has been added to the ledger.");
        }
    }
}
