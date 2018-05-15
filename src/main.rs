#[macro_use] extern crate serde_derive;
extern crate serde_xml_rs;
extern crate bincode;
mod osm;

use std::env;
use std::fs::File;
use std::path::Path;
use osm::Map;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Syntax: {} <filename>", args[0]);
        return
    }

    let path = Path::new(&args[1]);
    let ext = path.extension().unwrap().to_str().unwrap();
    if ext == "xml" {
        let infile = File::open(path).unwrap();
        println!("reading data...");
        let map: Map = serde_xml_rs::from_reader(infile).unwrap();
        println!("number of nodes: {}", map.nodes.len());
        println!("number of ways: {}", map.ways.len());
        println!("number of relations: {}", map.relations.len());

        let stem = path.file_stem().unwrap().to_str().unwrap();
        let filename = format!("{}.bin", stem);
        let outfile = File::create(filename).unwrap();
        bincode::serialize_into(outfile, &map).unwrap();
    }
    else if ext == "bin" {
        let infile = File::open(path).unwrap();
        println!("reading data...");
        let map: Map = bincode::deserialize_from(infile).unwrap();
        println!("number of nodes: {}", map.nodes.len());
        println!("number of ways: {}", map.ways.len());
        println!("number of relations: {}", map.relations.len());
    }
}
