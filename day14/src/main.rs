
use std::collections::HashMap;
use std::fs;



fn main() {
    let (template, rules) = get_input("input.txt");
    solve(template.clone(), &rules, 10);
    solve(template.clone(), &rules, 40);
}


fn solve(template: String, rules: &HashMap<(char,char), char>, steps: i32) {

    let mut bucket: HashMap<(char,char), i64> = HashMap::new();
    //for every rule in the ruleset, add the rule to the bucket with value 0
    for rule in rules.iter() {
        bucket.insert(*rule.0, 0);
    }

    let mut char_occurrences: HashMap<char,i64> = HashMap::new();

    let chars = template.chars().collect::<Vec<char>>();
    for c in 0..chars.len() {
        let new_char = char_occurrences.entry(chars[c]).or_insert(0);
        *new_char += 1;
        if c == template.len() - 1 {
            break;
        }
        if rules.contains_key(&(chars[c], chars[c+1])) {
            //add to bucket
            let count = bucket.entry((chars[c], chars[c+1])).or_insert(0);
            *count += 1;
        }
    }

    

    for _ in 0..steps{
        let mut new_bucket: HashMap<(char,char),i64> = bucket.clone();
        //set all values to 0
        for rule in rules.iter() {
            new_bucket.insert(*rule.0, 0);
        }
        for rule in rules.iter() {

            if *bucket.get(rule.0).unwrap() == 0 {
                continue;
            }
            let (c1,c3) = *rule.0;
            let c2 = *rule.1;

            let inc = *bucket.get(&(c1,c3)).unwrap();

            //increment (c1,c2) 
            let count = new_bucket.entry((c1,c2)).or_insert(0);
            *count += inc;

            //increment (c2,c3)
            let count = new_bucket.entry((c2,c3)).or_insert(0);
            *count += inc;

            let new_char = char_occurrences.entry(c2).or_insert(0);
            *new_char += inc;

        }
        bucket = new_bucket;
    }

    //get largest and smallest occurences of char
    let mut largest_occurence = 0;
    let mut smallest_occurence = std::i64::MAX;
    for occurence in char_occurrences.values() {
        if *occurence > largest_occurence {
            largest_occurence = *occurence;
        }
        if *occurence < smallest_occurence {
            smallest_occurence = *occurence;
        }
    }
    println!("Largest: {}", largest_occurence);
    println!("Smallest: {}", smallest_occurence);
    println!("Difference: {}", largest_occurence - smallest_occurence);
}



fn get_input(file: &str) -> (String, HashMap<(char,char), char>) {
    let input = fs::read_to_string(file).expect("Failed to read input");
    let blank_line_index = input.find("\n\n").expect("Failed to find blank line");
    let template = input[..blank_line_index].to_string();
    let mut rules = HashMap::new();
    input[blank_line_index..].trim().lines().for_each(|line| {
        let mut rule = line.split(" -> ");
        let left = rule.next().expect("Failed to find left side of rule").chars().collect::<Vec<_>>();
        let right = rule.next().expect("Failed to find right side of rule").chars().collect::<Vec<_>>();
        rules.insert((left[0], left[1]), right[0]);
    });
    (template, rules)
}