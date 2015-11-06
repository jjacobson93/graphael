extern crate rustc_serialize;
extern crate graphael;
use std::io::{self, BufRead, Write};
use graphael::Graph;



// Shorthand HashMap
// dict!({"yes": "1", "no": "0"}) => vec!($(($key, $value)),*).move_iter().collect();
// macro_rules! dict (
// 	({$($key:expr : $value:expr),*}) => (vec!($(($key, $value)),*).move_iter().collect())
// );

fn main() {
	println!("Graphael 0.1");

	print!("Enter a graph name> ");
	io::stdout().flush().unwrap();

	// Get the graph file name from stdin or use 'langs'
	let mut input = String::new();
	let graph_file = match io::stdin().read_line(&mut input) {
		Ok(_) => {
			println!("Using '{}'", &input.trim());
			input.trim().to_owned()
		},
		Err(e) => {
			println!("Error: {}.\nUsing 'langs'", e);
			"langs".to_string()
		}
	};

	// Read a already filled database
	let graph = Graph::read_from_file(format!("./data/{}.graph", graph_file));

	// Current state variable to keep track of which
	// type of query we are doing
	let mut current_state = 0;

	println!("1. Nodes with attribute.");
	println!("2. Nodes with key-value.");
	println!("3. Edges with label.");
	println!("4. Edges from node with label.");
	println!("5. Look up node.");
	println!("");
	print!(">>> ");
	io::stdout().flush().unwrap();

	// Read from stdin
	let locked_in = io::stdin();
	for line in locked_in.lock().lines() {
		match line {
			Ok(s) => {
				match current_state {
					// Initial state
					0 => match s.trim().parse::<i32>() {
						Ok(1) => current_state = 1,
						Ok(2) => current_state = 2,
						Ok(3) => current_state = 3,
						Ok(4) => current_state = 4,
						Ok(5) => current_state = 5,
						_ => current_state = 0
					},
					1 => { // Nodes with attribute
						println!("{:?}", graph.nodes_with_attr(&s.trim().to_string()));
						current_state = 0
					},
					2 => { // Node by key-value pair
						let kv : Vec<&str> = s.trim().split('=').map(|x| x.trim()).collect();
						println!("{:?}", graph.nodes_with_prop(&kv[0].to_string(), &kv[1].to_string()));
						current_state = 0
					},
					3 => { // Edges with label
						println!("{:?}", graph.edges_with_label(&s.trim().to_string()));
						current_state = 0
					},
					4 => { // Edges from node (NodeIndex) with label
						let node_label : Vec<&str> = s.trim().split("HAS").map(|x| x.trim()).collect();
						println!("{:?}", graph.edges_with_label_from((node_label[0].to_owned().parse::<usize>()).unwrap(), node_label[1]));
						current_state = 0
					},
					5 => { // Look up node by id (NodeIndex)
						let nodeid = (s.trim().parse::<usize>()).unwrap();
						println!("{:?}", graph.get_node(nodeid));
						current_state = 0
					},
					_ => { print!(">>> ") }
				}
			},
			Err(e) => panic!("Error: {}", e)
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
		io::stdout().flush().unwrap();
	}

}
