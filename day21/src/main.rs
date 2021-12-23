
use std::fs;
use std::collections::HashMap;

fn main() {
    part_one();
    part_two();
}

fn get_input(file: &str) -> (u64,u64){
    let contents = fs::read_to_string(file).expect("Something went wrong reading the file");
    let mut lines = contents.lines();
    let line1 = lines.next().unwrap();
    let p1_start = line1.split(": ").nth(1).unwrap().parse::<u64>().unwrap();
    let line2 = lines.next().unwrap();
    let p2_start = line2.split(": ").nth(1).unwrap().parse::<u64>().unwrap();
    (p1_start,p2_start)
}

fn part_one(){
    let (p1_start, p2_start) = get_input("input.txt");
    let mut p1_pos = p1_start;
    let mut p2_pos = p2_start;
    let mut p1_score = 0;
    let mut p2_score = 0;
    let mut die = DeterministicDie::new(100);
    let mut turn = 1;
    while p1_score < 1000 && p2_score < 1000 {
        let player_turn = (turn - 1) % 2 + 1;
        let roll1 = die.roll();
        let roll2 = die.roll();
        let roll3 = die.roll();
        match player_turn {
            1 => {
                p1_pos = (p1_pos + roll1 + roll2 + roll3 - 1 ) % 10 + 1;
                p1_score += p1_pos;
            },
            2 => {
                p2_pos = (p2_pos + roll1 + roll2 + roll3 - 1 ) % 10 + 1;
                p2_score += p2_pos;
            }
            _ => {
                panic!("Invalid player turn");
            }
        }
        turn += 1;
    } 
    println!("Part one: {}", p1_score.min(p2_score)*die.rolls);
}

fn part_two() {
    let (p1_start, p2_start) = get_input("input.txt");
  
    let mut dice_cache: HashMap<u64, u64> = HashMap::new();
    for dice1 in 1..=3 {
        for dice2 in 1..=3 {
            for dice3 in 1..=3 {
                let roll = dice1 + dice2 + dice3;
                *dice_cache.entry(roll).or_default() += 1;
            }
        }
    }
    let mut states: HashMap<([u64;2], [u64;2], usize), u64> = HashMap::new();
    states.entry(([p1_start,p2_start], [0,0], 0)).or_insert(1);
    let mut wins: [u64; 2] = [0,0];
    while states.len() > 0 {
        let (positions, scores, turn) = states.keys().nth(0).copied().unwrap();
        let instance_count = states.remove(&(positions, scores, turn)).unwrap();
        let next_turn = if turn == 0 { 1 } else { 0 }; 
        for (roll, count) in dice_cache.iter() {
            let mut positions = positions.clone();
            let mut scores = scores.clone();
            let instance_count = instance_count * count;
            let new_pos = (positions[turn] + roll - 1) % 10 + 1;
            positions[turn] = new_pos;
            scores[turn] += new_pos;
            if scores[turn] >= 21 {
                wins[turn] += instance_count as u64;
                continue;
            }
            let key = (positions, scores, next_turn);
            *states.entry(key).or_insert(0) += instance_count;
        }
    } 
    println!("Part two: {}", wins.iter().max().copied().unwrap());
}

struct DeterministicDie{
    sides: u8,
    current_value: u64,
    rolls: u64,
}

impl DeterministicDie{
    fn new(sides: u8) -> DeterministicDie{
        DeterministicDie{
            sides: sides,
            current_value: 1,
            rolls: 0,
        }
    }

    fn roll(&mut self) -> u64 {
        let val = self.current_value;
        //increment current value
        //if greater than sides, reset to 1
        self.current_value += 1;  
        if self.current_value > self.sides as u64{
            self.current_value = 1;
        }
        self.rolls += 1;
        val
    }
}