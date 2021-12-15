use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fs;

fn main() {
    //part_one();
    part_two();
}

fn part_two() {
    let grid = get_input_expanded("input.txt");
    let path = grid.dijkstra((0,0), (grid.width-1, grid.height-1));
    for i in 0..grid.width {
        for j in 0..grid.height {
            let value = grid.get(i,j).value;
            if path.contains(&(i,j)) {
                print!("\x1b[0;31m{}\x1b[0m", value);
            } else {
                print!("{}",value);
            }
        }
        println!();
    }
    //print total path risk
    let mut risk = 0;
    for i in 0..path.len() {
        risk += grid.get(path[i].0, path[i].1).value;
    }
    println!("Total risk: {}", risk);
}

fn part_one() {
    let grid = get_input("input.txt");
    let path = grid.dijkstra((0,0), (grid.width-1, grid.height-1));
    for i in 0..grid.width {
        for j in 0..grid.height {
            let value = grid.get(i,j).value;
            if path.contains(&(i,j)) {
                print!("\x1b[0;31m{}\x1b[0m", value);
            } else {
                print!("{}",value);
            }
        }
        println!();
    }
    //print total path risk
    let mut risk = 0;
    for i in 0..path.len() {
        risk += grid.get(path[i].0, path[i].1).value;
    }
    println!("Total risk: {}", risk);
}

fn get_input_expanded(file: &str) -> Grid {
    let grid = get_input(file);
    let mut expanded = Grid::new();
    for i in 0..grid.width {
        for j in 0..grid.height {
            for k in 0..5 {
                for l in 0..5 {
                    // modulo to wrap around values in grid, only allow values between 1 and 9 (inclusive)
                    let value = (grid.get(i,j).value + k + l - 1) % 9 + 1;
                    expanded.set(i + grid.width*k as usize, j + grid.height*l as usize, value);
                }
            }
        }
    }
    expanded
}
fn get_input(file: &str) -> Grid {
    let mut grid = Grid::new();
    //read from file into grid
    let input = fs::read_to_string(file).expect("Unable to read file");
    for (i,line) in input.lines().enumerate() {
        for (j,c) in line.chars().enumerate() {
            //parse to i32
            let d = c.to_digit(10).unwrap() as i32;
            grid.set(j, i, d);
        }
    }
    grid
}

#[derive(Debug,Copy,Clone,Hash,Eq,PartialEq)]
struct Square {
    x: i32,
    y: i32,
    value: i32,
}

impl Ord for Square {
    fn cmp(&self, other: &Square) -> std::cmp::Ordering {
        other.value.cmp(&self.value)
    }
}

impl PartialOrd for Square {
    fn partial_cmp(&self, other: &Square) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone)]
struct Grid {
    width: usize,
    height: usize,
    grid: Vec<Vec<Square>>,
}

impl Grid {

    fn new() -> Grid {
        Grid {
            width: 0,
            height: 0,
            grid: Vec::new(),
        }
    }
    
    fn set(&mut self, x: usize, y: usize, value: i32) {
        if x >= self.width {
            self.width = x + 1;
        }
        if y >= self.height {
            self.height = y + 1;
        }
        while self.grid.len() <= y {
            self.grid.push(Vec::new());
        }
        while self.grid[y].len() <= x {
            self.grid[y].push(Square {
                x: 0,
                y: 0,
                value: 0,
            });
        }
        self.grid[y][x] = Square {
            x: x as i32,
            y: y as i32,
            value: value,
        };
    }

    fn get(&self, x: usize, y: usize) -> Square {
        if x >= self.width || y >= self.height {
            panic!("Out of bounds");
        }
        self.grid[y][x]
    }

    //get all neighbors of a given point, not diagonals
    fn neighbors(&self, x: usize, y: usize) -> Vec<(usize,usize)> {
        let mut neighbors = Vec::new();
        if x > 0 {
            neighbors.push((x-1, y));
        }
        if x < self.width - 1 {
            neighbors.push((x+1, y));
        }
        if y > 0 {
            neighbors.push((x, y-1));
        }
        if y < self.height - 1 {
            neighbors.push((x, y+1));
        }
        neighbors
    }

    //reconstruct path from prev
    fn reconstruct_path(&self, prev: HashMap<(usize, usize),(usize, usize)>, goal: (usize, usize)) -> Vec<(usize, usize)> {
        let mut path = Vec::new();
        let mut current = goal;
        while current != (0,0) {
            path.push(current);
            current = prev[&current];
        }
        path.reverse();
        path
    }

