extern crate time;
extern crate sha1;

struct Block {
    timestamp: time::Tm,
    data: i32,
    previous: Option<sha1::Sha1>,
}

fn main() {

    /* generate the genesis block */
    let chain = Block {
        timestamp: time::now_utc(),
        data: 0,
        previous: None,
    };

    println!("Genesis block has been generated.");
}
