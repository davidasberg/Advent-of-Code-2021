
use std::collections::HashMap;
use std::fs;


fn main() {
    //part_one();
    part_two();
}

fn part_two(){
    //read input from file 
    let input = fs::read_to_string("input.txt").expect("Error reading file");
    let edges = input.trim().split("\n").collect::<Vec<&str>>();
    let mut graph = Graph::new();
    for edge in edges {
        let edge = edge.split("-").collect::<Vec<&str>>();
        let node1 = edge[0].to_string();
        let node2 = edge[1].to_string();
        let is_small1 = node1 == node1.to_lowercase();
        let is_small2 = node2 == node2.to_lowercase();
        if node1 == "start" || node2 == "end" {
            graph.add_edge(Node::new(node1,is_small1), Node::new(node2,is_small2));
        }
        else if node1 == "end" || node2 == "start" {
            graph.add_edge(Node::new(node2,is_small1), Node::new(node1,is_small2));
        }
        else {
            graph.add_double_edge(Node::new(node1,is_small1), Node::new(node2,is_small2));
        }    }
    let paths = graph.get_paths(graph.get_node("start").unwrap().clone(), graph.get_node("end").unwrap().clone(), 2);
    println!("{}", paths.len());
}

fn part_one() {
    //read input from file 
    let input = fs::read_to_string("input.txt").expect("Error reading file");
    let edges = input.trim().split("\n").collect::<Vec<&str>>();
    let mut graph = Graph::new();
    for edge in edges {
        let edge = edge.split("-").collect::<Vec<&str>>();
        let node1 = edge[0].to_string();
        let node2 = edge[1].to_string();
        let is_small1 = node1 == node1.to_lowercase();
        let is_small2 = node2 == node2.to_lowercase();
        if node1 == "start" || node2 == "end" {
            graph.add_edge(Node::new(node1,is_small1), Node::new(node2,is_small2));
        }
        else if node1 == "end" || node2 == "start" {
            graph.add_edge(Node::new(node2,is_small1), Node::new(node1,is_small2));
        }
        else {
            graph.add_double_edge(Node::new(node1,is_small1), Node::new(node2,is_small2));
        }
    }
    let paths = graph.get_paths(graph.get_node("start").unwrap().clone(), graph.get_node("end").unwrap().clone(), 1);
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
    
    fn add_double_edge(&mut self, node1: Node, node2: Node) {
        self.add_node(node1.clone());
        self.add_node(node2.clone());
        self.nodes.get_mut(&node1).unwrap().push(node2.clone());
        self.nodes.get_mut(&node2).unwrap().push(node1.clone());
    }

    fn get_neighbors(&self, node: &Node) -> &Vec<Node> {
        self.nodes.get(node).unwrap()
    }

    fn get_node(&self, node: &str) -> Option<&Node> {
        self.nodes.iter().find(|(key, _)| key.name == node).map(|(key, _)| key)
    }
   
    //search algorithm that finds all possible paths
    //can only visit small node once and large nodes can be visited multiple times
    fn get_paths(&self, start: Node, end: Node, small_max_visitations: u32) -> Vec<Vec<Node>> {
   
        let mut stack: Vec<(Vec<Node>, HashMap<Node, bool>, i32)> = Vec::new();
        let mut complete_paths: Vec<Vec<Node>> = Vec::new();

        let mut visited = HashMap::new();
        visited.insert(start.clone(), true);

        stack.push((vec![start.clone()], visited, 1));

        while !stack.is_empty() {
            let (path, visited, small_visitations) = stack.pop().unwrap();
            let node = path.last().unwrap();
            if *node == end {
                complete_paths.push(path);
            } else {
                for neighbor in self.get_neighbors(node) {
                    if !neighbor.is_small || !visited.contains_key(&neighbor) || small_visitations < small_max_visitations as i32 {
                        let mut new_visited = visited.clone();
                        new_visited.insert(neighbor.clone(), true);
                        let mut new_path = path.clone();
                        new_path.push(neighbor.clone());
                        if neighbor.is_small && small_visitations < small_max_visitations as i32 && visited.contains_key(&neighbor){
                            stack.push((new_path, new_visited, small_visitations + 1));
                        }
                        else {
                            stack.push((new_path, new_visited, small_visitations));
                        }
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
            is_small,
        }
    }
}