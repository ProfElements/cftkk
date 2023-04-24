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

    let data = fs::read(&args[1]).unwrap();
    let gcp = GcpReader::new(data).unwrap();

    for resource in gcp.resource_entries() {
        if resource.name.contains(".fetm") {
            let fetm = FetmReader::new(resource.data).unwrap();

            for token in fetm.tokens() {
                println!("{:?}", token);
            }
        }
        if resource.tag == Tag::Texture && !resource.name.contains(".sys") {
            let texr = TexrReader::new(resource.data).unwrap();

            println!(
                "Name: {}, Width: {}, Height: {}, Data length: {}. format: {:?}",
                resource.name,
                texr.header().width,
                texr.header().height,
                texr.image_data().len(),
                texr.header().texr_format
            );
        }
        if resource.tag != Tag::Texture && !resource.name.contains(".fetm") {
            println!("{}, {:?}", resource.name, resource.tag);
        }
    }
}
