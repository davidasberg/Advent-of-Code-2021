
use std::collections::HashMap;
use std::fs;


fn main() {
    part_one();
}

fn part_one() {
    //read input from file 
    let input = fs::read_to_string("input2.txt").expect("Error reading file");
    let edges = input.trim().split("\n").collect::<Vec<&str>>();
    let mut graph = Graph::new();
    for edge in edges {
        let edge = edge.split("-").collect::<Vec<&str>>();
        let node1 = edge[0].to_string();
        let node2 = edge[1].to_string();
        let is_small1 = node1 == node1.to_lowercase();
        let is_small2 = node2 == node2.to_lowercase();
        graph.add_edge(Node::new(node1,is_small1), Node::new(node2,is_small2));
    }
    let paths = graph.get_paths(graph.get_node("start").unwrap().clone(), graph.get_node("end").unwrap().clone());
    println!("{}", paths.len());
}

struct Graph {
    nodes: HashMap<Node, Vec<Node>>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            nodes: HashMap::new(),
        }
    }

    //insert only if node is not in the graph
    fn add_node(&mut self, node: Node) {
        if !self.nodes.contains_key(&node) {
            self.nodes.insert(node, Vec::new());
        }
    }

    fn add_edge(&mut self, node1: Node, node2: Node) {
        self.add_node(node1.clone());
        self.add_node(node2.clone());
        self.nodes.get_mut(&node1).unwrap().push(node2);
    }

    fn get_neighbors(&self, node: &Node) -> &Vec<Node> {
        self.nodes.get(node).unwrap()
    }

    fn get_node(&self, node: &str) -> Option<&Node> {
        self.nodes.iter().find(|(key, _)| key.name == node).map(|(key, _)| key)
    }
   
    //search algorithm that finds all possible paths
    //can only visit small node once and large nodes can be visited multiple times
    fn get_paths(&self, start: Node, end: Node) -> Vec<Vec<Node>> {
        let mut paths: Vec<Vec<Node>> = Vec::new();
        let mut visited = HashMap::new();
        let mut stack = Vec::new();
        let mut complete_paths: Vec<Vec<Node>> = Vec::new();

        paths.push(vec![start.clone()]);
        stack.push(start.clone());
        visited.insert(start.clone(), true);
        while !stack.is_empty() {
            let node = stack.pop().unwrap();
            let mut path = paths.pop().unwrap();
            if node == end {
                path.push(node.clone());
                complete_paths.push(path);
            } else {
                for neighbor in self.get_neighbors(&node) {
                    if !neighbor.is_small || (neighbor.is_small && !visited.contains_key(&neighbor)) {
                        visited.insert(neighbor.clone(), true);
                        stack.push(neighbor.clone());
                        path.push(neighbor.clone());
                        paths.push(path.clone());
                    }
                }
            }
        }
        complete_paths
    }

    fn print_graph(&self) {
        for (key, value) in &self.nodes {
            println!("{} -> {:?}", key.name, value.iter().map(|x| x.name.clone()).collect::<Vec<String>>());
        }
    }

}


#[derive(PartialEq, Eq, Hash, Clone)]
struct Node {
    name: String,
    is_small: bool,
}

impl Node {
    fn new(name: String, is_small: bool) -> Self {
        Node{
            name,
            is_small
        }
    }
}