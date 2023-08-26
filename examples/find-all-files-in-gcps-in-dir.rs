#![feature(array_windows)]

use cftkk::{
    actr::{ActorNode, ActrReader, Index, Vertex},
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

    if let Ok(entries) = fs::read_dir(&args[1]) {
        for entry in entries {
            if let Ok(dir_entry) = entry {
                if let Ok(file_type) = dir_entry.file_type() {
                    if file_type.is_file() {
                        if let Some(ext) = dir_entry.path().extension() {
                            if ext == "gcp" {
                                std::println!("Current Package: {:?}", dir_entry.path());
                                if let Ok(gcp) = GcpReader::new(fs::read(dir_entry.path()).unwrap())
                                {
                                    for resource in gcp.resource_entries() {
                                        if string_table.iter().any(|e| {
                                            e.0 == resource.name.to_string()
                                                && resource.data.to_vec() == e.2
                                        }) {
                                            copied_files.push(resource.name.to_string());
                                            continue;
                                        } else {
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

                                        if resource.tag == Tag::Actor
                                            && !resource.name.contains(".sys")
                                            && !resource.name.contains(".lit")
                                        {
                                            let actr = ActrReader::new(resource.data).unwrap();
                                            //println!("{} is currently being parsed", resource.name);

                                            let mut string = String::new();
                                            if let Ok(vertices) = actr.verticies() {
                                                for vertex in vertices {
                                                    string.push_str(&format!(
                                                        "v {} {} {}\n",
                                                        vertex.x, vertex.y, vertex.z
                                                    ));
                                                }
                                            } else {
                                                let nodes: Vec<ActorNode> = actr
                                                    .nodes()
                                                    .unwrap()
                                                    .filter(|node| node.vertex_offset != 0)
                                                    .collect::<Vec<ActorNode>>();

                                                if nodes.len() == 0 {
                                                    println!(
                                                        "Actor not currently supported: {}",
                                                        resource.name
                                                    );

                                                    continue;
                                                }

                                                for node in nodes {
                                                    if let Ok(vertices) = node.verticies() {
                                                        for vertex in vertices {
                                                            string.push_str(&format!(
                                                                "v {} {} {}\n",
                                                                vertex.x, vertex.y, vertex.z
                                                            ));
                                                        }
                                                    }
                                                }
                                            }

                                            if let Ok(texcoords) = actr.texcoords() {
                                                for texcoord in texcoords {
                                                    string.push_str(&format!(
                                                        "vt {} {}\n",
                                                        texcoord.x,
                                                        -(texcoord.y - 1.0)
                                                    ));
                                                }
                                            } else {
                                                let nodes: Vec<ActorNode> = actr
                                                    .nodes()
                                                    .unwrap()
                                                    .filter(|node| node.texcoord_ofset != 0)
                                                    .collect::<Vec<ActorNode>>();

                                                if nodes.len() == 0 {
                                                    println!(
                                                        "Actor not currently supported: {}",
                                                        resource.name
                                                    );

                                                    continue;
                                                }

                                                for node in nodes {
                                                    if let Ok(texcoords) = node.texcoords() {
                                                        for texcoord in texcoords {
                                                            string.push_str(&format!(
                                                                "vt {} {}\n",
                                                                texcoord.x,
                                                                -(texcoord.y - 1.0)
                                                            ));
                                                        }
                                                    }
                                                }
                                            }

                                            if let Ok(indices) = actr.indexes() {
                                                let mut curr_group_num = 0;
                                                let mut first_run = true;
                                                for &[one, two, three] in indices
                                                    .collect::<Vec<(Index, usize)>>()
                                                    .array_windows()
                                                {
                                                    if curr_group_num != one.1 {
                                                        curr_group_num = one.1;

                                                        if !first_run {
                                                            let find = string
                                                                .find(&format!(
                                                                    "f {}",
                                                                    one.0.pos_idx - 1
                                                                ))
                                                                .unwrap_or(string.len());
                                                            string = string[..find].to_string();
                                                        }
                                                        first_run = false;

                                                        string.push_str(&format!(
                                                            "g part_{}\n",
                                                            curr_group_num
                                                        ));
                                                    }

                                                    string.push_str(&format!(
                                                        "f {}/{} {}/{} {}/{}\n",
                                                        one.0.pos_idx + 1,
                                                        one.0.texcoord_idx + 1,
                                                        two.0.pos_idx + 1,
                                                        two.0.texcoord_idx + 1,
                                                        three.0.pos_idx + 1,
                                                        three.0.texcoord_idx + 1,
                                                    ));
                                                }
                                            } else {
                                                let nodes: Vec<ActorNode> = actr
                                                    .nodes()
                                                    .unwrap()
                                                    .filter(|node| node.display_list_offset != 0)
                                                    .collect::<Vec<ActorNode>>();

                                                if nodes.len() == 0 {
                                                    println!(
                                                        "Actor not currently supported: {}",
                                                        resource.name
                                                    );

                                                    continue;
                                                }

                                                for node in nodes {
                                                    if let Ok(indicies) = node.indexes() {
                                                        let mut curr_group_num = 0;
                                                        let mut first_run = true;
                                                        for &[one, two, three] in indicies
                                                            .collect::<Vec<(Index, usize)>>()
                                                            .array_windows()
                                                        {
                                                            if curr_group_num != one.1 {
                                                                curr_group_num = one.1;

                                                                if !first_run {
                                                                    let find = string
                                                                        .find(&format!(
                                                                            "f {}",
                                                                            one.0.pos_idx - 1
                                                                        ))
                                                                        .unwrap_or(string.len());
                                                                    string =
                                                                        string[..find].to_string();
                                                                }
                                                                first_run = false;

                                                                string.push_str(&format!(
                                                                    "g part_{}\n",
                                                                    curr_group_num
                                                                ));
                                                            }

                                                            string.push_str(&format!(
                                                                "f {}/{} {}/{} {}/{}\n",
                                                                one.0.pos_idx + 1,
                                                                one.0.texcoord_idx + 1,
                                                                two.0.pos_idx + 1,
                                                                two.0.texcoord_idx + 1,
                                                                three.0.pos_idx + 1,
                                                                three.0.texcoord_idx + 1,
                                                            ));
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        /* for str in string_table {
            println!("Name: {}, Tag: {:?}", str.0, str.1);
        }

        println!("COPIED FILES:");
        for str in copied_files {
            println!("{str}");
        }

        println!(
            "File Count: {}, Unique File Count: {}",
            file_count, unique_file_count
        ); */
    }
}
