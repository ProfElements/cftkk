use cftkk::gcp::GcpReader;
use std::{env, fs, path::PathBuf};

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        eprintln!("usage: {} <package.gcp>", args[0]);
    }

    let data = fs::read(&args[1]).unwrap();
    let gcp = GcpReader::new(data).unwrap();

    let mut path = PathBuf::from(&args[1]);
    path.set_extension("");

    std::fs::create_dir(&path).unwrap();

    for resource in gcp.resource_entries() {
        std::fs::write(path.join(resource.name), resource.data).unwrap();
    }
}
