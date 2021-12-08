
use std::fs;
use std::collections::HashMap;

fn main() {
    part_two();
}


// how many times each segment appears in a 7 segment display (across all numbers)
// Number:      a b c d e f g
// Occurrences: 8 6 8 7 4 9 7

fn part_two(){
    //read file to string
    let contents = fs::read_to_string("input.txt").expect("Error reading file");
    //split on newline
    let lines = contents.trim().split("\n").collect::<Vec<&str>>();

    let mut output_sum = 0;
    for line in lines {
        let parts = line.split("|").map(|x| x.trim()).collect::<Vec<&str>>();
        let digits: Vec<&str> = parts[0].split(" ").collect();
        let output: Vec<&str> = parts[1].split(" ").collect();

        //get first (and only) element with len 2
        let one = *digits.iter().find(|x| x.len() == 2).unwrap();
        //get first element with len 4
        let four = *digits.iter().find(|x| x.len() == 4).unwrap();
        //get first element with len 3
        let seven = *digits.iter().find(|x| x.len() == 3).unwrap();
        //get first element with len 8
        let eight = *digits.iter().find(|x| x.len() == 7).unwrap();

        //create map from char to char
        let mut map: Vec<(char, char)> = Vec::new();


        //get chars that not shared by one and seven, should only be one element
        //and the char should be 'a', whichever we get should be mapped to 'a'
        let a = get_diff(one, seven).chars().collect::<Vec<char>>()[0];
        map.push((a, 'a'));

        //get the char that appears exactly 6 times in the vector
        //and the char should be 'b', whichever we get should be mapped to 'b'
        let b = get_exact(&digits, 6);
        map.push((b, 'b'));

        //get the char that appears exactly 4 times in the vector
        //and the char should be 'e', whichever we get should be mapped to 'e'
        let e = get_exact(&digits, 4);
        map.push((e, 'e'));

        //get the char that appears exactly 9 times in the vector
        //and the char should be 'f', whichever we get should be mapped to 'f'
        let f = get_exact(&digits, 9);
        map.push((f, 'f'));

        //get the char that is in seven, but neither a nor f, must be c
        let c = get_diff(seven, &get_unmapped_string("af", &map)).chars().collect::<Vec<char>>()[0];
        map.push((c, 'c'));

        //get the char that is in 4, but neither b, c nor f, must be d
        let d = get_diff(four, &get_unmapped_string("bcf", &map)).chars().collect::<Vec<char>>()[0];
        map.push((d, 'd'));

        //get the char that is in 8, but neither a, b, c, d, e, f must be g
        let g = get_diff(eight, &get_unmapped_string("abcdef",&map)).chars().collect::<Vec<char>>()[0];
        map.push((g, 'g'));

        let mut row_output = String::new();
        for i in 0..output.len() {
            let mapped_string = get_mapped_string(output[i], &map);
            let digit = get_digit_from_chars(&mapped_string).to_string();
            row_output.push_str(digit.as_str());
        }
        output_sum += row_output.parse::<i32>().unwrap();
    }
    println!("{}", output_sum);
}

//given a string, return the correct digit
fn get_digit_from_chars(s: &str) -> i32 {
    //sort s
    let mut chars = s.chars().collect::<Vec<char>>();
    chars.sort();
    //get string from sorted chars
    let sorted_string = chars.iter().collect::<String>();
    match sorted_string.as_str() {
        "abcefg" => 0,
        "cf" => 1,
        "acdeg" => 2,
        "acdfg" => 3,
        "bcdf" => 4,
        "abdfg" => 5,
        "abdefg" => 6,
        "acf"  => 7,
        "abcdefg" => 8,
        "abcdfg" => 9,
        _ => -1,
    }
}

fn get_unmapped_string(s: &str, map: &Vec<(char,char)>) -> String {
    let mut result = String::new();
    for c in s.chars() {
        let mapped_char = map.iter().find(|x| x.1 == c).unwrap();
        result.push(mapped_char.0);
    }
    result
}

//given a string and a map, return the string mapped accordingly
fn get_mapped_string(s: &str, map: &Vec<(char,char)>) -> String {
    let mut result = String::new();
    for c in s.chars() {
        let mapped_char = map.iter().find(|x| x.0 == c).unwrap();
        result.push(mapped_char.1);
    }
    result
}


//given a number n and a vector, return the char that appears exactly n times
fn get_exact(v: &Vec<&str>, n: usize) -> char {
    let mut count = HashMap::new();
    for i in v {
        for j in i.chars() {
            *count.entry(j).or_insert(0) += 1;
        } 
        
    }
    for (c, count) in count {
        if count == n {
            return c;
        }
    }
    panic!("Could not find");
}

//given two strings, return all chars that are in one but not in the other and vice versa
fn get_diff(one: &str, two: &str) -> String {
    let mut diff = String::new();
    for c in one.chars() {
        if !two.contains(c) {
            diff.push(c);
        }
    }
    for c in two.chars() {
        if !one.contains(c) && !diff.contains(c) {
            diff.push(c);
        }
    }
    diff
}

fn part_one() {
    //read file to string
    let contents = fs::read_to_string("input.txt").expect("Error reading file");
    //split on newline
    let lines = contents.trim().split("\n").collect::<Vec<&str>>();

    let mut counter = 0;
    for line in lines {
        let parts = line.split("|").map(|x| x.trim()).collect::<Vec<&str>>();
        let _digits: Vec<&str> = parts[0].split(" ").collect();
        let output: Vec<&str> = parts[1].split(" ").collect();
        for i in output {
            // must be a 1, 7, 4, 8
            if i.len() == 2 || i.len() == 3 || i.len() == 4 || i.len() == 7 {
                counter += 1;
            }
        }
    }
    println!("{}", counter);
}

