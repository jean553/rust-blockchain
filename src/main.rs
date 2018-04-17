extern crate time;
extern crate sha1;
extern crate bincode;

extern crate serde;
#[macro_use] extern crate serde_derive;

#[derive(Serialize)]
struct Block {
    timestamp: i64,
    data: i32,
    previous: [u8; 20],
}

fn main() {

    /* generate the genesis block */

    let chain = Block {
        timestamp: time::now_utc().to_timespec().sec,
        data: 0,
        previous: [0; 20],
    };

    println!("Genesis block has been generated.");

    /* add one block to the ledger: serialize the previous object into raw bytes,
       in order to generate a new hash digest from those bytes,
       and use that digest as a previous field content for the new block */

    let previous_bytes = bincode::serialize(&chain).unwrap();
    let previous_digest = sha1::Sha1::from(previous_bytes).digest().bytes();

    let _chain = Block {
        timestamp: time::now_utc().to_timespec().sec,
        data: 10,
        previous: previous_digest,
    };

    println!("One block has been added to the ledger.");
}
