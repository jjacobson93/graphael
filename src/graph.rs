#![allow(dead_code)]

use std::collections::{BTreeMap};
use std::collections::btree_map::{Entry};
use std::{fmt, cmp};
use std::fs::{File};
use std::io::{Result, Read, Write};
use std::string::ToString;
use rustc_serialize::{Decodable};
use rustc_serialize::json::{Json, ToJson, Decoder};

/**************************/
/*** Struct definitions ***/
/**************************/

#[derive(RustcDecodable, PartialEq, Debug)]
pub struct Node {
	id: NodeIndex,
    pub props: BTreeMap<Box<String>, Box<String>>
}
#[derive(RustcDecodable, Debug)]
pub struct Edge {
	pub labels: Vec<String>
}

#[derive(RustcDecodable, Debug)]
pub struct Graph {
	pub nodes: BTreeMap<NodeIndex, Node>,
	pub edges: BTreeMap<NodeIndex, BTreeMap<NodeIndex, Edge>>
}

//#[derive(Eq, Hash, TotalEq, RustcDecodable, RustcEncodable)]
pub type NodeIndex = usize;

/***********************/
/*** Implementations ***/
/***********************/

/*** Node ***/

/// Used to give a string representation for Node
impl fmt::Display for Node {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Node<id={}, props=", self.id);
		for (k, v) in self.props.iter() {
			write!(f, "{}, {}", k, v);
		}
		write!(f, ">")
	}
}

/// Used to convert a Node to JSON
impl ToJson for Node {
	fn to_json(&self) -> Json {
		let mut d = BTreeMap::new();
		let mut props = BTreeMap::new();
		d.insert("id".to_owned(), self.id.to_json());
		for (&ref k, &ref v) in self.props.iter() {
			props.insert(*k.clone(), Json::String(*v.clone()));
		}
		d.insert("props".to_owned(), Json::Object(props));
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

/*** Edge ***/

/// Used to give a string representation for Edge
impl fmt::Display for Edge {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Edge<labels={}>", self.labels.join(","))
	}
}

impl cmp::PartialEq for Edge {
	fn eq(&self, other: &Edge) -> bool {
		self.labels == other.labels
	}
}

/*** Graph ***/

/// Used to give a string representation for Graph
// impl fmt::Display for Graph {
// 	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
// 		write!(f.buf, "Graph (\n\tnodes = {}\n\tedges = {}\n)", self.nodes, self.edges)
// 	}
// }

impl Graph {

	/// Creates a new graph with empty BTreeMaps
	pub fn new() -> Graph {
		Graph { nodes: BTreeMap::new(), edges: BTreeMap::new() }
	}

	/// Encode and decode from file

	pub fn read_from_file(name: String) -> Graph {
		let mut contents = String::new();
		let mut file:File = File::open(name).unwrap();

		file.read_to_string(&mut contents).unwrap();
		let jsonstring = match Json::from_str(&contents) {
			Ok(a) => a,
			Err(e) => panic!("Error reading JSON string: {}", e)
		};

		let mut decoder = Decoder::new(jsonstring);
		match Decodable::decode(&mut decoder) {
			Ok(x) => x,
			Err(e) => panic!("Could not decode to graph: {}", e)
		}
	}

	pub fn write_to_file(&self, name: &'static str) -> Result<usize> {
		let mut file = try!(File::create(name));
		file.write(self.to_json().to_string().into_bytes().as_slice())
	}

	/*******************/
	/****** Nodes ******/
	/*******************/

	/// Gets the next node index
	pub fn next_node_index(&self) -> NodeIndex {
		self.nodes.len() + 1
	}

	/// Adds a Node to the Graph with a vector of attributes and a BTreeMap of properties
	pub fn add_node(&mut self, attr: &[&str], props: BTreeMap<Box<String>, Box<String>>) -> NodeIndex {
		let idx = self.next_node_index();
		self.nodes.insert(idx, Node { id: idx, props: props });
		// TODO This is weird
		for a in attr.iter() {
			self.add_edge(idx, idx, a.to_string());
		}
		idx
	}

	/// Removes a Node from the Graph given a NodeIndex
	pub fn remove_node(&mut self, idx: NodeIndex) -> Option<Node> {
		// TODO should also remove the attached edges
		match self.nodes.remove(&idx) {
			Some(n) => Some(n),
			None => None
		}
	}


	/// Get Nodes with a given attribute
	pub fn nodes_with_attr<'a>(&'a self, attr: &String) -> Vec<&Node> {
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
	pub fn nodes_with_prop<'a>(&'a self, key: &String, value: &String) -> Vec<&Node> {
		self.nodes.values().filter(|node|
			node.props.iter().find(|&(k, v)| **k == *key && **v == *value ).is_some()).collect()
	}

	/*******************/
	/****** Edges ******/
	/*******************/

	/// Add an Edge to the graph from the source NodeIndex to the target NodeIndex with a label
	pub fn add_edge<'a>(&'a mut self, source: NodeIndex, target: NodeIndex, label: String) -> & mut Edge {
		//map.entry(key).get().unwrap_or_else(|v| v.insert(default))
		let src_map = self.edges.entry(source).or_insert(BTreeMap::new());
		match src_map.entry(target) {
			Entry::Vacant(e) => {e.insert(Edge { labels: vec![label] });},
			Entry::Occupied(mut e) => if e.get().labels.contains(&label) { e.get_mut().labels.push(label); } else {()}
		};
		src_map.get_mut(&source).unwrap()
	}

	/// Get Edges with a given label
	pub fn edges_with_label<'a>(&'a self, label: &String) -> BTreeMap<&'a NodeIndex, BTreeMap<&'a NodeIndex, &'a Edge>> {
		let mut edges = BTreeMap::<&NodeIndex, BTreeMap<&NodeIndex, &Edge>>::new();
		for (src, map) in self.edges.iter() {
			let filtered: BTreeMap<&NodeIndex, &Edge> = map.iter().filter(|&(_idx, edge)| edge.labels.contains(label)).collect();
			if filtered.len() > 0 {
				edges.insert(src, filtered);
			}
		}
		edges
	}

	/// Get Nodes with a given label from a source NodeIndex
	pub fn edges_with_label_from<'a>(&'a self, source: NodeIndex, label: String) -> Vec<&'a Node> {
		if let Some(edge) = self.edges.get(&source) {
		edge.iter().filter_map(
			|(&idx, edge)|
				if edge.labels.contains(&label) {
					self.nodes.get(&idx)
				} else {
					None
				}).collect()
			} else {
				vec![]
			}
	}

}
