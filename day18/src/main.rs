

//solution HEAVILY inspired by u/Chaosteil (almost all code is his by this point) 
//had a very similar solution to begin with,
//but it was a bit more complicated than I thought it would be
//and I used his solution to fix my own instead of burning hours with my bugs


use std::fs;
use std::fmt;

fn main() {
    let elements = get_input("input.txt");
    println!("{}", part_one(&elements));
    println!("{}", part_two(&elements));
}

fn get_input(file: &str) -> Vec<SnailFishElement> {
    fs::read_to_string(file)
        .expect("Something went wrong reading the file")
        .trim()
        .lines()
        .map(|line| parse(&mut line.trim().chars()))
        .collect()
}

fn parse<T>(input: &mut T) -> SnailFishElement
where
    T: Iterator<Item = char>,
{
    let mut snail_fish_elements: Vec<SnailFishElement> = Vec::new();
    while let Some(c) = input.next() {
        match c {
            '[' => {
                let element = parse(input);
                snail_fish_elements.push(element);
            }
            c @ '0'..='9' => {
                let element = SnailFishElement::Number(c.to_digit(10).unwrap() as i32);
                snail_fish_elements.push(element);
            }
            ',' => {}
            ']' => {
                return SnailFishElement::Pair(
                    Box::new(
                        (snail_fish_elements.swap_remove(0), 
                        snail_fish_elements.swap_remove(0))
                    )
                );
            }
            _ => panic!("Unexpected character"),
        }
    }
    snail_fish_elements.swap_remove(0)
}


fn part_one(elements: &Vec<SnailFishElement>) -> i32 {
    //print each element
    elements.iter().for_each(|element| {
        println!("{}", element);
    });


    let mut elements = elements.to_owned().into_iter();
    let mut new_element = SnailFishElement::Pair(Box::new((
        elements.next().unwrap().reduce(), 
        elements.next().unwrap().reduce()
    ))).reduce();
    println!("{}", new_element);
    
    for element in elements {
        new_element = SnailFishElement::Pair(Box::new((
            new_element,
            element.reduce()
        ))).reduce();
        println!("{}", new_element);
    }
    println!("{}", new_element);
    new_element.magnitude()

}

fn part_two(elements: &Vec<SnailFishElement>) -> i32 {
    //get the largest magnitude by trying all pairs of elements
    let mut largest_magnitude = 0;
    for x in 0..elements.len() {
        for y in 0..elements.len() {
            if x == y {
                continue;
            }
            let new_element = SnailFishElement::Pair(
                Box::new((
                    elements[x].to_owned().reduce(),
                    elements[y].to_owned().reduce()
                ))
            ).reduce();
            let new_mag = new_element.magnitude();
            if new_mag > largest_magnitude {
                largest_magnitude = new_mag;
            }
        }
    }
    largest_magnitude

}


#[derive(Debug, Clone)]
enum SnailFishElement {
    Number(i32),
    Pair(Box<(SnailFishElement, SnailFishElement)>),
}

impl fmt::Display for SnailFishElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SnailFishElement::Number(n) => write!(f, "{}", n),
            SnailFishElement::Pair(pair) => write!(f, "[{}, {}]", pair.0, pair.1),
        }
    }
}

impl SnailFishElement {
    fn new(number: i32) -> SnailFishElement {
        SnailFishElement::Number(number)
    }

    fn reduce(mut self) -> Self {
        loop {
            while self.explode(0).0 {}
            if self.split() {
                continue;
            }
            break;
        }       
        self
    }

    fn magnitude(&self) -> i32 {
        match self {
            SnailFishElement::Number(n) => *n,
            SnailFishElement::Pair(pair) => 3*pair.0.magnitude() + 2*pair.1.magnitude(),
        }
    }

    fn explode(&mut self, depth: i32) -> (bool, Option<i32>, Option<i32>) {
        match self {
            SnailFishElement::Number(_) => (false, None, None),
            SnailFishElement::Pair(p) => {
                if depth >= 4 {
                    if let SnailFishElement::Number(l) = p.0 {
                        if let SnailFishElement::Number(r) = p.1 {
                            *self = SnailFishElement::Number(0);
                            return (true, Some(l), Some(r));
                        }
                    }
                }
                let (l_exploded, left, mut right) = p.0.explode(depth + 1);
                if l_exploded {
                    if right.is_some() {
                        p.1.insert_to_left(right.unwrap());
                        right = None
                    }
                    return (true, left, right);
                }
                let (r_exploded, mut left, right) = p.1.explode(depth + 1);
                if r_exploded {
                    if left.is_some() {
                        p.0.insert_to_right(left.unwrap());
                        left = None
                    }
                    return (true, left, right);
                }
                (false, None, None)
            }
        }
    }

    fn insert_to_left(&mut self, value: i32) {
        match self {
            SnailFishElement::Number(ref mut l) => *l += value,
            SnailFishElement::Pair(p) => p.0.insert_to_left(value),
        }
    }

    fn insert_to_right(&mut self, value: i32) {
        match self {
            SnailFishElement::Number(ref mut r) => *r += value,
            SnailFishElement::Pair(p) => p.1.insert_to_right(value),
        }
    }

    fn split(&mut self) -> bool {
        match self {
            SnailFishElement::Number(num) => {
                if *num >= 10 {
                    *self = SnailFishElement::Pair(Box::new((
                        SnailFishElement::new(*num/2),
                        //number divided by 2 and rounded up
                        SnailFishElement::new(*num/2 + *num % 2)
                    )));
                    return true;
                }
                false
            }
            SnailFishElement::Pair(p) => p.0.split() || p.1.split(), 
        }
    }

}