    //dijkstra's algorithm using min heap
    fn dijkstra(&self, start: (usize,usize), end: (usize,usize)) -> Vec<(usize,usize)> {
        //for each point in grid, set distance to infinity
        let mut distances = HashMap::new();
        let mut prev: HashMap<(usize,usize), (usize,usize)> = HashMap::new();
        for i in 0..self.width {
            for j in 0..self.height {
                distances.insert((i,j), std::i32::MAX);
            }
        }
        let mut heap = BinaryHeap::new();
        distances.insert(start, 0);
        heap.push(Square {
            x: start.0 as i32,
            y: start.1 as i32,
            value: 0,
        });
        while let Some(Square{x, y, value}) = heap.pop() {
            if x == end.0 as i32 && y == end.1 as i32 {
                return self.reconstruct_path(prev, (x as usize ,y as usize));
            }
            for neighbor in self.neighbors(x as usize, y as usize) {
                let next = Square {
                    x: neighbor.0 as i32,
                    y: neighbor.1 as i32,
                    value: value + self.get(neighbor.0, neighbor.1).value,
                };
                if next.value < distances[&(next.x as usize, next.y as usize)]{
                    distances.insert((next.x as usize, next.y as usize), next.value);
                    heap.push(next);
                    prev.insert(neighbor, (x as usize, y as usize));
                }
               
            }
        }
        Vec::new()
    }

    

    // //dijsktras algorithm to find the shortest path
    // fn dijkstra(&self, start: (usize, usize), goal: (usize, usize)) -> Vec<(usize, usize)> {
    //     let mut dist: Vec<Vec<usize>> = Vec::new();
    //     let mut prev: HashMap<(usize, usize),(usize, usize)> = HashMap::new();
    //     let mut set: HashSet<(usize, usize)> = HashSet::new();
    //     for i in 0..self.width {
    //         dist.push(Vec::new());
    //         for j in 0..self.height {
    //             dist[i].push(usize::MAX);
    //             set.insert((i,j));
    //         }
    //     }
    //     dist[start.1][start.0] = 0;
    //     while !set.is_empty() {
    //         let mut min = usize::MAX;
    //         let mut min_index = (0,0);
    //         for i in 0..self.width {
    //             for j in 0..self.height {
    //                 if set.contains(&(i,j)) && dist[j][i] < min {
    //                     min = dist[j][i];
    //                     min_index = (i,j);
    //                 }
    //             }
    //         }
    //         set.remove(&min_index);
    //         if min_index == goal {
    //             return self.reconstruct_path(prev, min_index);
    //         }
    //         for neighbor in self.neighbors(min_index.0, min_index.1) {
    //             if set.contains(&neighbor) {
    //                 let alt = min + self.get(neighbor.0, neighbor.1).value as usize;
    //                 if alt < dist[neighbor.1][neighbor.0] {
    //                     dist[neighbor.1][neighbor.0] = alt;
    //                     prev.insert(neighbor, min_index);
    //                 }
    //             }
    //         }
    //     }
    //     Vec::new()
    // }

    // //A* algorithm to find the shortest path, using heuristic
    // fn a_star(&self, start: (usize, usize), goal: (usize, usize)) -> Vec<(usize, usize)> {
    //     let mut open_set: HashSet<(usize, usize)> = HashSet::new();
    //     let mut prev: HashMap<(usize, usize),(usize, usize)> = HashMap::new();
    //     let mut g_score: HashMap<(usize, usize),i32> = HashMap::new();
    //     let mut f_score: HashMap<(usize, usize),i32> = HashMap::new();
    //     for i in 0..self.width {
    //         for j in 0..self.height {
    //             g_score.insert((i,j), i32::MAX);
    //             f_score.insert((i,j), i32::MAX);
    //         }
    //     }
    //     g_score.insert(start, 0);
    //     f_score.insert(start, self.heuristic(start, goal));
    //     open_set.insert(start);
    //     while !open_set.is_empty() {
    //         //get smallest f_score given that it is in the open set
    //         let current = *f_score.iter().filter(|&(k,v)| open_set.contains(k)).min_by(|&(_,v1), &(_,v2)| v1.cmp(v2)).unwrap().0;
    //         if current == goal {
    //             return self.reconstruct_path(prev, current);
    //         }
    //         open_set.remove(&current);
    //         for neighbor in self.neighbors(current.0, current.1) {
    //             let tentative_g_score = g_score[&current] + self.get(neighbor.0, neighbor.1) as i32;
    //             if tentative_g_score < g_score[&neighbor] {
    //                 prev.insert(neighbor, current);
    //                 g_score.insert(neighbor, tentative_g_score);
    //                 f_score.insert(neighbor, g_score[&neighbor] + self.heuristic(neighbor, goal));
    //                 if !open_set.contains(&neighbor) {
    //                     open_set.insert(neighbor);
    //                 }
    //             }
    //         }
    //     }
    //     Vec::new()
    // }   


    // //heuristic function for manhattan distance
    // fn heuristic(&self, p: (usize,usize), goal: (usize,usize)) -> i32{
    //     let dx = (p.0 as i32) - (goal.0 as i32);
    //     let dy = (p.1 as i32) - (goal.1 as i32);
    //     dx.abs() + dy.abs()
    // }



    // //reconstruct path from prev
    // fn reconstruct_path(&self, prev: HashMap<(usize, usize),(usize, usize)>, goal: (usize, usize)) -> Vec<(usize, usize)> {
    //     let mut path = Vec::new();
    //     let mut current = goal;
    //     while current != (0,0) {
    //         path.push(current);
    //         current = prev[&current];
    //     }
    //     path.reverse();
    //     path
    // }

    // fn print(&self) {
    //     for i in 0..self.height {
    //         for j in 0..self.width {
    //             print!("{}", self.get(j, i));
    //         }
    //         println!();
    //     }
    // }

    

        
}