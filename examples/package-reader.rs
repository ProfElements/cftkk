use std::env;
use std::fs;
use std::fs::create_dir_all;
use std::ptr::read;

use cftkk::collision_mesh;
use cftkk::resource::Kind;
use cftkk::resource::ResourceInfo;

const fn init_crc_table() -> [u32; 256] {
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
fn main() {
    let table = init_crc_table();
    let args = env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        eprintln!("usage: {} <package>", args[0]);
    }

    let data = fs::read(&args[1]).unwrap();
    let data_crc = checksum(&table, &data[4..]);
    let gcp = cftkk::package::Reader::new(data).unwrap();

    let package_crc = gcp.header().crc >> 1;

    println!("{package_crc} : {data_crc}");

    for file in gcp.files() {
        let file_name = file.name.to_str().unwrap();
        let file_size = file.data.len();
        let file_crc = file.crc;

        let mut file_tag = if file_name.as_bytes().contains(&b"."[0]) {
            Kind::None
        } else {
            Kind::try_from(&file.tag.tag).unwrap()
        };

        let mut file_kind = Kind::None;

        if file.data.len() > ResourceInfo::LENGTH {
            if let Ok(info) =
                ResourceInfo::from_bytes(&file.data[0..ResourceInfo::LENGTH].try_into().unwrap())
            {
                let kind = info.resource_kind;
                file_kind = kind;
            }
        }

        if file_kind != file_tag {
            file_kind = file_tag
        }

        if file_name == "TagTable.pak.sys" || file_name == "dummy" {
            file_tag = Kind::None;
            println!("{:?}", core::str::from_utf8(file.data));
        }

        match file_tag {
            Kind::CollisionMesh => {
                collision_mesh_info(file.data);
            }
            _ => (),
        }

        println!("{file_name} : {file_kind:?} : {file_size} : {file_tag:?} : {file_crc}");
    }
}

pub fn collision_mesh_info(data: &[u8]) {
    let reader = collision_mesh::Reader::new(data).unwrap();
    let kind = reader.header().kind;
    println!("{kind:?}");
    println!("{:?}", reader.header().cell_node_root_offset);
    println!("{:?}", reader.header());

    if kind == collision_mesh::Kind::AabbTree {
        let node = collision_mesh::Node::from_bytes(
            reader.data()[reader.header().cell_node_root_offset as usize
                ..reader.header().cell_node_root_offset as usize + collision_mesh::Node::LENGTH]
                .try_into()
                .unwrap(),
        );
        println!("{node:?}");
    }
}
