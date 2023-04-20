use cftkk::fetm::FetmReader;
use std::{env, fs};

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() < 1 {
        eprintln!("usage: {} <fetm.fetm>", args[0]);
    }

    let data = fs::read(&args[1]).unwrap();
    let fetm = FetmReader::new(&data).unwrap();

    for token in fetm.tokens() {
        println!("{:?}", token);
    }
}
