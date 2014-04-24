#![allow(unused_imports)]
#![allow(dead_code)]

use collections::HashMap;
use std::fmt;
use std::cmp;
use std::uint;
// use std::slice;
// use std::iter::Filter;
// use std::iter::Map;

pub struct Node {
	id: NodeIndex,
    pub props: HashMap<&'static str, &'static str>
}

pub struct Edge {
	pub labels: ~[&'static str]
}

pub struct Graph {
	pub nodes: HashMap<NodeIndex, Node>,
	pub edges: HashMap<NodeIndex, HashMap<NodeIndex, Edge>>
}

#[deriving(Eq, Hash, TotalEq)]
pub struct NodeIndex(pub uint);

impl NodeIndex {
	pub fn get(&self) -> uint { let NodeIndex(v) = *self; v }
}

// impl Hash<Writer> for NodeIndex {
// 	fn hash(&self, state: &mut Writer) {
// 		self.get().hash(state);
// 	}
// }

impl fmt::Show for Node {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f.buf, "Node<id={}, props={}>", self.id, self.props)
	}
}

impl fmt::Show for NodeIndex {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f.buf, "{}", self.get())
	}
}

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

impl fmt::Show for Graph {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f.buf, "Graph (\n\tnodes = {}\n\tedges = {}\n)", self.nodes, self.edges)
	}
}

impl Graph {
	pub fn new() -> Graph {
		Graph { nodes: HashMap::new(), edges: HashMap::new() }
	}

	/*******************/
	/****** Nodes ******/
	/*******************/

	pub fn next_node_index(&self) -> NodeIndex {
		NodeIndex(self.nodes.len() + 1)
	}

	pub fn add_node(&mut self, attr: ~[&'static str], props: HashMap<&'static str, &'static str>) -> NodeIndex {
		let idx = self.next_node_index();
		self.nodes.insert(idx, Node { id: idx, props: props });
		for a in attr.iter() {
			self.add_edge(idx, idx, *a);
		}
		idx
	}

	pub fn remove_node(&mut self, idx: NodeIndex) -> bool {
		self.nodes.remove(&idx)
	}

	// pub fn node_attributes<'a>(&'a mut self, idx: NodeIndex) -> &'a mut ~[&'static str] {
	// 	&mut self.nodes.get_mut(&idx).attr
	// }

	pub fn node_properties<'a>(&'a mut self, idx: NodeIndex) -> &'a mut HashMap<&'static str, &'static str> {
		&mut self.nodes.get_mut(&idx).props
	}

	pub fn nodes_with_attr(&self, attr: &'static str) -> ~[&Node] {
		self.nodes.values().filter(|node| 
			match self.edges.find(&node.id) {
				Some(map) => match map.find(&node.id) {
					Some(edge) => edge.labels.contains(&attr),
					None => false
				},
				None => false
			}).collect()
	}

	pub fn nodes_with_prop(&self, key: &'static str, value: &'static str) -> ~[&Node] {
		self.nodes.values().filter(|node|
			match node.props.find(&key) {
				Some(v) => v == &value,
				None => false,
			}).collect()
	}

	/*******************/
	/****** Edges ******/
	/*******************/

	pub fn add_edge<'a>(&'a mut self, source: NodeIndex, target: NodeIndex, label: &'static str) -> &'a mut Edge {
		let src_map = self.edges.find_or_insert(source, HashMap::new());
		src_map.insert_or_update_with(target, 
			Edge { labels: ~[label] },
			|_k, v| if !v.labels.contains(&label) { v.labels.push(label) })
	}

	pub fn edges_with_label<'a>(&'a mut self, label: &'static str) -> HashMap<&'a NodeIndex, HashMap<&'a NodeIndex, &'a Edge>> {
		let mut edges = HashMap::<&NodeIndex, HashMap<&NodeIndex, &Edge>>::new();
		for (src, map) in self.edges.iter() {
			let filtered: HashMap<&NodeIndex, &Edge> = map.iter().filter(|&(_idx, edge)| edge.labels.contains(&label)).collect();
			if filtered.len() > 0 {
				edges.insert(src, filtered);
			}
		}
		edges
	}

	// pub fn edges_with_label<'a>(&'a mut self, label: &'static str) -> HashMap<NodeIndex, HashMap<&NodeIndex, &Edge>>  {
	// 	let mut edges = HashMap::<NodeIndex, HashMap<&NodeIndex, &Edge>>::new();

	// 	// self.edges.iter().filter_map(|_src, map|
	// 	// 	Some(map.iter().filter(|&(idx, edge)| edge.labels.contains(&label)))

	// 	for (&src, map) in self.edges.iter() {
	// 		let filtered: HashMap<&NodeIndex, &Edge> = map.iter().filter(|&(idx, edge)| edge.labels.contains(&label)).map(|(k, v)| (k, v)).collect();

	// 		edges.insert(src, filtered);

	// 		// let map: HashMap<NodeIndex, Edge> = self.edges_with_label_from(*src, label).iter().map(|&(&k, &v)| (k, v)).collect();
	// 		// for (tgt, edge) in map.iter() {
	// 		// 	if edge.labels.contains(&label) {
	// 		// 		edges.push((src, tgt, edge));
	// 		// 	}
	// 		// }
	// 	}

	// 	edges

	// 	// self.edges.iter().flat_map(|(&src, &map)| map.iter().filter(|&(idx, edge)| edge.labels.contains(&label))).collect()
	// }

	pub fn edges_with_label_from<'a>(&'a mut self, source: NodeIndex, label: &'static str) -> HashMap<&'a NodeIndex, &'a Edge> {
		self.edges.get(&source).iter().filter(|&(_idx, edge)| edge.labels.contains(&label)).collect()
	} 

	/*******************/
	/****** Paths ******/
	/*******************/

	// pub fn find_path(&'a mut )

}