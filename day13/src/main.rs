use std::fs;

fn main() {
    //part_one();
    part_two();
}

fn part_two() {
    let input = fs::read_to_string("input.txt").expect("Error reading file");
    //split on empty line
    let input_lines: Vec<&str> = input.trim().split("\n\n").collect();
    let coordinates = input_lines[0].split("\n").collect::<Vec<&str>>();
    let folds = input_lines[1].split("\n").collect::<Vec<&str>>();
    let mut paper = Paper::new();
    for coord in coordinates {
        let mut coord_split = coord.split(",");
        let x = coord_split.next().unwrap().trim().parse::<i32>().unwrap();
        let y = coord_split.next().unwrap().trim().parse::<i32>().unwrap();
        paper.add_point(x, y);
    }
    paper.clone().print_paper();
    println!("");
    for fold in folds {
        let fold_split = fold.split(" ").collect::<Vec<&str>>();
        let fold = fold_split[2].split("=").collect::<Vec<&str>>();
        let axis = fold[0].trim();
        let value = fold[1].trim().parse::<i32>().unwrap();
        if axis == "x" {
            paper.fold_left(value as usize);
        } else {
            paper.fold_up(value as usize);
        }
        paper.clone().print_paper();
        println!("");
    }
}

fn part_one() {
    let input = fs::read_to_string("input.txt").expect("Error reading file");
    //split on empty line
    let input_lines: Vec<&str> = input.split("\n\n").collect();
    let coordinates = input_lines[0].split("\n").collect::<Vec<&str>>();
    let folds = input_lines[1].split("\n").collect::<Vec<&str>>();
    let mut paper = Paper::new();
    for coord in coordinates {
        let mut coord_split = coord.split(",");
        let x = coord_split.next().unwrap().trim().parse::<i32>().unwrap();
        let y = coord_split.next().unwrap().trim().parse::<i32>().unwrap();
        paper.add_point(x, y);
    }
    for fold in folds {
        let fold_split = fold.split(" ").collect::<Vec<&str>>();
        let fold = fold_split[2].split("=").collect::<Vec<&str>>();
        let axis = fold[0].trim();
        let value = fold[1].trim().parse::<i32>().unwrap();
        if axis == "x" {
            paper.fold_left(value as usize);
        } else {
            paper.fold_up(value as usize);
        }
        break;
    }
  
    //paper.print_paper();
    println!("{}", paper.count_dots());
    
    
}

#[derive(Clone)]
struct Paper {
    grid: Vec<Vec<bool>>,
    width: usize,
    height: usize,
}

impl Paper {
    fn new() -> Self {
        Self {
            grid: Vec::new(),
            width: 0,
            height: 0,
        }
    }

    fn add_point(&mut self, x: i32, y: i32) {
        while self.height <= y as usize {
            self.grid.push(vec![false; self.width]);
            self.height += 1;
        }
        while self.width <= x as usize {
            for i in 0..self.height {
                self.grid[i].push(false);
            }
            self.width += 1;
        }
        self.grid[y as usize][x as usize] = true;
    }

    //rows become columns and columns become rows
    fn transpose_grid( &mut self) {
        let mut new_grid = vec![vec![false; self.height]; self.width];
        for i in 0..self.height {
            for j in 0..self.width {
                new_grid[j][i] = self.grid[i][j];
            }
        }
        self.grid = new_grid;
        let temp = self.width;
        self.width = self.height;
        self.height = temp;
    }

    fn fold_left(&mut self, x: usize) {
        self.transpose_grid();
        self.fold_up(x);
        self.transpose_grid();
    }

    //fold the grid along the given axis
    //Because this is a horizontal line, fold the bottom half up. 
    //Some of the dots might end up overlapping after the fold is complete, 
    //but dots will never appear exactly on a fold line. 
    fn fold_up(&mut self, y: usize){
           
        let mut bottom_half = self.grid.split_off(y);
        
        if bottom_half.len() < self.grid.len() {
            bottom_half.resize(self.grid.len(), vec![false; self.width]);
        }
        
        if bottom_half.len() > self.grid.len() {
            self.grid.resize(bottom_half.len(), vec![false; self.width]);
        }
        bottom_half.reverse();


        for (i, row) in self.grid.iter_mut().enumerate() {
            for (j, col) in row.iter_mut().enumerate() {
                *col = *col || bottom_half[i][j];
            }
        }
        self.grid.pop();
        self.height = self.grid.len();
}

    fn count_dots(&self) -> usize {
        let mut count = 0;
        for row in self.grid.iter() {
            for dot in row.iter() {
                if *dot {
                    count += 1;
                }
            }
        }
        count
    }

    fn print_paper(self) {
        for row in self.grid {
            for point in row {
                if point {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!("");
        }
    }

    

}