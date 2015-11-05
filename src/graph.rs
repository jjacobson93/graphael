extern crate rustc_serialize;
use std::collections::{HashMap, HashSet, BTreeMap};
use std::collections::hash_map::Entry;
use rustc_serialize::{Decoder as BaseDecoder, Decodable};
use rustc_serialize::json::{Json, ToJson, Decoder};
use std::fs::File;
use std::io::{Read, Write};
use std::io::Result as IOResult;
use std::string::ToString;

/**************************/
/*** Struct definitions ***/
/**************************/

#[derive(RustcDecodable, PartialEq, Debug)]
pub struct Node {
    pub id: NodeIndex,
    pub props:HashMap<String, String>,
}

#[derive(RustcDecodable, Debug, PartialEq)]
pub struct Edge {
	pub labels: Vec<String>
}

#[derive (Debug)]
pub struct Graph {
    nodes: HashMap<NodeIndex, Node>,
    edges: HashMap<NodeIndex, HashMap<NodeIndex, Edge>>,
    reverse_edges: HashMap<NodeIndex, HashSet<NodeIndex>>,
    max_node_id: NodeIndex
}

pub type NodeIndex = usize;

/***********************/
/*** Implementations ***/
/***********************/

impl PartialEq for Graph {
    fn eq(&self, other:&Self) -> bool {
        return self.nodes.eq(&other.nodes) && self.edges.eq(&other.edges)
    }
}

/// Used to convert a Node to JSON
impl ToJson for Node {
	fn to_json(&self) -> Json {
		let mut d = BTreeMap::new();
		let mut props = BTreeMap::new();
		d.insert("id".to_string(), self.id.to_json());
		for (k, v) in self.props.iter() {
			props.insert(k.clone(), Json::String(v.clone()));
		}
		d.insert("props".to_string(), Json::Object(props));
		Json::Object(d)
	}
}

impl ToJson for Edge {
	fn to_json(&self) -> Json {
		let mut d = BTreeMap::new();
		d.insert("labels".to_string(), Json::Array(self.labels.iter().map(|l| Json::String(l.to_string())).collect()));
		Json::Object(d)
	}
}

impl ToJson for Graph {
	fn to_json(&self) -> Json {
		let mut d = BTreeMap::new();
		let mut edge_json_map = BTreeMap::new();
		d.insert("nodes".to_string(), Json::Object(self.nodes.iter().map(|(i, n)| (i.to_string(), n.to_json())).collect()));
		for (index, edge) in self.edges.iter() {
			edge_json_map.insert(index.to_string(), Json::Object(edge.iter().map(|(i, n)| (i.to_string(), n.to_json())).collect()));
		}
		d.insert("edges".to_string(), Json::Object(edge_json_map));
		Json::Object(d)
    }
}
impl Decodable for Graph {

    fn decode<D:BaseDecoder>(decoder: &mut D) -> Result<Self, D::Error> {
        decoder.read_struct("root", 0, |decoder| {
            let mut max_node_id = 0;
            let nodes = try!(decoder.read_struct_field("nodes", 0, |decoder| {
                decoder.read_map(|decoder, len| {
                    let mut nodes = HashMap::new();
                    for idx in 0..len {
                        let node_index = try!(decoder.read_map_elt_key(idx, |decoder| decoder.read_usize()));
                        let node:Node = try!(decoder.read_map_elt_val(idx, |decoder| Decodable::decode(decoder)));
                        if node_index > max_node_id {
                            max_node_id = node_index + 1;
                        }
                        nodes.insert(node_index, node);
                    }
                    Ok(nodes)
                })

        }));
        let mut g = Graph{
                max_node_id: max_node_id,
                nodes: nodes,
                edges: HashMap::new(),
                reverse_edges: HashMap::new()
            };
            try!(decoder.read_struct_field("edges", 0, |decoder| {
                decoder.read_map(|decoder, len| {
                    for idx in 0..len {
                        let source_index = try!(decoder.read_map_elt_key(idx, |decoder| decoder.read_usize()));
                        let edge_map:HashMap<NodeIndex, Edge> = try!(decoder.read_map_elt_val(idx, |decoder| Decodable::decode(decoder)));
                        for (destination_index, edge) in edge_map.iter() {
							for label in edge.labels.iter() {
                            	g.connect_nodes(source_index, *destination_index, &*label)
							}
                        };
                    }
                    Ok(())
                })
            }));
            Ok(g)
        })

    }
}

