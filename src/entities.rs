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

impl Map {
    pub fn find_node(&self, node_id: i64) -> Option<&Node> {
        self.nodes.iter().find(|node| node.id == node_id)
    }

    pub fn ways_for_node(&self, node: &Node) -> Vec<&Way> {
        self.ways.iter().
            filter(|way| way.contains_node_id(node.id)).
            collect()
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

    // tags
    pub name: Option<String>,
    pub tags: HashMap<String, String>
}

impl From<osm::Node> for Node {
    fn from(node: osm::Node) -> Node {
        let mut name = None;
        let mut tags = HashMap::new();
        for tag in node.tags {
            if tag.k == "name" {
                name = Some(tag.v);
            }
            else {
                tags.insert(tag.k, tag.v);
            }
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
            name: name,
            tags: tags
        }
    }
}

impl Node {
    pub fn haversine_distance(&self, other: &Node) -> f64 {
        let r = 6371e3; // meters
        let phi_1 = self.lat.to_radians();
        let phi_2 = other.lat.to_radians();
        let delta_phi = (other.lat - self.lat).to_radians();
        let delta_lambda = (other.lon - self.lon).to_radians();

        let a = (delta_phi / 2.0).sin().powi(2) * phi_1.cos() * phi_2.cos() *
            (delta_lambda / 2.0).sin().powi(2);
        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
        return r * c;
    }
}

enum WayDirection {
    None,
    Forward,
    Reverse
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

    // tags
    pub name: Option<String>,
    pub tags: HashMap<String, String>
}

impl From<osm::Way> for Way {
    fn from(way: osm::Way) -> Way {
        let mut name = None;
        let mut tags = HashMap::new();
        for tag in way.tags {
            if tag.k == "name" {
                name = Some(tag.v);
            }
            else {
                tags.insert(tag.k, tag.v);
            }
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
            name: name,
            tags: tags
        }
    }
}

impl Way {
    pub fn contains_node_id(&self, node_id: i64) -> bool {
        self.node_refs.iter().find(|nr| nr.id == node_id).is_some()
    }

    pub fn neighbors_of_node(&self, node_id: i64) -> Vec<i64> {
        let mut result = Vec::new();
        let index = self.node_refs.iter().position(|nr| nr.id == node_id);
        match index {
            Some(index) => {
                if index > 0 {
                    result.push(self.node_refs[index-1].id);
                }
                if index < (self.node_refs.len() - 1) {
                    result.push(self.node_refs[index+1].id);
                }
            },
            None => {}
        }
        return result;
    }

    pub fn find_path(&self, start_id: i64, end_id: i64) -> Option<Vec<i64>> {
        if start_id == end_id {
            return Some(vec![start_id]);
        }

        if !self.contains_node_id(start_id) || !self.contains_node_id(end_id) {
            return None;
        }

        let mut path = Vec::new();
        let mut direction = WayDirection::None;
        for node_ref in &self.node_refs {
            if node_ref.id == start_id {
                match direction {
                    WayDirection::None => {
                        // found start_id first, begin path
                        direction = WayDirection::Forward;
                        path.push(node_ref.id);
                    },
                    WayDirection::Reverse => {
                        // found start_id last, return path in reverse
                        path.push(node_ref.id);
                        path.reverse();
                        return Some(path);
                    },
                    _ => panic!("invalid direction")
                }
            }
            else if node_ref.id == end_id {
                match direction {
                    WayDirection::None => {
                        // found end_id first, begin path
                        direction = WayDirection::Reverse;
                        path.push(node_ref.id);
                    },
                    WayDirection::Forward => {
                        // found end_id last, return path
                        path.push(node_ref.id);
                        return Some(path);
                    },
                    _ => panic!("invalid direction")
                }
            }
            else {
                match direction {
                    WayDirection::None => {
                        // found unimportant node, do nothing
                    },
                    _ => {
                        // found node en-route, add to path
                        path.push(node_ref.id);
                    }
                }
            }
        }

        None
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

    // tags
    pub name: Option<String>,
    pub tags: HashMap<String, String>
}

impl From<osm::Relation> for Relation {
    fn from(relation: osm::Relation) -> Relation {
        let mut name = None;
        let mut tags = HashMap::new();
        for tag in relation.tags {
            if tag.k == "name" {
                name = Some(tag.v);
            }
            else {
                tags.insert(tag.k, tag.v);
            }
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
            name: name,
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
                osm::Tag { k: "name".to_string(), v: "foo".to_string() },
                osm::Tag { k: "baz".to_string(), v: "qux".to_string() }
            ]
        };

