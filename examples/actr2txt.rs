#![feature(array_windows, array_chunks)]

use std::{env, fs};

use cftkk::actr::ActrReader;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() < 1 {
        eprintln!("usage: {} <collision_mesh>", args[0]);
    }

    let data = fs::read(&args[1]).unwrap();

    let actr = ActrReader::new(data).unwrap();

    let mut use_nodes = false;

    if actr.header().vertex_offset == 0
        && actr.header().normal_offset == 0
        && actr.header().texcoord_offset == 0
        && actr.header().color_offset == 0
    {
        use_nodes = true;
    }

    match use_nodes {
        true => {
            let mut node_count = 0;
            if let Ok(nodes) = actr.nodes() {
                let mut vertex_offset = 0;
                let mut texcoord_offset = 0;

                let mut obj = String::new();
                for node in nodes {
                    obj.push_str(format!("g node_{}\n", node_count).as_str());
                    node_count += 1;

                    if let Ok(verticies) = node.verticies() {
                        for vertex in verticies {
                            obj.push_str(
                                format!("v {} {} {}\n", vertex.x, vertex.y, vertex.z).as_str(),
                            );
                        }
                    }

                    if let Ok(texcoords) = node.texcoords() {
                        for texcoord in texcoords {
                            obj.push_str(
                                format!("vt {} {}\n", texcoord.x, -(texcoord.y - 1.0)).as_str(),
                            );
                        }
                    }

                    if let Ok(indicies) = node.indexes() {
                        let mut curr_group = 1;
                        for &[one, two, three] in indicies.collect::<Vec<_>>().array_windows() {
                            if one.1 != curr_group {
                                curr_group = one.1;

                                let find = obj
                                    .rfind(
                                        obj.lines().collect::<Vec<_>>()
                                            [obj.lines().collect::<Vec<_>>().len() - 2],
                                    )
                                    .unwrap();

                                obj = obj[..find].to_string();
                            }

                            obj.push_str(
                                format!(
                                    "f {} {} {}\n",
                                    one.0.pos_idx as usize + 1 + vertex_offset,
                                    two.0.pos_idx as usize + 1 + vertex_offset,
                                    three.0.pos_idx as usize + 1 + vertex_offset,
                                )
                                .as_str(),
                            );
                        }
                    }

                    vertex_offset += node.verticies().unwrap().collect::<Vec<_>>().len();

                    if let Ok(texcoords) = node.texcoords() {
                        texcoord_offset += texcoords.collect::<Vec<_>>().len();
                    }
                }

                let _ = fs::write(format!("{}.obj", &args[1]), obj);
            }
        }
        false => {
            let mut obj = String::new();
            if let Ok(verticies) = actr.verticies() {
                for vertex in verticies {
                    obj.push_str(format!("v {} {} {}\n", vertex.x, vertex.y, vertex.z).as_str());
                }
            }

            if let Ok(texcoords) = actr.texcoords() {
                for texcoord in texcoords {
                    obj.push_str(format!("vt {} {}\n", texcoord.x, -(texcoord.y - 1.0)).as_str());
                }
            }

            if let Ok(indicies) = actr.indexes() {
                let mut curr_group = 1;
                for &[one, two, three] in indicies.collect::<Vec<_>>().array_windows() {
                    if one.1 != curr_group {
                        curr_group = one.1;

                        let find = obj
                            .rfind(
                                obj.lines().collect::<Vec<_>>()
                                    [obj.lines().collect::<Vec<_>>().len() - 2],
                            )
                            .unwrap();

                        obj = obj[..find].to_string();
                        obj.push_str(format!("g node_{}\n", curr_group).as_str());
                    }

                    obj.push_str(
                        format!(
                            "f {}/{} {}/{} {}/{}\n",
                            one.0.pos_idx as usize + 1,
                            one.0.texcoord_idx + 1,
                            two.0.pos_idx as usize + 1,
                            two.0.texcoord_idx + 1,
                            three.0.pos_idx as usize + 1,
                            three.0.texcoord_idx + 1,
                        )
                        .as_str(),
                    );
                }
            }

            let _ = fs::write(format!("{}.obj", &args[1]), obj);
        }
    }
}
