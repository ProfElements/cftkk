use cftkk::fetm::Fetm;
use std::{env, fs};

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() < 1 {
        eprintln!("usage: {} <fetm.fetm>", args[0]);
    }

    let data = fs::read(&args[1]).unwrap();
    let fetm = Fetm::new(data).unwrap();

    let tokens = fetm.collect_tokens();
    for token in &tokens {
        println!("{:?}", token);
    }
}
