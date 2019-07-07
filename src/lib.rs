//! It's the library!

//use std::io;

extern crate matrix;

use matrix::prelude::*;
use std::fmt;

pub struct Graph
{
	nodes: Vec<Node>,
	edges: Vec<Edge>,
}

struct Node
{
	id: usize,
	name: String,
}

struct Edge
{
	source: usize,
	destination: usize,
	cost: f64,
}

struct Path
{
	path: Graph,
}

impl Graph
{

	pub fn new () -> Graph
	{
		let n0: Node = Node { id: 0, name: "myFirstNode".to_string() };
		let n1: Node = Node { id: 1, name: "mySecondNode".to_string() };
		let n2: Node = Node { id: 2, name: "myThirdNode".to_string() };
		
		let e0: Edge = Edge { source: 0, destination: 1, cost: 2.0 };
		let e1: Edge = Edge { source: 0, destination: 2, cost: 3.0 };
		let e2: Edge = Edge { source: 1, destination: 2, cost: 0.5 };
		
		Graph { nodes: vec![n0,n1,n2], edges: vec![e0,e1,e2] }
	}
	
	fn makeGrid ( &self ) -> matrix::format::Compressed<f64>
	{
	
		let mut matrix = Compressed::zero( (self.nodes.len(),self.nodes.len() ) );
		
		for edge in &self.edges
		{
			matrix.set((edge.source, edge.destination), edge.cost);
		}
		
		matrix
	}

}

/*
impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //write!(f, "look here:");
        write!(f,
        	"{0: <10} | {1: <10} | {2: <10} | {3: <10}",
        	"total", "blanks", "comments", "code"
    	)
    }
}*/


impl fmt::Display for Graph
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
        let mut line : String = String::new();
        line.push_str("|               ");
        
        for current_node in &self.nodes
        {
        	line.push_str(" | ");
        	line.push_str(&current_node.name);
        }
        
        writeln!(f, "{}", line)
    }
}




