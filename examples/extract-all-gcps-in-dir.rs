use cftkk::gcp::{GcpReader, Tag};
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

                if entry.path().extension().unwrap() == "gcp"
                    || entry.path().extension().unwrap() == "rev"
                {
                    if let Ok(gcp) = GcpReader::new(std::fs::read(entry.path()).unwrap()) {
                        let mut path = PathBuf::from(entry.path());
                        path.set_extension("");
                        let _ = std::fs::create_dir(&path);
                        for file in gcp.resource_entries() {
                            let mut name = String::from(file.name);

                            if file.name == "FilenameTable.pak.sys"
                                || file.name == "TagTable.pak.sys"
                            {
                                let _ = std::fs::write(path.join(file.name), file.data);
                                continue;
                            }

                            match file.tag {
                                Tag::Texture => {
                                    name.push_str(".texr");
                                    let _ = std::fs::write(path.join(name), file.data);
                                }
                                Tag::CollisionMesh => {
                                    name.push_str(".cmes");
                                    let _ = std::fs::write(path.join(name), file.data);
                                }
                                Tag::Actor => {
                                    name.push_str(".actr");
                                    let _ = std::fs::write(path.join(name), file.data);
                                }

                                _ => {
                                    let _ = std::fs::write(path.join(file.name), file.data);
                                }
                            }
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
