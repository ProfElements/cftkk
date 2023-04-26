use std::{
    env,
    fs::{self, write},
};

use cftkk::cmes::CMesReader;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() < 1 {
        eprintln!("usage: {} <collision_mesh>", args[0]);
    }

    let data = fs::read(&args[1]).unwrap();

    let cmes = CMesReader::new(data).unwrap();

    println!(
        "Name: {}, Vertex Count: {}, Normal Count: {}, Triangle Count: {}",
        &args[1],
        cmes.header().vertices_count,
        cmes.header().normal_count,
        cmes.header().triangle_count
    );
    let mut string = String::new();

    string.push_str(format!("o {}\n", &args[1]).as_str());

    for vertex in cmes.vertices().unwrap() {
        string.push_str(format!("v {} {} {}\n", -(vertex.x), vertex.y, vertex.z).as_str());
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
    let _ = write(format!("{}.obj", &args[1]), string);
}
