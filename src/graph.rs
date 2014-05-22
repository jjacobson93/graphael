#![allow(unused_imports)]
#![allow(dead_code)]

use collections::{HashMap, TreeMap};
use std::{fmt, cmp, uint};
use std::io::{File, IoResult};
use serialize::{json, Decodable, Encodable};

/**************************/
/*** Struct definitions ***/
/**************************/

#[deriving(Decodable, Encodable)]
pub struct Node {
	id: NodeIndex,
    pub props: HashMap<~str, ~str>
}

#[deriving(Decodable, Encodable)]
pub struct Edge {
	pub labels: ~[~str]
}

#[deriving(Decodable, Encodable)]
pub struct Graph {
	pub nodes: HashMap<NodeIndex, Node>,
	pub edges: HashMap<NodeIndex, HashMap<NodeIndex, Edge>>
}

#[deriving(Eq, Hash, TotalEq, Decodable, Encodable)]
type NodeIndex = uint;

/***********************/
/*** Implementations ***/
/***********************/

/*** Node ***/

/// Used to give a string representation for Node
impl fmt::Show for Node {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f.buf, "Node<id={}, props={}>", self.id, self.props)
	}
}

/// Used to convert a Node to JSON
impl json::ToJson for Node {
	fn to_json(&self) -> json::Json {
		let mut d = box TreeMap::new();
		d.insert("id".to_owned(), self.id.to_json());
		d.insert("props".to_owned(), self.props.to_json());
		json::Object(d)
	}
}

/*** Edge ***/

/// Used to give a string representation for Edge
impl fmt::Show for Edge {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f.buf, "Edge<labels={}>", self.labels)
	}
}

impl cmp::Eq for Edge {
	fn eq(&self, other: &Edge) -> bool {
		self.labels == other.labels
	}
}

/*** Graph ***/

/// Used to give a string representation for Graph
impl fmt::Show for Graph {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f.buf, "Graph (\n\tnodes = {}\n\tedges = {}\n)", self.nodes, self.edges)
	}
}

impl Graph {

	/// Creates a new graph with empty HashMaps
	pub fn new() -> Graph {
		Graph { nodes: HashMap::new(), edges: HashMap::new() }
	}

	/// Encode and decode from file

	pub fn read_from_file(name: ~str) -> Graph {
		let contents = File::open(&Path::new(name)).read_to_str();
		let string = match contents {
			Ok(v) => v,
			Err(v) => fail!("Failed to load the file: {}", v)
		};
		let jsonstring = match json::from_str(string) {
			Ok(a) => a,
			Err(e) => fail!("Error reading JSON string: {}", e)
		};

		let mut decoder = json::Decoder::new(jsonstring);
		match Decodable::decode(&mut decoder) {
			Ok(x) => x,
			Err(e) => fail!("Could not decode to graph: {}", e)
		}
	}

	pub fn write_to_file(&self, name: &'static str) -> IoResult<()> {
		let mut file = File::create(&Path::new(name));
		file.write(json::Encoder::buffer_encode(&self).as_slice())
	}

	/*******************/
	/****** Nodes ******/
	/*******************/

	/// Gets the next node index
	pub fn next_node_index(&self) -> NodeIndex {
		self.nodes.len() + 1
	}

	/// Adds a Node to the Graph with a vector of attributes and a HashMap of properties
	pub fn add_node(&mut self, attr: ~[~str], props: HashMap<~str, ~str>) -> NodeIndex {
		let idx = self.next_node_index();
		self.nodes.insert(idx, Node { id: idx, props: props });
		for a in attr.iter() {
			self.add_edge(idx, idx, a.to_owned());
		}
		idx
	}

	/// Removes a Node from the Graph given a NodeIndex
	pub fn remove_node(&mut self, idx: NodeIndex) -> bool {
		self.nodes.remove(&idx)
	}


	/// Get Nodes with a given attribute
	pub fn nodes_with_attr<'a>(&'a self, attr: ~str) -> ~[&'a Node] {
		self.nodes.values().filter(|node| 
			match self.edges.find(&node.id) {
				Some(map) => match map.find(&node.id) {
					Some(edge) => edge.labels.contains(&attr),
					None => false
				},
				None => false
			}).collect()
	}

	/// Get Nodes with a key-value pair
	pub fn nodes_with_prop<'a>(&'a self, key: ~str, value: ~str) -> ~[&Node] {
		self.nodes.values().filter(|node|
			match node.props.find(&key) {
				Some(v) => v == &value,
				None => false,
			}).collect()
	}

	/*******************/
	/****** Edges ******/
	/*******************/

	/// Add an Edge to the graph from the source NodeIndex to the target NodeIndex with a label
	pub fn add_edge<'a>(&'a mut self, source: NodeIndex, target: NodeIndex, label: ~str) -> &'a mut Edge {
		let src_map = self.edges.find_or_insert(source, HashMap::new());
		src_map.insert_or_update_with(target, 
			Edge { labels: ~[label.clone()] },
			|_k, v| if !v.labels.contains(&label) { v.labels.push(label.clone()) })
	}

	/// Get Edges with a given label
	pub fn edges_with_label<'a>(&'a mut self, label: ~str) -> HashMap<&'a NodeIndex, HashMap<&'a NodeIndex, &'a Edge>> {
		let mut edges = HashMap::<&NodeIndex, HashMap<&NodeIndex, &Edge>>::new();
		for (src, map) in self.edges.iter() {
			let filtered: HashMap<&NodeIndex, &Edge> = map.iter().filter(|&(_idx, edge)| edge.labels.contains(&label)).collect();
			if filtered.len() > 0 {
				edges.insert(src, filtered);
			}
		}
		edges
	}

	/// Get Nodes with a given label from a source NodeIndex
	pub fn edges_with_label_from<'a>(&'a mut self, source: NodeIndex, label: ~str) -> ~[&'a Node] {
		self.edges.get(&source).iter().filter_map(
			|(&idx, edge)| 
				if edge.labels.contains(&label) {
					self.nodes.find(&idx)
				} else {
					None
				}).collect()
	} 

}