/*** Graph ***/

impl Graph {

	/// Creates a new graph with no nodes or edges
    pub fn new() -> Graph {
        Graph {
            max_node_id: 0,
            nodes: HashMap::new(),
            edges: HashMap::new(),
            reverse_edges: HashMap::new()
        }
    }

	/// Encode and decode from file

	pub fn from_json(json: Json) -> Graph {
		let mut decoder = Decoder::new(json);
		match Decodable::decode(&mut decoder) {
			Ok(x) => x,
			Err(e) => panic!("Could not decode to graph: {}", e)
		}
	}

	pub fn read_from_file(name: String) -> Graph {
		let mut contents = String::new();
		let mut file:File = File::open(name).unwrap();

		file.read_to_string(&mut contents).unwrap();
		let jsonstring = match Json::from_str(&contents) {
			Ok(a) => a,
			Err(e) => panic!("Error reading JSON string: {}", e)
		};

		Graph::from_json(jsonstring)
	}

	pub fn write_to_file(&self, name: &'static str) -> IOResult<usize> {
		let mut file = try!(File::create(name));
		file.write(self.to_json().to_string().as_bytes())
	}

	/*******************/
	/****** Nodes ******/
	/*******************/

	/// Gets the next node id
    fn get_node_next_id(&mut self) -> NodeIndex {
        self.max_node_id = self.max_node_id + 1;
        self.max_node_id
    }

	/// Adds a Node to the Graph
    pub fn add_node(&mut self) -> NodeIndex {
        let idx = self.get_node_next_id();
        let node:Node = Node {
            id: idx,
            props: HashMap::new(),
        };
        self.nodes.insert(idx, node);
		idx
    }

	/// Adds a Node to the Graph with the specified properties
	pub fn add_node_with_props(&mut self, props: HashMap<String, String>) -> NodeIndex {
		let id = self.add_node();
		if let Some(node) = self.get_node_mut(id) {
			node.props = props;
		}
		id
	}

	// Removes a Node from the Graph given a NodeIndex
    pub fn remove_node(&mut self, node_id:NodeIndex) {
        if !self.nodes.contains_key(&node_id) {
            panic!("Tried to remove a node that didn't exist: {}", node_id);
        }
        if let Some(re) = self.reverse_edges.get(&node_id) {
            for n in re {
                self.edges.get_mut(n).unwrap().remove(&node_id);
            }
        }
        if let Some(e) = self.edges.get(&node_id) {
            for n in e.keys() {
                self.reverse_edges.get_mut(n).unwrap().remove(&node_id);
            }
        }
        self.edges.remove(&node_id);
        self.reverse_edges.remove(&node_id);
        self.nodes.remove(&node_id);
    }

    pub fn get_node(&self, node_id:NodeIndex) -> Option<&Node> {
        self.nodes.get(&node_id)
    }

    pub fn get_node_mut(&mut self, node_id:NodeIndex) -> Option<&mut Node> {
        self.nodes.get_mut(&node_id)
    }

	/// Get Nodes with a given attribute
    pub fn nodes_with_attr(&self, attr: &String) -> Vec<&Node> {
		self.nodes.values().filter(|node|
			if let Some(e) = self.edges.get(&node.id) {
				if let Some(edge) = e.get(&node.id) {
					edge.labels.contains(attr)
				} else {
					false
				}
			}
			else {
				false
			}).collect()
    }

