use std::collections::HashMap;
use std::convert::From;
use osm;

#[derive(Debug, Serialize, Deserialize)]
pub struct Map {
    pub version: String,
    pub generator: String,
    pub note: String,
    pub meta: Meta,
    pub bounds: Bounds,
    pub nodes: Vec<Node>,
    pub ways: Vec<Way>,
    pub relations: Vec<Relation>
}

impl From<osm::Map> for Map {
    fn from(map: osm::Map) -> Map {
        Map {
            version: map.version,
            generator: map.generator,
            note: map.note,
            meta: map.meta.into(),
            bounds: map.bounds.into(),
            nodes: map.nodes.into_iter().map(|x| x.into()).collect(),
            ways: map.ways.into_iter().map(|x| x.into()).collect(),
            relations: map.relations.into_iter().map(|x| x.into()).collect()
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Meta {
    pub osm_base: String
}

impl From<osm::Meta> for Meta {
    fn from(meta: osm::Meta) -> Meta {
        Meta { osm_base: meta.osm_base }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bounds {
    pub minlat: f64,
    pub minlon: f64,
    pub maxlat: f64,
    pub maxlon: f64
}

impl From<osm::Bounds> for Bounds {
    fn from(bounds: osm::Bounds) -> Bounds {
        Bounds {
            minlat: bounds.minlat,
            minlon: bounds.minlon,
            maxlat: bounds.maxlat,
            maxlon: bounds.maxlon
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Node {
    pub id: i64,
    pub lat: f64,
    pub lon: f64,
    pub version: u16,
    pub timestamp: String,
    pub changeset: u64,
    pub uid: Option<i64>,
    pub user: Option<String>,
    pub tags: HashMap<String, String>
}

impl From<osm::Node> for Node {
    fn from(node: osm::Node) -> Node {
        let mut tags = HashMap::new();
        for tag in node.tags {
            tags.insert(tag.k, tag.v);
        }
        Node {
            id: node.id,
            lat: node.lat,
            lon: node.lon,
            version: node.version,
            timestamp: node.timestamp,
            changeset: node.changeset,
            uid: node.uid,
            user: node.user,
            tags: tags
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Way {
    pub id: i64,
    pub version: u16,
    pub timestamp: String,
    pub changeset: u64,
    pub uid: Option<i64>,
    pub user: Option<String>,
    pub node_refs: Vec<NodeRef>,
    pub tags: HashMap<String, String>
}

impl From<osm::Way> for Way {
    fn from(way: osm::Way) -> Way {
        let mut tags = HashMap::new();
        for tag in way.tags {
            tags.insert(tag.k, tag.v);
        }
        let node_refs = way.node_refs.into_iter().map(|node_ref| {
            node_ref.into()
        }).collect();
        Way {
            id: way.id,
            version: way.version,
            timestamp: way.timestamp,
            changeset: way.changeset,
            uid: way.uid,
            user: way.user,
            node_refs: node_refs,
            tags: tags
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct NodeRef {
    pub id: i64
}

impl From<osm::NodeRef> for NodeRef {
    fn from(node_ref: osm::NodeRef) -> NodeRef {
        NodeRef { id: node_ref.id }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Relation {
    pub id: i64,
    pub version: u16,
    pub timestamp: String,
    pub changeset: u64,
    pub uid: Option<i64>,
    pub user: Option<String>,
    pub members: Vec<Member>,
    pub tags: HashMap<String, String>
}

impl From<osm::Relation> for Relation {
    fn from(relation: osm::Relation) -> Relation {
        let mut tags = HashMap::new();
        for tag in relation.tags {
            tags.insert(tag.k, tag.v);
        }
        let members = relation.members.into_iter().map(|member| {
            member.into()
        }).collect();
        Relation {
            id: relation.id,
            version: relation.version,
            timestamp: relation.timestamp,
            changeset: relation.changeset,
            uid: relation.uid,
            user: relation.user,
            members: members,
            tags: tags
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Member {
    pub kind: String,
    pub id: i64,
    pub role: String
}

impl From<osm::Member> for Member {
    fn from(member: osm::Member) -> Member {
        Member {
            kind: member.kind,
            id: member.id,
            role: member.role
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use osm;

    #[test]
    fn node_tags_conversion() {
        let node = osm::Node {
            id: 1,
            lat: 1.0,
            lon: 1.0,
            version: 123,
            timestamp: "foo".to_string(),
            changeset: 123,
            uid: Some(123),
            user: Some("dude".to_string()),
            tags: vec![
                osm::Tag { k: "foo".to_string(), v: "bar".to_string() },
                osm::Tag { k: "baz".to_string(), v: "qux".to_string() }
            ]
        };

        let mut tags = HashMap::new();
        tags.insert("foo".to_string(), "bar".to_string());
        tags.insert("baz".to_string(), "qux".to_string());
        let expected = Node {
            id: 1,
            lat: 1.0,
            lon: 1.0,
            version: 123,
            timestamp: "foo".to_string(),
            changeset: 123,
            uid: Some(123),
            user: Some("dude".to_string()),
            tags: tags
        };
        let actual: Node = node.into();
        assert_eq!(expected, actual);
    }

    #[test]
    fn way_tags_conversion() {
        let way = osm::Way {
            id: 1,
            version: 123,
            timestamp: "foo".to_string(),
            changeset: 123,
            uid: Some(123),
            user: Some("dude".to_string()),
            node_refs: vec![
                osm::NodeRef { id: 1 },
                osm::NodeRef { id: 2 },
                osm::NodeRef { id: 3 }
            ],
            tags: vec![
                osm::Tag { k: "foo".to_string(), v: "bar".to_string() },
                osm::Tag { k: "baz".to_string(), v: "qux".to_string() }
            ]
        };

        let mut tags = HashMap::new();
        tags.insert("foo".to_string(), "bar".to_string());
        tags.insert("baz".to_string(), "qux".to_string());
        let expected = Way {
            id: 1,
            version: 123,
            timestamp: "foo".to_string(),
            changeset: 123,
            uid: Some(123),
            user: Some("dude".to_string()),
            node_refs: vec![
                NodeRef { id: 1 },
                NodeRef { id: 2 },
                NodeRef { id: 3 }
            ],
            tags: tags
        };
        let actual: Way = way.into();
        assert_eq!(expected, actual);
    }

    #[test]
    fn relation_tags_conversion() {
        let relation = osm::Relation {
            id: 1,
            version: 123,
            timestamp: "foo".to_string(),
            changeset: 123,
            uid: Some(123),
            user: Some("dude".to_string()),
            members: vec![
                osm::Member { kind: "foo".to_string(), id: 1, role: "qux".to_string() },
                osm::Member { kind: "bar".to_string(), id: 2, role: "corge".to_string() },
                osm::Member { kind: "baz".to_string(), id: 3, role: "grault".to_string() }
            ],
            tags: vec![
                osm::Tag { k: "foo".to_string(), v: "bar".to_string() },
                osm::Tag { k: "baz".to_string(), v: "qux".to_string() }
            ]
        };

        let mut tags = HashMap::new();
        tags.insert("foo".to_string(), "bar".to_string());
        tags.insert("baz".to_string(), "qux".to_string());
        let expected = Relation {
            id: 1,
            version: 123,
            timestamp: "foo".to_string(),
            changeset: 123,
            uid: Some(123),
            user: Some("dude".to_string()),
            members: vec![
                Member { kind: "foo".to_string(), id: 1, role: "qux".to_string() },
                Member { kind: "bar".to_string(), id: 2, role: "corge".to_string() },
                Member { kind: "baz".to_string(), id: 3, role: "grault".to_string() }
            ],
            tags: tags
        };
        let actual: Relation = relation.into();
        assert_eq!(expected, actual);
    }
}
