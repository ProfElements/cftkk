#![feature(array_windows)]

use std::{
    env, fs,
    io::BufWriter,
    ops::Range,
    path::{Path, PathBuf},
};

use cftkk::{
    actr::{
        experimental::{self, Mesh, SoftSkin},
        ActrReader,
    },
    package::File,
    texr::{Format, TexrReader},
};
use gctex::TextureFormat;

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
    let mut mat = String::new();
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
            let batches = skin.batches_from_buffer(actor.data);
            let mut batch_crcs = Vec::with_capacity(batches.len() * 2);

            for batch in skin.batches_from_buffer(actor.data) {
                if batch.texture_1_crc != 0 {
                    batch_crcs.push(batch.texture_1_crc);
                }
                if batch.texture_2_crc != 0 {
                    batch_crcs.push(batch.texture_2_crc);
                }
            }
            let mut path_buf = PathBuf::new();
            path_buf.push(name);
            if let Some(mat_names) = cwd_crcs_try_match(&path_buf, &batch_crcs) {
                for name in mat_names {
                    write_texr_png(&path_buf.parent().unwrap().join(format!("{}.texr", name.0)));
                    mat.push_str(format!("newmtl {}\n", name.0).as_str());
                    mat.push_str(format!("map_Kd {}.texr.png\n", name.0).as_str());
                    mat.push_str(" \n");
                }
            };

            blah(&skin, actor.data);

            let vertices = skin.positions_from_buffer(actor.data);
            let texcoords = skin.texcoords_from_buffer(actor.data, actor.vertex_type);
            let display_list_parts =
                skin.display_list_parts_from_buffer(actor.data, actor.vertex_type);

            obj.push_str(format!("mtllib {}.mtl\n", name.split("/").last().unwrap()).as_str());
            obj.push_str(format!("o skin\n",).as_str());

            let material_ranges = blah(&skin, actor.data);
            let material_names = cwd_crcs_try_match(&path_buf, &batch_crcs);

            for vertex in vertices {
                obj.push_str(format!("v {} {} {}\n", vertex.x, vertex.y, vertex.z).as_str());
            }

            for texcoord in texcoords {
                obj.push_str(format!("vt {} {}\n", texcoord.x, -(texcoord.y - 1.0)).as_str());
            }

            let mut already_set_name = "";
            for display_list_part in display_list_parts {
                for n in 0..display_list_part.1.len() - 2 {
                    let (p_0_idx, _, _, t_0_idx) = display_list_part.1[n];
                    let (p_1_idx, _, _, t_1_idx) = display_list_part.1[n + 1];
                    let (p_2_idx, _, _, t_2_idx) = display_list_part.1[n + 2];

                    if let Some(idx) = material_ranges
                        .iter()
                        .position(|range| range.0.contains(&usize::try_from(p_0_idx).unwrap()))
                    {
                        let crc = material_ranges[idx].1;

                        if let Some(ref material_names) = material_names {
                            if let Some(name) = material_names
                                .iter()
                                .find(|material_names| material_names.1 == crc)
                            {
                                if already_set_name != name.0 {
                                    obj.push_str(format!("usemtl {}\n", name.0).as_str());
                                    already_set_name = &name.0;
                                }
                            }
                        }
                    }

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

                let batches = mesh.batches_from_buffer(actor.data);
                let mut batch_crcs = Vec::with_capacity(batches.len() * 2);

                for batch in mesh.batches_from_buffer(actor.data) {
                    if batch.texture_1_crc != 0 {
                        batch_crcs.push(batch.texture_1_crc);
                    }
                    if batch.texture_2_crc != 0 {
                        batch_crcs.push(batch.texture_2_crc);
                    }
                }

                let display_list_parts = mesh.display_list_parts_from_buffer(actor.data);

                obj.push_str(format!("mtllib {}.mtl\n", name).as_str());
                obj.push_str(format!("o {}\n", node_name).as_str());

                for vertex in vertices {
                    obj.push_str(format!("v {} {} {}\n", vertex.x, vertex.y, vertex.z).as_str());
                }

                for texcoord in texcoords {
                    obj.push_str(format!("vt {} {}\n", texcoord.x, -(texcoord.y - 1.0)).as_str());
                }
                let mut path_buf = PathBuf::new();
                path_buf.push(name);
                if let Some(mat_names) = cwd_crcs_try_match(&path_buf, &batch_crcs) {
                    for name in mat_names {
                        write_texr_png(
                            &path_buf.parent().unwrap().join(format!("{}.texr", name.0)),
                        );
                        mat.push_str(format!("newmtl {}\n", name.0).as_str());
                        mat.push_str(format!("map_Kd {}.texr.png\n", name.0).as_str());
                        mat.push_str(" \n");
                    }
                };

                let material_ranges = blah_2(&mesh, actor.data);
                let material_names = cwd_crcs_try_match(&path_buf, &batch_crcs);

                let mut already_set_name = "";
                for display_list_part in display_list_parts {
                    for n in 0..display_list_part.1.len() - 2 {
                        let (p_0_idx, _, _, t_0_idx) = display_list_part.1[n];
                        let (p_1_idx, _, _, t_1_idx) = display_list_part.1[n + 1];
                        let (p_2_idx, _, _, t_2_idx) = display_list_part.1[n + 2];

                        if let Some(idx) = material_ranges
                            .iter()
                            .position(|range| range.0.contains(&usize::try_from(p_0_idx).unwrap()))
                        {
                            let crc = material_ranges[idx].1;

                            if let Some(ref material_names) = material_names {
                                if let Some(name) = material_names
                                    .iter()
                                    .find(|material_names| material_names.1 == crc)
                                {
                                    if already_set_name != name.0 {
                                        obj.push_str(format!("usemtl {}\n", name.0).as_str());
                                        already_set_name = &name.0;
                                    }
                                }
                            }
                        }

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
    let _ = fs::write(format!("{}.mtl", name), mat);
    Ok(())
}

fn cwd_crcs_try_match(dir: &PathBuf, crcs: &Vec<u32>) -> Option<Vec<(String, u32)>> {
    println!("{dir:?}");
    let table = init_crc_table();
    let mut mat_names = Vec::new();
    if let Ok(reader) = std::fs::read_dir(dir.parent().unwrap()) {
        for entry in reader {
            if let Ok(entry) = entry {
                if !entry.file_type().unwrap().is_file() {
                    continue;
                }
                let name = Path::new(&entry.file_name().into_string().unwrap()).with_extension("");
                let bytes = checksum(&table, name.to_str().unwrap().as_bytes());
                if crcs.contains(&bytes) {
                    println!("{:?}", name);
                    mat_names.push((name.to_str().unwrap().to_owned(), bytes));
                }
            }
        }

        return Some(mat_names);
    }
    return None;
}

fn init_crc_table() -> [u32; 256] {
    let mut table = [0u32; 256];
    let mut i = 0;
    while i + 1 < 0x100 {
        let idx = i + 1;
        let mut size = 8;
        let mut data = i << 0x18;

        while size != 0 {
            if (data as i32) < 0 {
                data = data << 1 ^ 0x4c11db7;
            } else {
                data = data << 1;
            }
            size -= 1;
        }
        table[i as usize] = data;
        i = idx;
    }
    table
}

fn checksum(table: &[u32; 256], bytes: &[u8]) -> u32 {
    let mut init = 0x0000_0000 << (32u8 - 32);
    for i in 0..bytes.len() {
        let table_index = (((init >> 24) ^ bytes[i] as u32) & 0xFF) as usize;
        init = table[table_index] ^ (init << 8);
    }
    init = init >> (32u8 - 32);
    init = init ^ 0x0;
    init
}

fn blah<'a>(skin: &'a SoftSkin, buffer: &'a [u8]) -> Vec<(Range<usize>, u32)> {
    let mut vertex_indices_per_batch: Vec<(Range<usize>, u32)> = Vec::new();
    let mut vtx_offset = 0;
    let mut prim_offset = 0;
    let primitives = skin.primitives_from_buffer(buffer);
    for batch in skin.batches_from_buffer(buffer) {
        let end_offset = prim_offset + batch.number_of_primitives;

        let prims = &primitives[prim_offset as usize..end_offset as usize];

        let mut vtx_count = 0;
        for prim in prims {
            vtx_count += prim.number_of_vertices;
        }

        if batch.texture_1_crc != 0 {
            vertex_indices_per_batch.push((
                vtx_offset..(vtx_offset as usize + vtx_count as usize),
                batch.texture_1_crc,
            ));
        }
        prim_offset += batch.number_of_primitives;
        vtx_offset += vtx_count as usize;
    }

    vertex_indices_per_batch
}

fn blah_2<'a>(skin: &'a Mesh, buffer: &'a [u8]) -> Vec<(Range<usize>, u32)> {
    let mut vertex_indices_per_batch: Vec<(Range<usize>, u32)> = Vec::new();
    let mut vtx_offset = 0;
    let mut prim_offset = 0;
    let primitives = skin.primitives_from_buffer(buffer);
    for batch in skin.batches_from_buffer(buffer) {
        let end_offset = prim_offset + batch.number_of_primitives;

        let prims = &primitives[prim_offset as usize..end_offset as usize];

        let mut vtx_count = 0;
        for prim in prims {
            vtx_count += prim.number_of_vertices;
        }

        if batch.texture_1_crc != 0 {
            vertex_indices_per_batch.push((
                vtx_offset..(vtx_offset as usize + vtx_count as usize),
                batch.texture_1_crc,
            ));
        }
        prim_offset += batch.number_of_primitives;
        vtx_offset += vtx_count as usize;
    }

    vertex_indices_per_batch
}

fn write_texr_png(path: &PathBuf) {
    let data = fs::read(path).unwrap();

    let texr = TexrReader::new(data).unwrap();

    let mut dest_data = vec![
        0u8;
        (texr.header().width * texr.header().height * 4)
            .try_into()
            .unwrap_or(0)
    ];

    let gx_format = match texr.header().texr_format {
        Format::Rgba8 => 0x6,
        Format::Rgb5a3 => 0x5,
        Format::Cmpr => 0xE,
        Format::Rgb565 => 0x4,
        Format::I4 => 0x0,
        Format::I8 => 0x1,
        Format::Ci8Rgb565 | Format::Ci8Rgb5a3 => 0x9,
        Format::Ci4Rgb565 | Format::Ci4Rgb5a3 => 0x8,
    };

    let tlut_format = match texr.header().texr_format {
        Format::Ci8Rgb565 | Format::Ci4Rgb565 => 0x1,
        Format::Ci4Rgb5a3 | Format::Ci8Rgb5a3 => 0x2,
        _ => 0,
    };

    gctex::decode_into(
        &mut dest_data,
        texr.image_data(),
        texr.header().width,
        texr.header().height,
        TextureFormat::from_u32(gx_format).unwrap(),
        texr.texture_lookup_data().unwrap_or(&[]),
        tlut_format,
    );

    let mut encoder = png::Encoder::new(
        BufWriter::new(std::fs::File::create(format!("{}.png", path.display())).unwrap()),
        texr.header().width,
        texr.header().height,
    );

    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&dest_data).unwrap();
}
