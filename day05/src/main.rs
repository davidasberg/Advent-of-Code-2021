use std::fs;

fn main() {
    part_two();
}

fn part_two(){
    //read file into string
    let contents = fs::read_to_string("input.txt").expect("Error reading file");
    //split at delimiter and trim
    let mut lines = contents.split("\n").collect::<Vec<&str>>();
    lines = lines.into_iter().filter(|&x| x != "").collect::<Vec<&str>>();
    


    let mut grid: Vec<Vec<i32>> = Vec::new();

    for line in lines {
        //split at delimiter and trim whitespace
        let line_split = line.split("->").map(|x| x.trim()).collect::<Vec<&str>>();
        let first_pair = line_split[0].split(",").map(|x| x.trim().parse::<i32>().unwrap()).collect::<Vec<i32>>(); 
        let second_pair = line_split[1].split(",").map(|x| x.trim().parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let x1 = first_pair[0];
        let y1 = first_pair[1];
        let x2 = second_pair[0];
        let y2 = second_pair[1];
        let real_line = Line::new(x1, y1, x2, y2);
        let points: Vec<(i32,i32)> = real_line.get_points();

        
        for point in points {
            let x = point.0;
            let y = point.1;
            while grid.len() <= y as usize {
                grid.push(Vec::new());
            }
            while grid[y as usize].len() <= x as usize {
                grid[y as usize].push(0);
            }
            grid[y as usize][x as usize] += 1;
        }
    }

    let mut count = 0;
    for row in grid {
        for column in row {
            print!("{}", column);
            if column > 1 {
                count += 1;
            }
        }
        println!("");
    }
    println!("{}", count);
}

fn part_one() {
    //read file into string
    let contents = fs::read_to_string("input.txt").expect("Error reading file");
    //split at delimiter and trim
    let mut lines = contents.split("\n").collect::<Vec<&str>>();
    lines = lines.into_iter().filter(|&x| x != "").collect::<Vec<&str>>();
    


    let mut grid: Vec<Vec<i32>> = Vec::new();

    for line in lines {
        //split at delimiter and trim whitespace
        let line_split = line.split("->").map(|x| x.trim()).collect::<Vec<&str>>();
        let first_pair = line_split[0].split(",").map(|x| x.trim().parse::<i32>().unwrap()).collect::<Vec<i32>>(); 
        let second_pair = line_split[1].split(",").map(|x| x.trim().parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let x1 = first_pair[0];
        let y1 = first_pair[1];
        let x2 = second_pair[0];
        let y2 = second_pair[1];
        let real_line = Line::new(x1, y1, x2, y2);
        let points: Vec<(i32,i32)> = real_line.get_points_simple();

        
        for point in points {
            let x = point.0;
            let y = point.1;
            while grid.len() <= y as usize {
                grid.push(Vec::new());
            }
            while grid[y as usize].len() <= x as usize {
                grid[y as usize].push(0);
            }
            grid[y as usize][x as usize] += 1;
        }
    }

    let mut count = 0;
    for row in grid {
        for column in row {
            if column > 1 {
                count += 1;
            }
        }
    }
    println!("{}", count);

    
}   

struct Line {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}
impl Line {
    fn new(x1: i32, y1: i32, x2: i32, y2: i32) -> Line {
        Line {
            x1,
            y1,
            x2,
            y2,
        }
    }
    //get all horizontal, vertical or diagonal points on line
    fn get_points(&self) -> Vec<(i32,i32)> {
        let mut points: Vec<(i32,i32)> = Vec::new();
        if self.x1 == self.x2 {
            //if y1 < y2, then we want to go up
            if self.y1 < self.y2 {
                for y in self.y1..self.y2+1 {
                    points.push((self.x1, y));
                }
            } else {
                for y in self.y2..self.y1+1 {
                    points.push((self.x1, y));
                }
            }

        } else if self.y1 == self.y2 {
            if self.x1 < self.x2 {
                for x in self.x1..self.x2+1 {
                    points.push((x, self.y1));
                }
            } else {
                for x in self.x2..self.x1+1 {
                    points.push((x, self.y1));
                }
            }
        }
        else if (self.x1 - self.x2).abs() == (self.y1 - self.y2).abs() {
            //diagonal
            let mut x = self.x1;
            let mut y = self.y1;
            let dx = if self.x1 < self.x2 { 1 } else { -1 };
            let dy = if self.y1 < self.y2 { 1 } else { -1 };
            
            for _ in 0..(self.x1 - self.x2).abs()+1 {
                points.push((x, y));
                x += dx;
                y += dy;
            }
        }
        points
    }

    //get all horizontal or vertical points
    fn get_points_simple(&self) -> Vec<(i32,i32)> {
        let mut points: Vec<(i32,i32)> = Vec::new();
        if self.x1 == self.x2 {
            //if y1 < y2, then we want to go up
            if self.y1 < self.y2 {
                for y in self.y1..self.y2+1 {
                    points.push((self.x1, y));
                }
            } else {
                for y in self.y2..self.y1+1 {
                    points.push((self.x1, y));
                }
            }

        } else if self.y1 == self.y2 {

            if self.x1 < self.x2 {
                for x in self.x1..self.x2+1 {
                    points.push((x, self.y1));
                }
            } else {
                for x in self.x2..self.x1+1 {
                    points.push((x, self.y1));
                }
            }
        }
        points
    }
}