    /// Get Nodes with a key-value pair
    pub fn nodes_with_prop(&self, key: &String, value: &String) -> Vec<NodeIndex> {
        self.nodes.values().filter(|node|
            node.props.iter().find(|&(k, v)| **k == *key && **v == *value ).is_some())
			.map(|node|
				node.id
			).collect()
    }

	/*******************/
	/****** Edges ******/
	/*******************/

	/// Add an Edge to the graph from the source NodeIndex to the target NodeIndex with a label
    fn connect_nodes(&mut self, origin:NodeIndex, destination:NodeIndex, label:&str) {
        if !self.nodes.contains_key(&origin) {
            panic!("Tried to connect node id that wasn't in the database: {}", origin)
        }

        if !self.nodes.contains_key(&destination) {
            panic!("Tried to connect node id that wasn't in the database: {}", destination)
        }
		let lbl = label.to_string();
		let src_map = self.edges.entry(origin).or_insert(HashMap::new());
		match src_map.entry(destination) {
			Entry::Vacant(e) => {e.insert(Edge { labels: vec![lbl] });},
			Entry::Occupied(mut e) => if !e.get().labels.contains(&lbl) { e.get_mut().labels.push(lbl); } else {()}
		};

        if match self.reverse_edges.get_mut(&origin){
            Some(m) => {
                if m.contains(&destination) {
                    m.remove(&destination);
                    false
                }
                else {
                    true
                }
            },
            None => true
        } {
            if !self.reverse_edges.contains_key(&destination) {
                self.reverse_edges.insert(destination, HashSet::new());
            }
            self.reverse_edges.get_mut(&destination).unwrap().insert(origin);
        }
    }

	/// Check if two nodes are connected on the graph
    pub fn are_connected(&mut self, origin:NodeIndex, destination:NodeIndex) -> bool {
        if !self.nodes.contains_key(&origin) {
            panic!("Tried to check node id that wasn't in the database: {}", origin)
        }

        if !self.nodes.contains_key(&destination) {
            panic!("Tried to check node id that wasn't in the database: {}", destination)
        }
        match self.edges.get(&origin) {
            Some(m) => m.contains_key(&destination),
            None    => false
        }
	}

    /// Get Edges with a given label
	pub fn edges_with_label(&self, label: &String) -> HashMap<&NodeIndex, HashMap<&NodeIndex, &Edge>> {
		let mut edges = HashMap::<&NodeIndex, HashMap<&NodeIndex, &Edge>>::new();
		for (src, map) in self.edges.iter() {
			let filtered: HashMap<&NodeIndex, &Edge> = map.iter().filter(|&(_idx, edge)| edge.labels.contains(label)).collect();
			if filtered.len() > 0 {
				edges.insert(src, filtered);
			}
		}
		edges
	}

	/// Get Nodes with a given label from a source NodeIndex
	pub fn edges_with_label_from(&self, source: NodeIndex, label: &str) -> Vec<NodeIndex> {
		let lbl = label.to_string();
		if let Some(edge) = self.edges.get(&source) {
		edge.iter().filter_map(
			|(&idx, edge)|{
				if edge.labels.contains(&lbl) {
					Some(idx)
				} else {
					None
				}}).collect()
			} else {
				vec![]
			}
	}

}

#[cfg(test)]
mod tests {
    use super::*;
    use rustc_serialize::json::{Json, ToJson};
	use std::collections::HashMap;
    #[test]
    fn adding_nodes() {
        let mut g = Graph::new();
        let id1 = g.add_node();
        let id2 = g.add_node();
        {
            let n1 = g.get_node(id1).unwrap();
            assert!(n1.id == id1);
        }
        {
            let n2 = g.get_node_mut(id2).unwrap();
            assert!(n2.id == id2);
            n2.props.insert("hey".to_string(), "you".to_string());
        }

    }

