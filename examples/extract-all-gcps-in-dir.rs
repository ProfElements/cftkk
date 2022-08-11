use cftkk::gcp::Gcp;
use std::{env, fs, path::PathBuf};

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() < 1 {
        eprintln!("usage: {} <package.gcp>", args[0]);
    }

    if let Ok(entries) = std::fs::read_dir(&args[1]) {
        for entry in entries {
            if let Ok(entry) = entry {
                if !entry.file_type().unwrap().is_file() {
                    continue;
                }

                println!("{}", entry.path().display());

                if entry.path().extension().unwrap() == "gcp" {
                    let gcp = Gcp::new(std::fs::read(entry.path()).unwrap()).unwrap();
                    let mut path = PathBuf::from(entry.path());
                    path.set_extension("");
                    std::fs::create_dir(&path).unwrap();
                    for file in gcp.get_files() {
                        std::fs::write(path.join(file.name), file.data).unwrap();
                    }
                }
            }
        }
    }
}
