#![feature(array_windows, array_chunks)]

use std::{env, fs};

use cftkk::actr::{ActorNode, ActrReader, Index};

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() < 1 {
        eprintln!("usage: {} <collision_mesh>", args[0]);
    }

    let data = fs::read(&args[1]).unwrap();

    let actr = ActrReader::new(data).unwrap();

    let mut vertexes = Vec::new();

    let mut string = String::new();
    if let Ok(vertices) = actr.verticies() {
        for vertex in vertices {
            string.push_str(&format!("v {} {} {}\n", vertex.x, vertex.y, vertex.z));
        }
    } else {
        let nodes: Vec<ActorNode> = actr
            .nodes()
            .unwrap()
            .filter(|node| node.vertex_offset != 0)
            .collect::<Vec<ActorNode>>();

        if nodes.len() == 0 {
            println!("Actor not currently supported: {}", &args[1]);
            return;
        }

        for node in nodes {
            if let Ok(vertices) = node.verticies() {
                for vertex in vertices {
                    string.push_str(&format!("v {} {} {}\n", vertex.x, vertex.y, vertex.z));
                    vertexes.push(vertex)
                }
            }
        }
    }

    if let Ok(texcoords) = actr.texcoords() {
        for texcoord in texcoords {
            string.push_str(&format!("vt {} {}\n", texcoord.x, -(texcoord.y - 1.0)));
        }
    } else {
        let nodes: Vec<ActorNode> = actr
            .nodes()
            .unwrap()
            .filter(|node| node.texcoord_ofset != 0)
            .collect::<Vec<ActorNode>>();

        if nodes.len() == 0 {
            println!("Actor not currently supported: {}", &args[1]);
            return;
        }

        for node in nodes {
            if let Ok(texcoords) = node.texcoords() {
                for texcoord in texcoords {
                    string.push_str(&format!("vt {} {}\n", texcoord.x, -(texcoord.y - 1.0)));
                }
            }
        }
    }

    if let Ok(indices) = actr.indexes() {
        let mut curr_group_num = 0;
        let mut first_run = true;
        for &[one, two, three] in indices.collect::<Vec<(Index, usize)>>().array_windows() {
            if curr_group_num != two.1 {
                curr_group_num = two.1;

                if !first_run {
                    let find = string
                        .rfind(string.lines().last().unwrap())
                        .unwrap_or(string.len());
                    string = string[..find].to_string();
                }
                first_run = false;

                string.push_str(&format!("g part_{}\n", curr_group_num));
                continue;
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
            println!("Actor not currently supported: {}", &args[1]);
            return;
        }

        let mut indexes = Vec::new();
        let mut group_sum = 0;
        for node in nodes {
            if let Ok(indices) = node.indexes() {
                for index in indices {
                    indexes.push(index);
                }
            }
        }

        let mut curr_group_num = 20;
        let mut group_sum = 0;
        let mut first_run = true;
        for &[one, two, three] in indexes.array_windows() {
            if curr_group_num != one.1 {
                curr_group_num = one.1;
                string.push_str(&format!("g part_{}\n", group_sum));

                group_sum += 1;
            }

            string.push_str(&format!(
                "f {} {} {}\n",
                one.0.pos_idx + 1,
                //one.0.color_idx + 1,
                two.0.pos_idx + 1,
                //two.0.color_idx + 1,
                three.0.pos_idx + 1,
                //three.0.color_idx + 1,
            ));
        }
    }

    let _ = fs::write(format!("{}.obj", &args[1]), string);
}
