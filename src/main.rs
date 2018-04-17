extern crate time;
extern crate sha1;
extern crate bincode;

extern crate serde;
#[macro_use] extern crate serde_derive;

use std::io::stdin;

#[derive(Serialize)]
struct Blockchain {
    timestamp: i64,
    data: i32,
    previous: [u8; 20],
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

fn main() {

    let mut chain = Blockchain {
        timestamp: get_current_timestamp(),
        data: 0,
        previous: [0; 20],
    };

    println!("Genesis block has been generated.");

    loop {

        println!("\nChoices:");
        println!("1. Add a block");
        println!("2. Update blockchain");

        let mut input = String::new();
        stdin().read_line(&mut input).expect("cannot read input");

        let choice = input.as_bytes()[0];
        if choice == 0x31 {

            /* TODO: should take the user input, use 10 as an example */
            chain.add_block(10);

            println!("One block has been added to the ledger.");
        }
    }
}
