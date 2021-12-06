
use std::fs::File;
use std::io::prelude::*;


fn main(){
    //part_one();
    part_two();
}

fn part_two(){
    //call read_file with filename and assign to variable
    let input = read_file("input.txt");
    // split input at whitespace
    let input_vec: Vec<&str> = input.split_whitespace().collect();
    // convert input_vec to a vector of ints
    let input_vec_int: Vec<i32> = input_vec.iter().map(|x| x.parse::<i32>().unwrap()).collect();

    let mut count: i32 = -1;
    let mut previous_sum: i32 = std::i32::MIN;
    //iterate over input_vec with index and value
    for i in 0..input_vec_int.len()-2{
        let mut sum: i32 = 0;
        for j in 0..3{
            sum += input_vec_int[i+j];
        }
        if sum > previous_sum{
            count += 1;
        }
        previous_sum = sum; 
    }


    println!("{}", count);

}

fn part_one() {
    //call read_file with filename and assign to variablegi
    let input = read_file("input.txt");
    // split input into array of string with whitespace as delimiter
    let input_array_str: Vec<&str> = input.split_whitespace().collect();
    // convert input_array to array of i32
    let input_array_i32: Vec<i32> = input_array_str.iter().map(|x| x.parse::<i32>().unwrap()).collect();

    //loop over input_array_i32 and count how many times a value is greater than the previous
    let mut count: i32 = -1;
    //let previous_value be smallest possible integer
    let mut previous_value = std::i32::MIN;
    for i in input_array_i32 {
        if i > previous_value {
            count += 1;
        }
        previous_value = i;
    }

    //print array of i32
    
    println!("{}", count);
}



// read a file into a string
fn read_file(filename: &str) -> String {
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file");
    contents
}
