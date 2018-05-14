use serde_xml_rs::deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename = "osm")]
struct OSM {
    version: String,
    generator: String,
    note: String,
    meta: Meta,
    bounds: Bounds,
    #[serde(rename = "node", default)]
    nodes: Vec<Node>
}

#[derive(Debug, Deserialize)]
#[serde(rename = "meta")]
struct Meta {
    osm_base: String
}

#[derive(Debug, Deserialize)]
#[serde(rename = "bounds")]
struct Bounds {
    minlat: String, // f64 is broken for attributes
    minlon: String,
    maxlat: String,
    maxlon: String
}

#[derive(Debug, Deserialize)]
#[serde(rename = "node")]
struct Node {
    id: String,
    lat: String,
    lon: String,
    version: String,
    timestamp: String,
    changeset: String,
    uid: String,
    user: String,
    #[serde(rename = "tag", default)]
    tags: Vec<Tag>
}

#[derive(Debug, Deserialize)]
#[serde(rename = "tag")]
struct Tag {
    k: String,
    v: String
}

#[test]
fn it_works() {
    let s = r##"
        <?xml version="1.0" encoding="UTF-8"?>
        <osm version="0.6" generator="Overpass API 0.7.55 579b1eec">
            <note>The data included in this document is from www.openstreetmap.org. The data is made available under ODbL.</note>
            <meta osm_base="2018-05-14T21:45:02Z"/>
            <bounds minlat="35.9717000" minlon="-87.0639000" maxlat="36.3594000" maxlon="-86.4627000"/>
            <node id="37060116" lat="36.0872268" lon="-87.0759046" version="30" timestamp="2012-07-18T16:22:48Z" changeset="12288724" uid="722137" user="OSMF Redaction Account"/>
            <node id="37060125" lat="36.0873400" lon="-87.0759682" version="1" timestamp="2007-08-30T23:16:11Z" changeset="277489" uid="10759" user="WJHildreth">
              <tag k="created_by" v="JOSM"/>
            </node>
        </osm>
    "##;
    let osm: OSM = deserialize(s.as_bytes()).unwrap();
    println!("{:#?}", osm);
}
