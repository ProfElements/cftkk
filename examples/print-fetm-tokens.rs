use cftkk::fetm::{
    EntityClassHeader, FetmReader, Node, NodeSimulation, Sector, TkKind, World, WorldNode,
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

    println!("dummy tokens {:?}", &tokens[pos..pos + world_node_3.size()]);

    let world_node_3 = WorldNode::from_tokens(&tokens[pos + 6..]).unwrap();
    println!("should be simulation_object: {:?}", world_node_3);

    let world_node_4 =
        WorldNode::from_tokens(&tokens[pos + 6 + world_node_3.size() + 9..]).unwrap();
    println!("should be prop: {:?}", world_node_4);

    let world_node_5 = WorldNode::from_tokens(
        &tokens[pos + 6 + world_node_3.size() + 9 + world_node_4.size() + 3..],
    )
    .unwrap();

    println!("should be effect: {:?}", world_node_5);

    let world_node_6 = WorldNode::from_tokens(
        &tokens[pos
            + 6
            + world_node_3.size()
            + 9
            + world_node_4.size()
            + 3
            + world_node_5.size()
            + 2..],
    )
    .unwrap();
    println!("should be effect: {:?}", world_node_6);

    let mut base = pos
        + 6
        + world_node_3.size()
        + 9
        + world_node_3.size()
        + 9
        + world_node_4.size()
        + 3
        + world_node_5.size()
        + 2;
    println!(
        "func size: {:?}, actual size: {:?}",
        world_node_6.size(),
        world_node_6.size() - 10
    );

    println!(
        "expected tokens: {:?}",
        tokens[base + world_node_6.size() - 16..].as_ref()
    );

    println!(
        "CFWorldNodeParticleSystem: size: 55, {:?}",
        Node::from_name("effect", &tokens[base + world_node_6.size() - 9..]).unwrap()
    );

    let world_node_7 =
        WorldNode::from_tokens(tokens[base + world_node_6.size() + 55..].as_ref()).unwrap();
    println!("should be prop: {:?}", world_node_7);

    let world_node_8 = WorldNode::from_tokens(
        tokens[base + world_node_6.size() + 55 + world_node_7.size() + 2..].as_ref(),
    )
    .unwrap();

    println!("should be prop: {:?}", world_node_8,);

    let world_node_9 = WorldNode::from_tokens(
        tokens
            [base + world_node_6.size() + 55 + world_node_7.size() + 2 + world_node_8.size() + 3..]
            .as_ref(),
    )
    .unwrap();
    println!("should be effect: {:?}", world_node_9);

    let world_node_10 = WorldNode::from_tokens(
        tokens[base
            + world_node_6.size()
            + 55
            + world_node_7.size()
            + 2
            + world_node_8.size()
            + 3
            + world_node_9.size()
            + 1..]
            .as_ref(),
    )
    .unwrap();

    println!("should be prop:  {:?}", world_node_10);

    base += world_node_6.size()
        + 55
        + world_node_7.size()
        + 2
        + world_node_8.size()
        + 3
        + world_node_9.size()
        + 1;

    let world_node_11 =
        WorldNode::from_tokens(tokens[base + world_node_10.size() + 2..].as_ref()).unwrap();

    println!("should be prop: {:?}", world_node_11,);

    let world_node_12 = WorldNode::from_tokens(
        tokens[base + world_node_10.size() + 2 + world_node_11.size() + 2..].as_ref(),
    )
    .unwrap();

    println!("should be dummy: {:?}", world_node_12,);

    let world_node_13 = WorldNode::from_tokens(
        tokens[base
            + world_node_10.size()
            + 2
            + world_node_11.size()
            + 2
            + world_node_12.size()
            + 2..]
            .as_ref(),
    )
    .unwrap();
    println!("should be effect: {:?}", world_node_13.size());

    let world_node_14 = WorldNode::from_tokens(
        tokens[base
            + world_node_10.size()
            + 2
            + world_node_11.size()
            + 2
            + world_node_12.size()
            + 2
            + world_node_13.size()
            + 1..]
            .as_ref(),
    )
    .unwrap();

    println!("should be effect: {:?}", world_node_14);

    base += world_node_10.size()
        + 2
        + world_node_11.size()
        + 2
        + world_node_12.size()
        + 2
        + world_node_13.size()
        + 1
        + world_node_14.size();

    let world_node_15 = WorldNode::from_tokens(tokens[base + 1..].as_ref()).unwrap();
    println!("should be effect: {:?}", world_node_15);

    let world_node_16 =
        WorldNode::from_tokens(tokens[base + 1 + world_node_15.size() + 1..].as_ref()).unwrap();

    println!("should be effect: {:?}", world_node_16);

    let world_node_17 = WorldNode::from_tokens(
        tokens[base + 1 + world_node_15.size() + 1 + world_node_16.size() + 1..].as_ref(),
    )
    .unwrap();

    println!("should be effect: {:?}", world_node_17);

    let world_node_18 = WorldNode::from_tokens(
        tokens[base
            + 1
            + world_node_15.size()
            + 1
            + world_node_16.size()
            + world_node_17.size()
            + 2..]
            .as_ref(),
    )
    .unwrap();

    println!("should be effect: {:?}", world_node_18);

    let world_node_19 = WorldNode::from_tokens(
        tokens[base
            + 1
            + world_node_15.size()
            + 1
            + world_node_16.size()
            + world_node_17.size()
            + 2
            + world_node_18.size()
            + 1..]
            .as_ref(),
    )
    .unwrap();

    println!("should be effect: {:?}", world_node_19);

    let world_node_20 = WorldNode::from_tokens(
        tokens[base
            + 1
            + world_node_15.size()
            + 1
            + world_node_16.size()
            + world_node_17.size()
            + 2
            + world_node_18.size()
            + 1
            + world_node_19.size()
            + 1..]
            .as_ref(),
    )
    .unwrap();

    println!("should be effect: {:?}", world_node_20);

    base += 1
        + world_node_15.size()
        + 1
        + world_node_16.size()
        + world_node_17.size()
        + 2
        + world_node_18.size()
        + 1
        + world_node_19.size()
        + 1
        + world_node_20.size();

    println!("???: {:?}", tokens[base + 1..].as_ref(),);

    let world_node_21 = WorldNode::from_tokens(tokens[base + 1..].as_ref()).unwrap();

    println!("should be effect: {:?}", world_node_21);

    let world_node_22 =
        WorldNode::from_tokens(tokens[base + 1 + world_node_21.size() + 1..].as_ref()).unwrap();
    println!("should be effect: {:?}", world_node_22,);

    let world_node_23 = WorldNode::from_tokens(
        tokens[base + 1 + world_node_21.size() + 1 + world_node_22.size() + 1..].as_ref(),
    )
    .unwrap();
    println!("should be effect: {:?}", world_node_23,);

    let world_node_24 = WorldNode::from_tokens(
        tokens[base
            + 1
            + world_node_21.size()
            + 1
            + world_node_22.size()
            + 1
            + world_node_23.size()
            + 1..]
            .as_ref(),
    )
    .unwrap();

    println!("should be effect: {:?}", world_node_24);

    let world_node_25 = WorldNode::from_tokens(
        tokens[base
            + 1
            + world_node_21.size()
            + 1
            + world_node_22.size()
            + 1
            + world_node_23.size()
            + 1
            + world_node_24.size()
            + 1..]
            .as_ref(),
    )
    .unwrap();

    println!("should be decalsystem: {:?}", world_node_25,);

    let world_node_26 = WorldNode::from_tokens(
        tokens[base
            + 1
            + world_node_21.size()
            + 1
            + world_node_22.size()
            + 1
            + world_node_23.size()
            + 1
            + world_node_24.size()
            + 1
            + world_node_25.size()
            + 1..]
            .as_ref(),
    )
    .unwrap();

    println!("should be camera: {:?}", world_node_26,);

    let world_node_27 = WorldNode::from_tokens(
        tokens[base
            + 1
            + world_node_21.size()
            + 1
            + world_node_22.size()
            + 1
            + world_node_23.size()
            + 1
            + world_node_24.size()
            + 1
            + world_node_25.size()
            + 1
            + world_node_26.size()
            + 1..]
            .as_ref(),
    )
    .unwrap();

    println!("should be a controller: {world_node_27:?}");

    let world_node_28 = WorldNode::from_tokens(
        tokens[base
            + 1
            + world_node_21.size()
            + 1
            + world_node_22.size()
            + 1
            + world_node_23.size()
            + 1
            + world_node_24.size()
            + 1
            + world_node_25.size()
            + 1
            + world_node_26.size()
            + 1
            + world_node_27.size()
            + 1..]
            .as_ref(),
    )
    .unwrap();

    //Dummy size = 6
    println!("should be a dummy: {world_node_28:?}");

    let mut offset = base
        + 1
        + world_node_21.size()
        + 1
        + world_node_22.size()
        + 1
        + world_node_23.size()
        + 1
        + world_node_24.size()
        + 1
        + world_node_25.size()
        + 1
        + world_node_26.size()
        + 1
        + world_node_27.size()
        + 1
        + world_node_28.size()
        + 2;

    let world_node_29 = WorldNode::from_tokens(tokens[offset..].as_ref()).unwrap();

    println!("should be effect: {world_node_29:?}");

    offset += world_node_29.size();

    println!("???: {:?}", tokens[offset..].as_ref(),);

    println!("node count: {:?}", sector.node_sprite_batch_size);
    for _ in 0..sector.node_list_size {}
}
