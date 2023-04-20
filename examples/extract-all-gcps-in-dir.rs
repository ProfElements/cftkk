use cftkk::gcp::GcpReader;
use std::{env, path::PathBuf};

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

                //println!("{}", entry.path().display());

                if entry.path().extension().unwrap() == "gcp" {
                    if let Ok(gcp) = GcpReader::new(std::fs::read(entry.path()).unwrap()) {
                        let mut path = PathBuf::from(entry.path());
                        path.set_extension("");
                        std::fs::create_dir(&path).unwrap();
                        for file in gcp.resource_entries() {
                            let _ = std::fs::write(path.join(file.name), file.data);
                        }
                    } else {
                        if let Err(err) = GcpReader::new(std::fs::read(entry.path()).unwrap()) {
                            println!("name: {}, {:?}", entry.path().display(), err);
                        }
                    }
                }
            }
        }
    }
}
