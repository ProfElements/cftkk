use cftkk::fetm::objectdb::{
    transform::{Bounds, Transform},
    Node, Token,
};
use std::{env, fs};

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() < 1 {
        eprintln!("usage: {} <fetm.fetm>", args[0]);
    }

    let data = fs::read(&args[1]).unwrap();
    let reader = cftkk::fetm::objectdb::Reader::new(data).unwrap();

    let tokens: Vec<_> = reader.tokens().unwrap().skip(4).collect();

    let idx = tokens
        .iter()
        .position(|token| {
            let Token::String(kind) = token else {
                return false;
            };
            *kind == "sector"
        })
        .unwrap();

    let mut idx = idx;
    let node = Node::from_tokens(&tokens[idx..]).unwrap();

    let _ = std::fs::create_dir("./test");
    println!("{node:?}");
    let iter = core::iter::successors(Some(node), |node: &Node| {
        idx += node.len();

        let mut obj = String::new();
        let Token::String(end) = tokens[idx] else {
            return None;
        };

        if end == "end" {
            return None;
        }

        let node = Node::from_tokens(&tokens[idx..])?;
        println!("{node:?}");

        if node.kind != "dummy"
            && node.kind != "refpoint"
            && node.kind != "group"
            && node.kind != "condition"
            && node.kind != "sector"
            && node.kind != "room"
            || node.kind == "prop"
            || node.kind == "simulation_object"
        {
            let _sub_node_type = node.tokens[0];
            let _transform_type = node.tokens[1];
            let transform = &node.tokens[2..12];

            if let Token::U8(pause_when_room_not_visble_flag) = node.tokens[12] {
                println!(
                    "PAUSE_WHEN_ROOM_NOT_VISIBLE = {}",
                    pause_when_room_not_visble_flag > 0
                )
            } else {
                println!("{:?}", &node.tokens[12]);
            }

            let bounds = &node.tokens[13..19];
            let mut rest_of_tokens = &node.tokens[19..];

            if let Token::U8(registered) = rest_of_tokens[0] {
                println!("REGISTERED = {}", registered > 0)
            }

            if let Token::U8(missing_entity_class) = rest_of_tokens[1] {
                println!("MISSING_ENTITY_CLASS = {}", missing_entity_class > 0)
            }

            if let Token::U8(timer_active) = rest_of_tokens[2] {
                println!("TIMER_ACTIVE = {}", timer_active > 0)
            }

            if let Token::U8(pause_when_not_visble) = rest_of_tokens[3] {
                println!("PAUSE_WHEN_NOT_VISIBLE = {}", pause_when_not_visble > 0)
            }

            if let Token::Hex8(unknown_flag_pile) = rest_of_tokens[4] {
                println!("UNKNOWN_FLAG_SET = {unknown_flag_pile:X}");
            }

            if let Token::U8(is_static) = rest_of_tokens[5] {
                println!("IS_STATIC = {}", is_static > 0)
            }

            if let Token::F32(node_timer_start_time) = rest_of_tokens[6] {
                println!("NODE_TIMER = {}", node_timer_start_time)
            }

            if let Token::F32(node_timer_start_time) = rest_of_tokens[6] {
                println!("NODE_TIMER = {}", node_timer_start_time)
            }

            if let Token::U8(node_counter) = rest_of_tokens[7] {
                println!("NODE_COUNTER = {}", node_counter)
            }

            if let Token::U8(is_advanced_node) = rest_of_tokens[8] {
                println!("IS_ADVANCED = {}", is_advanced_node > 0)
            }

            if let Token::Hex8(node_room_offset) = rest_of_tokens[9] {
                println!("ROOM_OFFSET = {}", node_room_offset)
            }

            if let Token::U8(action_list_enabled) = rest_of_tokens[10] {
                println!(
                    "HAS_ACTION_LIST | ACTION_LIST_ENABLED = {}",
                    action_list_enabled > 0
                )
            }

            if let Token::U8(custom_bounding_box) = rest_of_tokens[11] {
                println!("HAS_CUSTOM_BOUNDING_BOX = {}", custom_bounding_box > 0)
            }

            if let Token::U8(pause_when_not_loaded) = rest_of_tokens[12] {
                println!("PAUSE_WHEN_NOT_LOADED = {}", pause_when_not_loaded > 0)
            }

            if let Token::U8(pause_when_sector_not_current) = rest_of_tokens[13] {
                println!(
                    "PAUSE_WHEN_SECTOR_NOT_CURRENT = {}",
                    pause_when_sector_not_current > 0
                )
            }

            if let Token::U8(pause_when_room_not_current) = rest_of_tokens[13] {
                println!(
                    "PAUSE_WHEN_ROOM_NOT_CURRENT = {}",
                    pause_when_room_not_current > 0
                )
            }

            if let Token::U8(pause_when_sector_not_visible) = rest_of_tokens[14] {
                println!(
                    "PAUSE_WHEN_SECTOR_NOT_VISIBLE = {}",
                    pause_when_sector_not_visible > 0
                )
            }

            if let Token::U8(load_sector) = rest_of_tokens[15] {
                println!("SHOULD_LOAD_SECTOR = {}", load_sector > 0)
            }

            if let Token::U8(action_entry_count) = rest_of_tokens[18] {
                println!("ACTION_LIST_ENTRY_COUNT = {}", action_entry_count);
                if action_entry_count > 0 {
                    println!("Currently dont handle action list entries");
                    return None;
                }
            }

            if let Token::U8(has_attachment) = rest_of_tokens[19] {
                println!("HAS_ATTACHMENT = {}", has_attachment > 0);

                if has_attachment > 0 {
                    println!("Currently dont handle extra attachment details");

                    return None;
                }
            }

            let rest_of_tokens = &rest_of_tokens[19..];
            println!("{:?}", &rest_of_tokens);
            let transform = Transform::from_tokens(&transform)?;
            let bounds = Bounds::from_tokens(&bounds)?;

            obj.push_str(format!("o {}\n", node.name).as_str());
            println!("transform: {:?}", transform);
            println!("bounds: {:?}", bounds);

            let pos_x = transform.position.x;
            let pos_y = transform.position.y;
            let pos_z = transform.position.z;

            obj.push_str(
                format!(
                    "v {} {} {}\n",
                    transform.position.x, transform.position.y, transform.position.z
                )
                .as_str(),
            );
            obj.push_str(
                format!(
                    "v {} {} {}\n",
                    pos_x + bounds.min.x,
                    pos_y + bounds.min.y,
                    pos_z + bounds.min.z
                )
                .as_str(),
            );
            obj.push_str(
                format!(
                    "v {} {} {}\n",
                    pos_x + bounds.max.x,
                    pos_y + bounds.max.y,
                    pos_z + bounds.max.z
                )
                .as_str(),
            );
            obj.push_str("\n");

            let name = format!("./test/{}.obj", node.name);
            let _ = std::fs::write(name, obj.as_bytes());
        }
        Some(node)
    });

    for _ in iter {}
}
