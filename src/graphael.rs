#![feature(macro_rules)]

extern crate collections;
use collections::HashMap;
use graph::Graph;
mod graph;


// Shorthand HashMap
// dict!({"yes": "1", "no": "0"}) => ~[("yes", "1"), ("no", "0")].iter().map(|&x| x).collect()
macro_rules! dict (
	({$($key:expr : $value:expr),*}) => ([$(($key, $value)),*].iter().map(|&x| x).collect())
)

fn main() {
	let mut graph = Graph::new();
}