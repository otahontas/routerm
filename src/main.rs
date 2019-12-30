extern crate osm_xml as osm;

use std::fs::File;

fn main() {
    let f = File::open("data/map.osm").unwrap();
    let doc = osm::OSM::parse(f).unwrap();

    /*
    for n in doc.nodes.iter() {
        let (id, node) = n;

        println!("id: {}", id);
        println!("\tlat: {}", node.lat);
        println!("\tlon: {}", node.lon);
        println!("\ttags:");

        for tag in &node.tags {
           println!("\t\t{}: {}", tag.key, tag.val);
        }
        println!("");
    }
    */

    for w in doc.ways.iter() {
        let (id, way) = w;

        println!("id: {}", id);
        println!("nodes:");

        for node in &way.nodes {
            match node {
                osm::UnresolvedReference::Node(id) => println!("\t{}", id),
                _ => println!(""),
            }
        }
        
        println!("tags:");
        for tag in &way.tags {
            println!("\t{}: {}", tag.key, tag.val);
        }
        println!("");
    }
}
