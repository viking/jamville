use entities::*;

pub fn find_path(map: &Map, start_id: i64, end_id: i64) -> Option<Vec<i64>> {
    // find start/end nodes
    let start = map.nodes.iter().find(|node| node.id == start_id).expect("Invalid start node");
    let end = map.nodes.iter().find(|node| node.id == end_id).expect("Invalid end node");

    // check to see if start and end node are the same
    if start.id == end.id {
        return Some(vec![start.id]);
    }

    // find ways that contain start/end nodes
    let start_ways: Vec<&Way> = map.ways.iter().
        filter(|way| way.contains_node_id(start_id)).
        collect();
    let end_ways: Vec<&Way> = map.ways.iter().
        filter(|way| way.contains_node_id(end_id)).
        collect();

    // check to see if nodes are contained by same way
    for way in &start_ways {
        if end_ways.contains(way) {
            return way.find_path(start.id, end.id);
        }
    }

    return None;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn create_map() -> Map {
        Map {
            version: "0.1".to_string(), generator: "test".to_string(),
            note: "foo".to_string(), meta: Meta { osm_base: "bar".to_string() },
            bounds: Bounds { minlat: 0.0, minlon: 0.0, maxlat: 10.0, maxlon: 10.0 },
            nodes: Vec::new(), ways: Vec::new(), relations: Vec::new()
        }
    }

    #[test]
    fn find_path_same_start_end() {
        let mut map = create_map();
        map.nodes.push(Node {
            id: 1, lat: 5.0, lon: 5.0, version: 1, timestamp: "foo".to_string(),
            changeset: 1, uid: Some(1), user: Some("viking".to_string()),
            name: None, tags: HashMap::new()
        });

        let expected = vec![1];
        let actual = find_path(&map, 1, 1).expect("couldn't find path");
        assert_eq!(expected, actual);
    }

    #[test]
    fn find_path_same_way_minimal_distance() {
        let mut map = create_map();
        map.nodes.push(Node {
            id: 1, lat: 5.0, lon: 5.0, version: 1, timestamp: "foo".to_string(),
            changeset: 1, uid: Some(1), user: Some("viking".to_string()),
            name: None, tags: HashMap::new()
        });
        map.nodes.push(Node {
            id: 2, lat: 5.1, lon: 5.1, version: 1, timestamp: "foo".to_string(),
            changeset: 1, uid: Some(1), user: Some("viking".to_string()),
            name: None, tags: HashMap::new()
        });
        map.ways.push(Way {
            id: 1, version: 1, timestamp: "foo".to_string(), changeset: 1,
            uid: Some(1), user: Some("viking".to_string()),
            node_refs: vec![NodeRef { id: 1 }, NodeRef { id: 2 }],
            name: None, tags: HashMap::new()
        });

        let expected = vec![1, 2];
        let actual = find_path(&map, 1, 2).expect("couldn't find path");
        assert_eq!(expected, actual);
    }

    #[test]
    fn find_path_same_way_multiple_steps() {
        let mut map = create_map();
        map.nodes.push(Node {
            id: 1, lat: 5.0, lon: 5.0, version: 1, timestamp: "foo".to_string(),
            changeset: 1, uid: Some(1), user: Some("viking".to_string()),
            name: None, tags: HashMap::new()
        });
        map.nodes.push(Node {
            id: 2, lat: 5.1, lon: 5.1, version: 1, timestamp: "foo".to_string(),
            changeset: 1, uid: Some(1), user: Some("viking".to_string()),
            name: None, tags: HashMap::new()
        });
        map.nodes.push(Node {
            id: 3, lat: 5.2, lon: 5.2, version: 1, timestamp: "foo".to_string(),
            changeset: 1, uid: Some(1), user: Some("viking".to_string()),
            name: None, tags: HashMap::new()
        });
        map.ways.push(Way {
            id: 1, version: 1, timestamp: "foo".to_string(), changeset: 1,
            uid: Some(1), user: Some("viking".to_string()),
            node_refs: vec![NodeRef { id: 1 }, NodeRef { id: 2 }, NodeRef { id: 3 }],
            name: None, tags: HashMap::new()
        });

        let expected = vec![1, 2, 3];
        let actual = find_path(&map, 1, 3).expect("couldn't find path");
        assert_eq!(expected, actual);
    }

    #[test]
    fn find_path_same_way_multiple_steps_reverse() {
        let mut map = create_map();
        map.nodes.push(Node {
            id: 1, lat: 5.0, lon: 5.0, version: 1, timestamp: "foo".to_string(),
            changeset: 1, uid: Some(1), user: Some("viking".to_string()),
            name: None, tags: HashMap::new()
        });
        map.nodes.push(Node {
            id: 2, lat: 5.1, lon: 5.1, version: 1, timestamp: "foo".to_string(),
            changeset: 1, uid: Some(1), user: Some("viking".to_string()),
            name: None, tags: HashMap::new()
        });
        map.nodes.push(Node {
            id: 3, lat: 5.2, lon: 5.2, version: 1, timestamp: "foo".to_string(),
            changeset: 1, uid: Some(1), user: Some("viking".to_string()),
            name: None, tags: HashMap::new()
        });
        map.ways.push(Way {
            id: 1, version: 1, timestamp: "foo".to_string(), changeset: 1,
            uid: Some(1), user: Some("viking".to_string()),
            node_refs: vec![NodeRef { id: 1 }, NodeRef { id: 2 }, NodeRef { id: 3 }],
            name: None, tags: HashMap::new()
        });

        let expected = vec![3, 2, 1];
        let actual = find_path(&map, 3, 1).expect("couldn't find path");
        assert_eq!(expected, actual);
    }
}
