use serde_xml_rs::from_str;

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
    minlat: f64,
    minlon: f64,
    maxlat: f64,
    maxlon: f64
}

#[derive(Debug, Deserialize)]
#[serde(rename = "node")]
struct Node {
    id: u64,
    lat: f64,
    lon: f64,
    version: u16,
    timestamp: String,
    changeset: u64,
    uid: u64,
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
    let osm: OSM = from_str(s).unwrap();
    println!("{:#?}", osm);
}
