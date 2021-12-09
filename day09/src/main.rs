
use std::fs;

fn main(){
    //part_one();
    part_two();
}

fn part_two() {
    let input = fs::read_to_string("input.txt").expect("Unable to read file");
    let rows = input.trim().split("\n").collect::<Vec<&str>>();

    let mut grid: Vec<Vec<i32>> = Vec::new();
 
    for row in rows {
        let heights: Vec<i32> = row
                        .trim()
                        .chars()
                        .map(|x| x.to_digit(10).unwrap() as i32)
                        .collect();

        grid.push(heights); 
    }

    let all_local_min = get_local_min(&grid);

    let mut basin_sizes: Vec<i32> = get_basin_sizes(&grid, &all_local_min);
    basin_sizes.sort_by(|a, b| b.cmp(a));
    println!("{}", basin_sizes[0]*basin_sizes[1]*basin_sizes[2]);
}

fn get_basin_sizes(grid: &Vec<Vec<i32>>, all_local_min: &Vec<Vec<i32>>) -> Vec<i32> {
    let mut basin_sizes = Vec::new();
    for i in 0..all_local_min.len() {
        for j in 0..grid[i].len() {
            //if element is a local min
            if all_local_min[i][j] == 1{
                basin_sizes.push(get_basin_size(grid, i, j));
            }
        }
    }
    basin_sizes
}

//given a local min, find the size of the basin it belongs to, do a BFS until a nine is hit
fn get_basin_size(grid: &Vec<Vec<i32>>, i: usize, j: usize) -> i32 {
    let mut queue: Vec<(usize, usize)> = Vec::new();
    let mut visited: Vec<Vec<bool>> = vec![vec![false; grid[0].len()]; grid.len()];
    let mut basin_size: i32 = 0;


    queue.push((i, j));
    visited[i][j] = true;

    while queue.len() > 0 {
        let (i, j) = queue.pop().unwrap();
        basin_size += 1;

        if grid[i][j] == 9 {
            return basin_size;
        }

        let neighbors = get_neighbors(grid, i, j);

        for neighbor in neighbors {
            if !visited[neighbor.0][neighbor.1] && grid[neighbor.0][neighbor.1] != 9 {
                queue.push(neighbor);
                visited[neighbor.0][neighbor.1] = true;
            }
        }
    }
    basin_size
}





fn part_one() {
    //read input from file to string
    let input = fs::read_to_string("input.txt").expect("Error reading file");
    //split string into vector of strings
    //parse each char into vector of ints

    let rows = input.trim().split("\n").collect::<Vec<&str>>();

    let mut grid: Vec<Vec<i32>> = Vec::new();
 
    for row in rows {
        let heights: Vec<i32> = row
                        .trim()
                        .chars()
                        .map(|x| x.to_digit(10).unwrap() as i32)
                        .collect();

        grid.push(heights); 
    }
    

    let all_local_min = get_local_min(&grid);

    let mut sum = 0;
    //iterate over all elements in grid
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            //if element is a local min
            if all_local_min[i][j] == 1{
                sum += grid[i][j] + 1;
            }
        }
    }

    //return sum of all local min, plus 1 for each of the local min
    println!("{}", sum);
}

fn get_neighbor_values(grid: &Vec<Vec<i32>>, i: usize, j: usize) -> Vec<i32> {
    let neighbors = get_neighbors(grid, i, j);
    neighbors.iter().map(|x| grid[x.0][x.1]).collect::<Vec<i32>>()
}

fn get_neighbors(grid: &Vec<Vec<i32>>, row: usize, col: usize) -> Vec<(usize,usize)> {
    let mut neighbors = Vec::new();
    
    //get all neighbors, as long as they are in the grid
    if row > 0 {
        neighbors.push((row-1,col));
    }
    if row < grid.len() - 1 {
        neighbors.push((row + 1, col));
    }

    if col > 0 {
        neighbors.push((row, col - 1));
    }
    if col < grid[row].len() - 1 {
        neighbors.push((row,col + 1));
    }
    neighbors
}

//get all local min by checking all 8 neighbors
//if a element is lower than all neighbors, mark it as a local min in a 2d vector of zeroes and ones
fn get_local_min(grid: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut local_min = vec![vec![0; grid[0].len()]; grid.len()];
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            let neighbor_values = get_neighbor_values(grid, row, col);

            if grid[row][col] < *neighbor_values.iter().min().unwrap() {
                local_min[row][col] = 1;
            }
        }
    }
    local_min
}
