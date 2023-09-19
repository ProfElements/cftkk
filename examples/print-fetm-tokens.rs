use cftkk::fetm::{FetmReader, Sector, TkKind, World};
use std::{env, fs};

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() < 1 {
        eprintln!("usage: {} <fetm.fetm>", args[0]);
    }

    let data = fs::read(&args[1]).unwrap();
    let fetm = FetmReader::new(&data).unwrap();

    let tokens: Vec<TkKind> = fetm.tokens().collect();

    let pos = tokens
        .iter()
        .position(|token| match token {
            TkKind::String(val) => val.as_bytes() == b"world",
            _ => false,
        })
        .unwrap();

    World::from_tokens(&tokens[pos + 1..]).unwrap();
    println!("{:?}", World::from_tokens(&tokens[pos + 1..]).unwrap());

    let pos = tokens
        .iter()
        .position(|token| match token {
            TkKind::String(val) => val.as_bytes() == b"sector",
            _ => false,
        })
        .unwrap();

    println!("{:?}", Sector::from_tokens(&tokens[pos + 1..]).unwrap());
}
