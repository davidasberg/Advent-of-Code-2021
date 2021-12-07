use std::fs;

fn main() {
    part_two();
}

//very unoptimized
fn part_two() {
    //read input from file into a string
    let input = fs::read_to_string("input.txt").expect("Error reading file");
    //split string into a vector of ints, split at commas
    let crabs: Vec<i32> = input.trim().split(",").map(|x| x.parse::<i32>().unwrap()).collect();
    
    //get max of vec
    let max = *crabs.iter().max().unwrap();
    //get min of vec
    let min = *crabs.iter().min().unwrap();


    let mut fuel_usage = vec![0; (max-min) as usize + 1];
    for i in min..max+1 {
        for crab in crabs.iter() {
            fuel_usage[(i-min) as usize] += get_fuel_burnrate((i - crab).abs());
        }
    }
    
    let min_fuel = fuel_usage.iter().min().unwrap();
    println!("{}", min_fuel);
}

//get fuel burnrate given steps
fn get_fuel_burnrate(steps: i32) -> i32 {
    let mut fuel_burnrate = 0;
    for i in 1..steps+1 {
        fuel_burnrate += i;
    }
    fuel_burnrate
}


fn part_one() {
    //read input from file into a string
    let input = fs::read_to_string("input.txt").expect("Error reading file");
    //split string into a vector of ints, split at commas
    let crabs: Vec<i32> = input.trim().split(",").map(|x| x.parse::<i32>().unwrap()).collect();
    
    //get max of vec
    let max = *crabs.iter().max().unwrap();
    //get min of vec
    let min = *crabs.iter().min().unwrap();


    let mut fuel_usage = vec![0; (max-min) as usize + 1];
    for i in min..max+1 {
        for crab in crabs.iter() {
            fuel_usage[(i-min) as usize] += (i - crab).abs() as i32;
        }
    }
    
    let min_fuel = fuel_usage.iter().min().unwrap();
    println!("{}", min_fuel);
    
}