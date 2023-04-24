use cftkk::{
    fetm::FetmReader,
    gcp::{GcpReader, Tag},
    texr::TexrReader,
};
use std::{env, fs};

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() < 1 {
        eprintln!("usage: {} <package.gcp>", args[0]);
    }

    let mut string_table: Vec<(String, Tag, Vec<u8>)> = Vec::new();
    let mut copied_files: Vec<String> = Vec::new();
    let mut file_count = 0;
    let mut unique_file_count = 0;

    if let Ok(entries) = fs::read_dir(&args[1]) {
        for entry in entries {
            if let Ok(dir_entry) = entry {
                if let Ok(file_type) = dir_entry.file_type() {
                    if file_type.is_file() {
                        if let Some(ext) = dir_entry.path().extension() {
                            if ext == "gcp" {
                                if let Ok(gcp) = GcpReader::new(fs::read(dir_entry.path()).unwrap())
                                {
                                    for resource in gcp.resource_entries() {
                                        file_count += 1;
                                        if string_table.iter().any(|e| {
                                            e.0 == resource.name.to_string()
                                                && resource.data.to_vec() == e.2
                                        }) {
                                            copied_files.push(resource.name.to_string());
                                            continue;
                                        } else {
                                            unique_file_count += 1;
                                            string_table.push((
                                                resource.name.to_string(),
                                                resource.tag.clone(),
                                                resource.data.to_vec(),
                                            ));
                                        }

                                        if resource.name.contains(".fetm") {
                                            if let Ok(fetm) = FetmReader::new(resource.data) {
                                                for token in fetm.tokens() {
                                                    // println!("{:?}", token);
                                                }
                                            }
                                        }
                                        if resource.tag == Tag::Texture
                                            && !resource.name.contains(".sys")
                                        {
                                            if let Ok(texr) = TexrReader::new(resource.data) {
                                                /* println!(
                                                    "Name: {}, Width: {}, Height: {}, Data length: {}. format: {:?}",
                                                    resource.name,
                                                    texr.header().width,
                                                    texr.header().height,
                                                    texr.image_data().len(),
                                                    texr.header().texr_format
                                                );*/
                                            }
                                        }

                                        if resource.tag != Tag::Texture
                                            && !resource.name.contains(".fetm")
                                        {
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        for str in string_table {
            println!("Name: {}, Tag: {:?}", str.0, str.1);
        }

        println!("COPIED FILES:");
        for str in copied_files {
            println!("{str}");
        }

        println!(
            "File Count: {}, Unique File Count: {}",
            file_count, unique_file_count
        );
    }
}