    #[test]
    fn connecting_nodes() {
        let mut g = Graph::new();
        let id1 = g.add_node();
        let id2 = g.add_node();
        let id3 = g.add_node();
        g.connect_nodes(id1, id2, "hello");
        assert!(g.are_connected(id1, id2));
        assert!(!g.are_connected(id2, id1));
        g.connect_nodes(id2, id1, "hi");
        g.connect_nodes(id2, id3, "hello");

        assert!(g.are_connected(id1, id2));
        assert!(g.are_connected(id2, id1));
        assert!(g.are_connected(id2, id3));
        assert!(!g.are_connected(id3, id2));

		g.connect_nodes(id1, id2, "second_label");
		println!("{:?}", g.edges_with_label_from(id1, "second_label"));
		assert!(g.edges_with_label_from(id1, "second_label") == vec![id2]);
		assert!(g.edges_with_label_from(id1, "hello") == vec![id2]);

    }

    #[test]
    fn removing_nodes() {
        let mut g = Graph::new();
        let id1 = g.add_node();
        let id2 = g.add_node();
        let id3 = g.add_node();
        g.connect_nodes(id1, id2, "hello");
        assert!(g.are_connected(id1, id2));
        assert!(!g.are_connected(id2, id1));
        g.connect_nodes(id2, id1, "hi");
        g.connect_nodes(id2, id3, "what's new");

        assert!(g.are_connected(id1, id2));
        assert!(g.are_connected(id2, id1));
        assert!(g.are_connected(id2, id3));
        assert!(!g.are_connected(id3, id2));
        assert!(!g.are_connected(id3, id1));
        assert!(!g.are_connected(id1, id3));

        g.remove_node(id3);
        assert!(g.are_connected(id1, id2));
        assert!(g.are_connected(id2, id1));
        assert!(g.get_node(id3).is_none());
    }
	#[test]
	fn node_properties() {
		let mut g = Graph::new();
		let mut props_hash_map1 = HashMap::new();
		props_hash_map1.insert("prop1".to_string(), "val1".to_string());
		props_hash_map1.insert("prop2".to_string(), "val2".to_string());
		props_hash_map1.insert("prop3".to_string(), "val3".to_string());

		let mut props_hash_map2 = HashMap::new();
		props_hash_map2.insert("prop2".to_string(), "val2".to_string());
		props_hash_map2.insert("prop3".to_string(), "val3".to_string());
		let id1 = g.add_node_with_props(props_hash_map1);
		let id2 = g.add_node_with_props(props_hash_map2);

		let props_list = g.nodes_with_prop(&"prop2".to_string(), &"val2".to_string());
		assert!(props_list.len() == 2);
		assert!(props_list.contains(&id1));
		assert!(props_list.contains(&id2));

		let props_list2 = g.nodes_with_prop(&"prop1".to_string(), &"val1".to_string());
		assert!(props_list2.len() == 1);
		assert!(props_list2.contains(&id1));
	}

    #[test]
    fn json_io() {
        let mut g = Graph::new();
        let id1 = g.add_node();
        let id2 = g.add_node();
        let id3 = g.add_node();
        g.connect_nodes(id1, id2, "hello");
        g.connect_nodes(id2, id1, "hi");
        g.connect_nodes(id2, id3, "what's new");

        let json_string = g.to_json().to_string();
        let expected_string = r#"{"edges":{"1":{"2":{"labels":["hello"]}},"2":{"1":{"labels":["hi"]},"3":{"labels":["what's new"]}}},"nodes":{"1":{"id":1,"props":{}},"2":{"id":2,"props":{}},"3":{"id":3,"props":{}}}}"#;
        assert!(json_string == expected_string);
        let new_json = Json::from_str(expected_string).unwrap();
        let g2 = Graph::from_json(new_json);
        assert!(g.eq(&g2));
    }
}
