#![feature(array_windows, array_chunks)]

use std::{env, fs};

use cftkk::actr::{experimental, ActrReader};

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() < 1 {
        eprintln!("usage: {} <collision_mesh>", args[0]);
    }

    let _ = export_actor(&args[1]);
}
pub fn export_actor(name: &'_ str) -> Result<(), ()> {
    let data = fs::read(name).unwrap();

    let data_start = 0;
    let data_end = data_start + 0xF4;

    let data_bytes = data.get(data_start..data_end).ok_or(())?;

    let mut obj = String::new();
    let actor: experimental::Actor<&[u8]> =
        experimental::Actor::from_bytes(data_bytes.try_into().unwrap(), &data);
    println!("{}", actor.vertex_type);
    let skin = actor.soft_skin();
    let uses_skin = skin.vertex_offset != 0
        && skin.normal_offset != 0
        && skin.texture_coord_offset != 0
        && skin.color_offset != 0;
    match uses_skin {
        true => {
            let vertices = skin.positions_from_buffer(actor.data);
            let texcoords = skin.texcoords_from_buffer(actor.data, actor.vertex_type);
            let display_list_parts =
                skin.display_list_parts_from_buffer(actor.data, actor.vertex_type);
            obj.push_str(format!("o skin\n",).as_str());
            for vertex in vertices {
                obj.push_str(format!("v {} {} {}\n", vertex.x, vertex.y, vertex.z).as_str());
            }

            for texcoord in texcoords {
                obj.push_str(format!("vt {} {}\n", texcoord.x, -(texcoord.y - 1.0)).as_str());
            }

            for display_list_part in display_list_parts {
                for n in 0..display_list_part.1.len() - 2 {
                    let (p_0_idx, _, _, t_0_idx) = display_list_part.1[n];
                    let (p_1_idx, _, _, t_1_idx) = display_list_part.1[n + 1];
                    let (p_2_idx, _, _, t_2_idx) = display_list_part.1[n + 2];
                    obj.push_str(
                        format!(
                            "f {}/{} {}/{} {}/{}\n",
                            p_0_idx + 1,
                            t_0_idx + 1,
                            p_1_idx + 1,
                            t_1_idx + 1,
                            p_2_idx + 1,
                            t_2_idx + 1
                        )
                        .as_str(),
                    );
                }
            }
        }
        false => {
            for root_node in actor.nodes() {
                let node_name = root_node.name_from_buffer(actor.data);
                let mesh = root_node.actor_info.mesh;

                let vertices = mesh.positions_from_buffer(actor.data);
                let texcoords = mesh.texcoords_from_buffer(actor.data);

                let display_list_parts = mesh.display_list_parts_from_buffer(actor.data);

                obj.push_str(format!("o {}\n", node_name).as_str());
                for vertex in vertices {
                    obj.push_str(format!("v {} {} {}\n", vertex.x, vertex.y, vertex.z).as_str());
                }

                for texcoord in texcoords {
                    obj.push_str(format!("vt {} {}\n", texcoord.x, -(texcoord.y - 1.0)).as_str());
                }

                for display_list_part in display_list_parts {
                    for n in 0..display_list_part.1.len() - 2 {
                        let (p_0_idx, _, _, t_0_idx) = display_list_part.1[n];
                        let (p_1_idx, _, _, t_1_idx) = display_list_part.1[n + 1];
                        let (p_2_idx, _, _, t_2_idx) = display_list_part.1[n + 2];
                        obj.push_str(
                            format!(
                                "f {}/{} {}/{} {}/{}\n",
                                p_0_idx + 1,
                                t_0_idx + 1,
                                p_1_idx + 1,
                                t_1_idx + 1,
                                p_2_idx + 1,
                                t_2_idx + 1
                            )
                            .as_str(),
                        );
                    }
                }
            }
        }
    }

    let _ = fs::write(format!("{}.obj", name), obj);
    Ok(())
}
