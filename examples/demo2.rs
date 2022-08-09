use cftkk::gcp::Gcp;
use std::{env, fs};

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() < 1 {
        eprintln!("usage: {} <package.gcp>", args[0]);
    }

    let data = fs::read(&args[1]).unwrap();
    let gcp = Gcp::new(data).unwrap();

    for files in gcp.get_files() {
        std::fs::write(files.name, files.data).unwrap();
    }
}
