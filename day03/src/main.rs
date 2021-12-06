//include fs
use std::fs;

fn main() {

    //part_one();
    part_two();
}

fn part_two() {
    // read input
    let input = fs::read_to_string("input.txt")
    .expect("Failed to read input file");
    let input_list: Vec<&str> = input.split_whitespace().collect();
    
    //copy mutable input
    let mut input_list_copy = input_list.clone();
    let len = input_list_copy.len();
    for i in 0..len {
        let mut zeroes = 0;
        let mut ones = 0;
        for j in 0..input_list_copy.len() {
            if input_list_copy[j].chars().nth(i).unwrap() == '0' {
                zeroes += 1;
            } else if input_list_copy[j].chars().nth(i).unwrap() == '1' {
                ones += 1;
            }
        }
        //print list 
        for k in 0..input_list_copy.len() {
            println!("{}: {}",i, input_list_copy[k]);
        }
        println!("{}",zeroes);
        println!("{}",ones);
        if zeroes > ones {
            input_list_copy.retain(|&x| x.chars().nth(i).unwrap() == '0');
        } else if zeroes <= ones {
            input_list_copy.retain(|&x| x.chars().nth(i).unwrap() == '1');
        }
        if input_list_copy.len() == 1 {
            break;
        }
    }
    let mut oxygenGenRating = 0;
   
    oxygenGenRating = isize::from_str_radix(input_list_copy[0], 2).unwrap();
    
    println!("{}", oxygenGenRating);
    
    let mut input_list_copy = input_list.clone();
    for i in 0..len {
        let mut zeroes = 0;
        let mut ones = 0;
        for j in 0..input_list_copy.len() {
            if input_list_copy[j].chars().nth(i).unwrap() == '0' {
                zeroes += 1;
            } else if input_list_copy[j].chars().nth(i).unwrap() == '1' {
                ones += 1;
            }
        }
        if zeroes > ones {
            input_list_copy.retain(|&x| x.chars().nth(i).unwrap() == '1');
        } else if zeroes <= ones {
            input_list_copy.retain(|&x| x.chars().nth(i).unwrap() == '0');
        }
        if input_list_copy.len() == 1 {
            break;
        }
    }
    let mut c02Rating = 0;
    c02Rating = isize::from_str_radix(input_list_copy[0], 2).unwrap();
    
    println!("{}", c02Rating);

    println!("{}", oxygenGenRating*c02Rating);

}

fn part_one () {
    // read input
    let input = fs::read_to_string("input.txt")
    .expect("Failed to read input file");
    //split at whitespace and convert to list of strings
    let input_list: Vec<&str> = input.split_whitespace().collect();
    
    let mut bits: Vec<i32> = vec![];
   
    //iterate over input list with value and index

    for i in (0..input_list[0].len()).rev() {
        let mut zeroes = 0;
        let mut ones = 0;
        for j in 0..input_list.len() {
            if input_list[j].chars().nth(i).unwrap() == '0' {
                zeroes += 1;
            } else {
                ones += 1;
            }
        }
        if zeroes > ones {
            bits.push(0);
        } else {
            bits.push(1);
        }
    }

    //convert array of 0 and 1 to dec
    let mut dec = 0;
    for i in 0..bits.len() {
        dec += bits[i] * (2_i32.pow(i as u32));
    }

    let mut inv_bits: Vec<i32> = vec![];  
    //iterate over bits with index
    for i in bits.iter() {
        if *i == 0{
            inv_bits.push(1);
        }
        else {
            inv_bits.push(0);
        }
    }
    let mut dec_inv = 0;
    for i in 0..inv_bits.len() {
        dec_inv += inv_bits[i] * (2_i32.pow(i as u32));
    }
    println!("{}", dec);
    println!("{}", dec_inv);
    println!("{}", dec*dec_inv);

}   