        let mut tags = HashMap::new();
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
            name: Some("foo".to_string()),
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
                osm::Tag { k: "name".to_string(), v: "foo".to_string() },
                osm::Tag { k: "baz".to_string(), v: "qux".to_string() }
            ]
        };

        let mut tags = HashMap::new();
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
            name: Some("foo".to_string()),
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
                osm::Tag { k: "name".to_string(), v: "foo".to_string() },
                osm::Tag { k: "baz".to_string(), v: "qux".to_string() }
            ]
        };

        let mut tags = HashMap::new();
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
            name: Some("foo".to_string()),
            tags: tags
        };
        let actual: Relation = relation.into();
        assert_eq!(expected, actual);
    }

    #[test]
    fn way_contains_node_id() {
        let way = Way {
            id: 1, version: 123, timestamp: "foo".to_string(), changeset: 123,
            uid: Some(123), user: Some("dude".to_string()),
            node_refs: vec![
                NodeRef { id: 1 },
                NodeRef { id: 2 },
                NodeRef { id: 3 }
            ],
            name: Some("foo".to_string()), tags: HashMap::new()
        };
        assert!(way.contains_node_id(1));
        assert!(!way.contains_node_id(4));
    }

    #[test]
    fn way_find_path_non_existent_ids() {
        let way = Way {
            id: 1, version: 123, timestamp: "foo".to_string(), changeset: 123,
            uid: Some(123), user: Some("dude".to_string()),
            node_refs: vec![
                NodeRef { id: 1 },
                NodeRef { id: 2 },
                NodeRef { id: 3 }
            ],
            name: Some("foo".to_string()), tags: HashMap::new()
        };
        assert!(way.find_path(1, 4).is_none());
        assert!(way.find_path(0, 3).is_none());
        assert!(way.find_path(0, 4).is_none());
    }

    #[test]
    fn way_find_path_same_node() {
        let way = Way {
            id: 1, version: 123, timestamp: "foo".to_string(), changeset: 123,
            uid: Some(123), user: Some("dude".to_string()),
            node_refs: vec![
                NodeRef { id: 1 },
                NodeRef { id: 2 },
                NodeRef { id: 3 }
            ],
            name: Some("foo".to_string()), tags: HashMap::new()
        };

        let expected = vec![1];
        let actual = way.find_path(1, 1).expect("couldn't find path");
        assert_eq!(expected, actual);
    }

    #[test]
    fn way_find_path_forward() {
        let way = Way {
            id: 1, version: 123, timestamp: "foo".to_string(), changeset: 123,
            uid: Some(123), user: Some("dude".to_string()),
            node_refs: vec![
                NodeRef { id: 1 },
                NodeRef { id: 2 },
                NodeRef { id: 3 }
            ],
            name: Some("foo".to_string()), tags: HashMap::new()
        };

        let expected = vec![1, 2, 3];
        let actual = way.find_path(1, 3).expect("couldn't find path");
        assert_eq!(expected, actual);
    }

    #[test]
    fn way_find_path_reverse() {
        let way = Way {
            id: 1, version: 123, timestamp: "foo".to_string(), changeset: 123,
            uid: Some(123), user: Some("dude".to_string()),
            node_refs: vec![
                NodeRef { id: 1 },
                NodeRef { id: 2 },
                NodeRef { id: 3 }
            ],
            name: Some("foo".to_string()), tags: HashMap::new()
        };

        let expected = vec![3, 2, 1];
        let actual = way.find_path(3, 1).expect("couldn't find path");
        assert_eq!(expected, actual);
    }
}
