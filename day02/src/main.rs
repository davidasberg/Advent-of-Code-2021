
use std::fs;

fn main() {
    //part_one();
    part_two();
}

fn part_two() {
     //read input from file and store in a string
     let input = fs::read_to_string("input.txt").expect("Error reading file");
     //split at newline and store in vector
     let input_vec: Vec<&str> = input.split("\n").collect();
 
     //for each line in vector split at space and store in vector
     let mut depth = 0;
     let mut horizontal_pos = 0;
     let mut aim = 0;
     for i in input_vec {
        let input_vec_split: Vec<&str> = i.split(" ").collect();
        //compare first element to determine direction
        if input_vec_split[0] == "forward" {
            //increment horizontal
            horizontal_pos += input_vec_split[1].parse::<i32>().unwrap();
            depth += aim*input_vec_split[1].parse::<i32>().unwrap();
        } else if input_vec_split[0] == "up" {
            //decrement depth
            aim -= input_vec_split[1].parse::<i32>().unwrap();
        } else if input_vec_split[0] == "down" {
            //decrement horizontal position
            aim += input_vec_split[1].parse::<i32>().unwrap();
        }
    }
    println!("{}", horizontal_pos * depth);
}

fn part_one() {
    //read input from file and store in a string
    let input = fs::read_to_string("input.txt").expect("Error reading file");
    //split at newline and store in vector
    let input_vec: Vec<&str> = input.split("\n").collect();

    //for each line in vector split at space and store in vector
    let mut depth = 0;
    let mut horizontal_pos = 0;
    for i in input_vec {
        let input_vec_split: Vec<&str> = i.split(" ").collect();
        //compare first element to determine direction
        if input_vec_split[0] == "forward" {
            //increment horizontal
            horizontal_pos += input_vec_split[1].parse::<i32>().unwrap();
        } else if input_vec_split[0] == "up" {
            //decrement depth
            depth -= input_vec_split[1].parse::<i32>().unwrap();
        } else if input_vec_split[0] == "down" {
            //decrement horizontal position
            depth += input_vec_split[1].parse::<i32>().unwrap();
        }
    }
    //print horizontal position multiplied by depth
    println!("{}", horizontal_pos * depth);
}
