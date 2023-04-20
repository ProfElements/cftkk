use cftkk::{fetm::FetmReader, gcp::GcpReader};
use std::{env, fs};

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() < 1 {
        eprintln!("usage: {} <package.gcp>", args[0]);
    }

    let data = fs::read(&args[1]).unwrap();
    let gcp = GcpReader::new(data).unwrap();

    for resource in gcp.resource_entries() {
        if resource.name.contains(".fetm") {
            let fetm = FetmReader::new(resource.data).unwrap();

            for token in fetm.tokens() {
                println!("{:?}", token);
            }
        }
    }
}
