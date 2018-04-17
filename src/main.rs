extern crate time;
extern crate sha1;

struct Block {
    timestamp: time::Tm,
    data: i32,
    previous: sha1::Sha1,
}

fn main() {
}
