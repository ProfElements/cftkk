use core::ffi::CStr;
use std::{env, fs};

use cftkk::actr::ActrReader;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() < 1 {
        eprintln!("usage: {} <collision_mesh>", args[0]);
    }

    let data = fs::read(&args[1]).unwrap();

    let actr = ActrReader::new(data).unwrap();

    for node in actr.nodes().unwrap() {
        let cstr =
            CStr::from_bytes_until_nul(actr.data().get(node.name_offset as usize..).unwrap())
                .unwrap();

        println!(
            "Name:{cstr:?}, next_node_offset: {:X}, {node:?}",
            node.next_node_offset
        );
    }

    println!("{}", actr.nodes().unwrap().len())
}
