#[macro_use] extern crate serde_derive;
extern crate serde_xml_rs;
extern crate bincode;
mod osm;
mod entities;
mod pathfinder;

use std::env;
use std::fs::File;
use std::path::Path;
use pathfinder::find_path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        println!("Syntax: {} <filename> <start-id> <end-id>", args[0]);
        return
    }
    let start_id = args[2].parse().unwrap();
    let end_id = args[3].parse().unwrap();

    let path = Path::new(&args[1]);
    let ext = path.extension().unwrap().to_str().unwrap();
    let map = match ext {
        "xml" => {
            let infile = File::open(path).unwrap();
            println!("Importing XML data...");
            let osm_map: osm::Map = serde_xml_rs::from_reader(infile).unwrap();

            println!("Exporting binary data for later use...");
            let stem = path.file_stem().unwrap().to_str().unwrap();
            let filename = format!("{}.bin", stem);
            let outfile = File::create(filename).unwrap();
            let result: entities::Map = osm_map.into();
            bincode::serialize_into(outfile, &result).unwrap();
            result
        },
        "bin" => {
            let infile = File::open(path).unwrap();
            println!("Importing binary data...");
            let result: entities::Map = bincode::deserialize_from(infile).unwrap();
            result
        }
        _ => {
            panic!("Invalid input file!");
        }
    };

    println!("Data summary:");
    println!("Number of nodes: {}", map.nodes.len());
    println!("Number of ways: {}", map.ways.len());
    println!("Number of relations: {}", map.relations.len());

    let path = find_path(&map, start_id, end_id);
    match path {
        Some(path) => {
            println!("=== Path between {} and {} ===", start_id, end_id);
            for node_id in &path {
                println!("{}", node_id);
            }
        },
        None => {
            println!("Couldn't find path between {} and {}", start_id, end_id);
        }
    }
}
