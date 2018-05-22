use entities::*;
use std::collections::HashMap;
use std::cmp::Ordering;

pub fn find_path(map: &Map, start_id: i64, goal_id: i64) -> Option<Vec<i64>> {
    // Find start/goal nodes
    let start = map.find_node(start_id).expect("Invalid start node");
    let goal = map.find_node(goal_id).expect("Invalid goal node");

    // Check to see if start and goal node are the same
    if start.id == goal.id {
        return Some(vec![start.id]);
    }

    // The set of nodes already evaluated
    let mut closed_set: Vec<&Node> = Vec::new();

    // The set of currently discovered nodes that are not evaluated yet.
    // Initially, only the start node is known.
    let mut open_set: Vec<&Node> = vec![&start];

    // For each node, which node it can most efficiently be reached from. If a
    // node can be reached from many nodes, came_from will eventually contain
    // the most efficient previous step.
    let mut came_from: HashMap<i64, i64> = HashMap::new();

    // For each node, the cost of getting from the start node to that node.
    // Default value should be infinity.
    let mut g_score: HashMap<i64, f64> = HashMap::new();

    // The cost of going from start to start is zero.
    g_score.insert(start.id, 0.0);

    // For each node, the total cost of getting from the start node to the goal
    // by passing by that node. That value is partly known, partly heuristic.
    // Default value should be infinity.
    let mut f_score: HashMap<i64, f64> = HashMap::new();

    // For the first node, that value is completely heuristic.
    f_score.insert(start.id, start.haversine_distance(goal));

    while !open_set.is_empty() {
        // Find node in open_set with the lowest f_score value
        open_set.sort_by(|a, b| {
            let a_score = *f_score.get(&a.id).unwrap();
            let b_score = *f_score.get(&b.id).unwrap();
            if a_score < b_score {
                Ordering::Less
            } else if a_score > b_score {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        });
        let current = open_set[0];
        if current == goal {
            return Some(reconstruct_path(&came_from, current.id))
        }

        open_set.remove(0);
        closed_set.push(current);

        // Find ways that include current node
        let current_ways = map.ways_for_node(current);
        for way in &current_ways {
            let neighbor_ids = way.neighbors_of_node(current.id);
            for neighbor_id in &neighbor_ids {
                if closed_set.iter().find(|n| n.id == *neighbor_id).is_some() {
                    // Ignore the neighbor which is already evaluated.
                    continue;
                }
                let neighbor = map.find_node(*neighbor_id).expect("invalid node id");

                // Discover a new node
                if open_set.iter().find(|n| n.id == *neighbor_id).is_none() {
                    open_set.push(neighbor);
                }

                // The distance from start to a neighbor
                let tentative_g_score = g_score.get(&current.id).unwrap() +
                    current.haversine_distance(neighbor);
                if let Some(n) = g_score.get(&neighbor.id) {
                    if tentative_g_score >= *n {
                        // This is not a better path.
                        continue;
                    }
                }

                // This path is the best until now. Record it!
                came_from.insert(neighbor.id, current.id);
                g_score.insert(neighbor.id, tentative_g_score);
                f_score.insert(neighbor.id, tentative_g_score + neighbor.haversine_distance(goal));
            }
        }
    }

    return None;
}

fn reconstruct_path(came_from: &HashMap<i64, i64>, node_id: i64) -> Vec<i64> {
    let mut result = vec![node_id];
    let mut current_id = node_id;
    while came_from.contains_key(&current_id) {
        current_id = *came_from.get(&current_id).unwrap();
        result.push(current_id);
    }
    result.reverse();
    result
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

    #[test]
    fn find_path_connected_ways() {
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
            node_refs: vec![NodeRef { id: 1 }, NodeRef { id: 2 }],
            name: None, tags: HashMap::new()
        });
        map.ways.push(Way {
            id: 2, version: 1, timestamp: "foo".to_string(), changeset: 1,
            uid: Some(1), user: Some("viking".to_string()),
            node_refs: vec![NodeRef { id: 2 }, NodeRef { id: 3 }],
            name: None, tags: HashMap::new()
        });

        let expected = vec![1, 2, 3];
        let actual = find_path(&map, 1, 3).expect("couldn't find path");
        assert_eq!(expected, actual);
    }
}
