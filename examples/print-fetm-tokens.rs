use cftkk::fetm::{
    EntityClassHeader, FetmReader, NodeSimulation, Sector, TkKind, World, WorldNode,
};
use std::{env, fs};

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() < 1 {
        eprintln!("usage: {} <fetm.fetm>", args[0]);
    }

    let data = fs::read(&args[1]).unwrap();
    let fetm = FetmReader::new(&data).unwrap();

    let tokens: Vec<TkKind> = fetm.tokens().collect();

    let mut pos = tokens
        .iter()
        .position(|token| match token {
            TkKind::String(val) => val.as_bytes() == b"world",
            _ => false,
        })
        .unwrap();

    let world = World::from_tokens(&tokens[pos + 1..]).unwrap();
    println!("{:?}", world);

    let sector = Sector::from_tokens(&tokens[pos + 1 + World::LENGTH + 1..]).unwrap();
    println!("{:?}", sector);

    println!(
        "{:?}",
        &tokens[pos + 1 + World::LENGTH + 1 + Sector::LENGTH]
    );

    let world_node = WorldNode::from_tokens(&tokens[sector.first_node_offset + 6..]).unwrap();
    println!("{:?}", world_node);

    pos = tokens
        .iter()
        .position(|token| match token {
            TkKind::String(val) => val.as_bytes() == b"collision_node",
            _ => false,
        })
        .unwrap();

    let world_node_2 = WorldNode::from_tokens(&tokens[pos..]).unwrap();
    println!("should be collision: {:?}", world_node_2);

    pos = tokens
        .iter()
        .position(|token| match token {
            TkKind::String(val) => val.as_bytes() == b"dummy",
            _ => false,
        })
        .unwrap();

    let world_node_3 = WorldNode::from_tokens(&tokens[pos..]).unwrap();
    println!("should be dummy: {:?}", world_node_3);

    let world_node_3 = WorldNode::from_tokens(&tokens[pos + 6..]).unwrap();
    println!("should be simulation_object: {:?}", world_node_3);

    for _ in 0..sector.node_list_size {}
}
