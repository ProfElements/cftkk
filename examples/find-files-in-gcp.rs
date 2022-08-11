use cftkk::{fetm::Fetm, gcp::Gcp};
use std::{env, fs, path::PathBuf};

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() < 1 {
        eprintln!("usage: {} <package.gcp>", args[0]);
    }

    let data = fs::read(&args[1]).unwrap();
    let gcp = Gcp::new(data).unwrap();

    let mut path = PathBuf::from(&args[1]);
    path.set_extension("");

    //std::fs::create_dir(&path).unwrap();
    for files in gcp.get_files() {
        if files.name.contains(".fetm") {
            let fetm = Fetm::new(files.data).unwrap();

            for token in fetm.collect_tokens() {
                println!("{:?}", token);
            }
        }

        //std::fs::write(path.join(files.name), files.data).unwrap();
    }
}
