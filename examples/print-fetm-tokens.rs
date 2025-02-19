use cftkk::fetm::objectdb::{Node, Token};
use std::{env, fs};

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() < 1 {
        eprintln!("usage: {} <fetm.fetm>", args[0]);
    }

    let data = fs::read(&args[1]).unwrap();
    let reader = cftkk::fetm::objectdb::Reader::new(data).unwrap();

    let tokens: Vec<_> = reader.tokens().unwrap().skip(4).collect();

    let idx = tokens
        .iter()
        .position(|token| {
            let Token::String(kind) = token else {
                return false;
            };
            *kind == "sector"
        })
        .unwrap();

    let mut idx = idx;
    let node = Node::from_tokens(&tokens[idx..]);

    println!("{node:?}");
    let iter = core::iter::successors(Some(node), |node: &Node| {
        idx += node.len();

        let Token::String(end) = tokens[idx] else {
            return None;
        };

        if end == "end" {
            return None;
        }

        let node = Node::from_tokens(&tokens[idx..]);
        println!("{node:?}");
        Some(node)
    });

    for _ in iter {}
}
