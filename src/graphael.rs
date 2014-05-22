#![feature(macro_rules)]

extern crate collections;
extern crate serialize;
// use std::io::net::tcp::TcpStream;
// use std::io::net::ip::{Ipv4Addr, SocketAddr};
use graph::Graph;
use std::io;
mod graph;


// Shorthand HashMap
// dict!({"yes": "1", "no": "0"}) => vec!($(($key, $value)),*).move_iter().collect();
macro_rules! dict (
	({$($key:expr : $value:expr),*}) => (vec!($(($key, $value)),*).move_iter().collect())
)

fn main() {
	println!("Graphael 0.1");

	print!("Enter a graph name> ");

	// Get the graph file name from stdin or use 'langs'
	let graph_file : ~str = match io::stdin().read_line() {
		Ok(s) => {
			println!("Using '{}'", &s.trim());
			s.trim().to_owned()
		},
		Err(e) => {
			println!("Error: {}.\nUsing 'langs'", e);
			~"langs"
		}
	};

	// Read a already filled database 
	let mut graph = Graph::read_from_file(format!("./data/{}.graph", graph_file));

	// Current state variable to keep track of which 
	// type of query we are doing
	let mut current_state = 0;

	println!("1. Nodes with attribute.");
	println!("2. Nodes with key-value.");
	println!("3. Edges with label.");
	println!("4. Edges from node with label.");
	println!("5. Look up node.");
	println!("");
	print!(">>> ")

	// Read from stdin
	for line in io::stdin().lines() {
		match line {
			Ok(s) => {
				match current_state {
					// Initial state
					0 => match from_str::<int>(s.trim()) {
						Some(1) => current_state = 1,
						Some(2) => current_state = 2,
						Some(3) => current_state = 3,
						Some(4) => current_state = 4,
						Some(5) => current_state = 5,
						_ => current_state = 0
					},
					1 => { // Nodes with attribute
						println!("{}", graph.nodes_with_attr(s.trim().to_owned()));
						current_state = 0
					},
					2 => { // Node by key-value pair
						let kv : ~[&str] = s.trim().split('=').map(|x| x.trim()).collect();
						println!("{}", graph.nodes_with_prop(kv[0].to_owned(), kv[1].to_owned()));
						current_state = 0
					},
					3 => { // Edges with label
						println!("{}", graph.edges_with_label(s.trim().to_owned()));
						current_state = 0
					},
					4 => { // Edges from node (NodeIndex) with label
						let node_label : ~[&str] = s.trim().split_str("HAS").map(|x| x.trim()).collect();
						println!("{}", graph.edges_with_label_from(from_str::<uint>(node_label[0].to_owned()).unwrap(), node_label[1].to_owned()));
						current_state = 0
					},
					5 => { // Look up node by id (NodeIndex)
						let nodeid = from_str::<uint>(s.trim()).unwrap();
						println!("{}", graph.nodes.find(&nodeid));
						current_state = 0
					},
					_ => { print!(">>> ") }
				}
			},
			Err(e) => fail!("Error: {}", e)
		};

		// Check the state and print accordingly
		match current_state {
			1 => print!("Enter attribute> "),
			2 => print!("Enter key-value> "),
			3 => print!("Enter label> "),
			4 => print!("Enter node and label> "),
			5 => print!("Enter node id> "),
			_ => {
				println!("");
				println!("1. Nodes with attribute.");
				println!("2. Nodes with key-value.");
				println!("3. Edges with label.");
				println!("4. Edges from node with label.");
				println!("5. Look up node by id.");
				println!("");
				print!(">>> ")
			}
		};
	}

}