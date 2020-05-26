use reverseo::{Tree};
use std::time::SystemTime;
use kdtree::distance::squared_euclidean;

fn main() {
    let target: [f64; 2] = [39.4702, -0.3769];
    let map_file = "./valencia.osm.pbf";
    let now = SystemTime::now();
    println!("Loading map...");
    let (nodes, _edges) = osm4routing::reader::read(map_file).expect("Read OSM file");
    println!(" ✓ duration: {}s\n", now.elapsed().unwrap().as_secs());

    let now = SystemTime::now();
    println!("Building tree");

    let result = Tree::build(nodes.clone());

    println!(" ✓ duration: {}s\n", now.elapsed().unwrap().as_secs());
    println!("Constructed a tree with size {} nodes", result.tree.size());

    let now = SystemTime::now();
    println!("Finding nearest node");
    let result = result.tree.nearest(&target, 1, &squared_euclidean)
        .unwrap()
        .first()
        .expect("No node found")
        .clone();

    println!(" ✓ duration: {}s\n", now.elapsed().unwrap().as_secs());
    let node = result.1;
    let distance = result.0;
    println!("Nearest Node");
    println!("\tID: {}", node.id);
    println!("\tNode Lat: {}", node.coord.lat);
    println!("\tNode Lon: {}", node.coord.lon);
    println!("\tUses: {}", node.uses);
    println!("\tDistance from origin: {}", distance);

}
