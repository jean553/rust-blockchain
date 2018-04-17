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

fn main() {

    let mut chain = Blockchain {
        timestamp: time::now_utc().to_timespec().sec,
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

            let previous_bytes = bincode::serialize(&chain).unwrap();
            let previous_digest = sha1::Sha1::from(previous_bytes).digest().bytes();

            chain = Blockchain {
                timestamp: time::now_utc().to_timespec().sec,
                data: 10,
                previous: previous_digest,
            };

            println!("One block has been added to the ledger.");
        }
    }
}
