use std::fs;

fn main() {
    //part_one();
    part_two();
}

fn part_two() {
    //read from file to string
    let input = fs::read_to_string("input.txt").expect("Error reading file");
    let mut lines: Vec<&str> = input
        .trim()
        .split("\n")
        .collect();
    let mut scores: Vec<i64> = Vec::new(); 
    
    for line in lines {
        let stack = get_remaining_stack(line);
        if stack.is_none() {
            continue;
        }
        let stack = stack.unwrap();
        
        let mut score: i64 = 0;
        for c in stack.chars().rev() {
            score *= 5;
            if c == '(' {
                score += 1;
            }
            else if c == '[' {
                score += 2;
            }
            else if c == '{' {
                score += 3;
            }
            else if c == '<' {
                score += 4;
            }

        }
        scores.push(score);
    }
    //get exact middle of scores
    let mut scores: Vec<i64> = scores.clone();
    scores.sort();
    let middle = scores.len() / 2;
    println!("{}", scores[middle]);
}

fn get_remaining_stack(line: &str) -> Option<String> {
    let mut stack = Vec::new();
    for c in line.chars() {
        if c == '(' {
            stack.push(c);
        } else if c == ')' {
            if stack.pop() != Some('(') {
                return None;
            }
        }
        else if c == '[' {
            stack.push(c);
        } else if c == ']' {
            if stack.pop() != Some('[') {
                return None;
            }
        }
        else if c == '{' {
            stack.push(c);
        } else if c == '}' {
            if stack.pop() != Some('{') {
                return None;
            }
        }
        else if c == '<' {
            stack.push(c);
        } else if c == '>' {
            if stack.pop() != Some('<') {
                return None;
            }
        }
    }
    return Some(stack.into_iter().collect());
}



fn part_one() {
    //read from file to string
    let input = fs::read_to_string("input.txt").expect("Error reading file");
    let mut lines: Vec<&str> = input
        .trim()
        .split("\n")
        .collect();
    let mut total = 0; 
    for line in lines {
        total += get_first_incorrect_closing(line);
    }
    println!("{}", total);
}

//finds the first incorrect closing character
fn get_first_incorrect_closing(line: &str) -> i32 {
    let mut stack = Vec::new();
    for c in line.chars() {
        if c == '(' {
            stack.push(c);
        } else if c == ')' {
            if stack.pop() != Some('(') {
                return 3;
            }
        }
        else if c == '[' {
            stack.push(c);
        } else if c == ']' {
            if stack.pop() != Some('[') {
                return 57;
            }
        }
        else if c == '{' {
            stack.push(c);
        } else if c == '}' {
            if stack.pop() != Some('{') {
                return 1197;
            }
        }
        else if c == '<' {
            stack.push(c);
        } else if c == '>' {
            if stack.pop() != Some('<') {
                return 25137;
            }
        }
    }
    return 0;
}

