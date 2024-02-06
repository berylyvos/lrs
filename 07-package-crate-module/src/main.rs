use rand::Rng;
// use std::io;
// use std::io::Write;
// use std::io::{self, Write};

fn main() {
    println!("{}", rand::thread_rng().gen_range(1..=0x64));
}
