extern crate time;
extern crate sha1;
extern crate bincode;

#[macro_use] extern crate serde;
#[macro_use] extern crate serde_derive;

#[derive(Serialize)]
struct Block {
    timestamp: i64,
    data: i32,
    previous: Option<[u8; 20]>,
}

fn main() {

    /* generate the genesis block */

    let chain = Block {
        timestamp: time::now_utc().to_timespec().sec,
        data: 0,
        previous: None,
    };

    println!("Genesis block has been generated.");

    /* add one block to the ledger: serialize the previous object into raw bytes,
       in order to generate a new hash digest from those bytes,
       and use that digest as a previous field content for the new block */

    let previous_block_bytes = bincode::serialize(&chain).unwrap();

    let chain = Block {
        timestamp: time::now_utc().to_timespec().sec,
        data: 10,
        previous: Some(sha1::Sha1::from(previous_block_bytes).digest().bytes()),
    };
}
