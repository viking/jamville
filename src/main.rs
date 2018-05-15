#[macro_use] extern crate serde_derive;
extern crate serde_xml_rs;
extern crate bincode;
mod osm;

use std::env;
use std::fs::File;
use std::path::Path;
use osm::OSM;

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
        let data: OSM = serde_xml_rs::from_reader(infile).unwrap();
        println!("number of nodes: {}", data.nodes.len());
        println!("number of ways: {}", data.ways.len());
        println!("number of relations: {}", data.relations.len());

        let stem = path.file_stem().unwrap().to_str().unwrap();
        let filename = format!("{}.bin", stem);
        let outfile = File::create(filename).unwrap();
        bincode::serialize_into(outfile, &data).unwrap();
    }
    else if ext == "bin" {
        let infile = File::open(path).unwrap();
        println!("reading data...");
        let data: OSM = bincode::deserialize_from(infile).unwrap();
        println!("number of nodes: {}", data.nodes.len());
        println!("number of ways: {}", data.ways.len());
        println!("number of relations: {}", data.relations.len());
    }
}
