use cftkk::{
    cmes::CMesReader,
    fetm::FetmReader,
    gcp::{GcpReader, Tag},
    texr::TexrReader,
};
use std::{
    env,
    fs::{self, write},
};

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
        if resource.tag == Tag::CollisionMesh {
            let cmes = CMesReader::new(resource.data).unwrap();

            println!(
                "Name: {}, Vertex Count: {}, Normal Count: {}, Triangle Count: {}",
                resource.name,
                cmes.header().vertices_count,
                cmes.header().normal_count,
                cmes.header().triangle_count
            );

            let mut string = String::new();

            string.push_str(format!("o {}\n", resource.name).as_str());

            for vertex in cmes.vertices().unwrap() {
                string.push_str(format!("v {} {} {}\n", vertex.x, vertex.y, vertex.z).as_str());
            }

            for normal in cmes.normals().unwrap() {
                string.push_str(format!("vn {} {} {}\n", normal.x, normal.y, normal.z).as_str());
            }

            string.push_str("s 0\n");

            for triangle in cmes.triangles().unwrap() {
                string.push_str(
                    format!(
                        "f {}//{} {}//{} {}//{}\n",
                        triangle.x_idx + 1,
                        triangle.normal_idx + 1,
                        triangle.y_idx + 1,
                        triangle.normal_idx + 1,
                        triangle.z_idx + 1,
                        triangle.normal_idx + 1
                    )
                    .as_str(),
                );
            }

            let _ = write(format!("{}.obj", resource.name), string);
        }
    }
}
