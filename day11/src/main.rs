use std::fs;

fn main() {
    //part_one();
    part_two();
}

fn part_two(){ 
    //read file into a 2d vector of ints
    let input = fs::read_to_string("input.txt").expect("Unable to read file");
    let mut grid: Vec<Vec<i32>> = Vec::new();
    for line in input.lines() {
        let mut row: Vec<i32> = Vec::new();
        for c in line.chars() {
            //convert to int and push to row
            row.push(c.to_digit(10).unwrap() as i32);
        }
        grid.push(row);
    }
    let mut step_count = 1;
    while step(&mut grid) != grid.len() as i32 * grid[0].len() as i32 {
        step_count += 1;
    }   
    println!("{}", step_count);
}

fn part_one(){ 
    //read file into a 2d vector of ints
    let input = fs::read_to_string("input.txt").expect("Unable to read file");
    let mut grid: Vec<Vec<i32>> = Vec::new();
    for line in input.lines() {
        let mut row: Vec<i32> = Vec::new();
        for c in line.chars() {
            //convert to int and push to row
            row.push(c.to_digit(10).unwrap() as i32);
        }
        grid.push(row);
    }
    let mut num_flashes = 0;
    for _ in 0..100 {
        num_flashes += step(&mut grid);
    }   
    println!("{}", num_flashes);
}

fn step(grid: &mut Vec<Vec<i32>>) -> i32{
    // for y in 0..grid.len() {
    //     for x in 0..grid[y].len() {
    //         grid[y][x] += 1;
    //     }
    // }
    let mut already_flashed: Vec<Vec<bool>> = vec![vec![false; grid[0].len()]; grid.len()];
    //initiate stack with all x,y pairs
    let mut queue: Vec<(usize, usize)> = Vec::new();
    for y in (0..grid.len()).rev() {
        for x in (0..grid[y].len()).rev() {
            queue.push((y,x));
        }
    }
    let mut num_flashes = 0;
    while queue.len() > 0 {
        let (y,x) = queue.pop().unwrap();
        if already_flashed[y][x] {
            continue;
        }
        grid[y][x] += 1;
        if grid[y][x] > 9 {
            already_flashed[y][x] = true;
            num_flashes += 1;
            //get all neighbors
            let neighbors = get_neighbours(grid, y as i32, x as i32);
            for n in neighbors {
                queue.push(n);
            }
        }
    }
    //set all greater than 9s to 0
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] > 9 {
                grid[y][x] = 0;
            }
        }
    }
    num_flashes
}
fn get_neighbours(grid: &Vec<Vec<i32>>, y: i32, x: i32) -> Vec<(usize,usize)>{
    let mut neighbours: Vec<(usize,usize)> = Vec::new();

    //add all neighbors, but 
    for i in y-1..y+2 {
        for j in x-1..x+2 {
            if i == y && j == x || i < 0 || j < 0 || i >= grid.len() as i32||  j >= grid[0].len() as i32{
                continue;
            }
            neighbours.push((i as usize ,j as usize));
        }
    }
    neighbours
}
