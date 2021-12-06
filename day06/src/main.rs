use std::fs;

fn main() {
    //part_one(80);
    part_two(256);
}

fn part_two(days: i32) {
    let input = fs::read_to_string("input.txt").expect("Error reading file");
    let fish_init = input.trim().split(",").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let mut phases: Vec<i64> = vec![0; 9];
    for f in fish_init {
        phases[f as usize] += 1;
    }
    println!("Day 0: {:?}", phases);
    for i in 1..days+1 {
        let phase_0 = phases[0];
        for i in 0..phases.len()-1 {
            phases[i] = phases[i+1];
        } 
        phases[8] = phase_0;
        phases[6] += phase_0;
        println!("Day {}: {:?}", i, phases);
    }

    println!("{}", phases.iter().sum::<i64>());

}

fn part_one(days: i32) {
    //read from file and split at comma
    let input = fs::read_to_string("input.txt").expect("Error reading file");
    let mut fish = input.trim().split(",").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    for _ in 0..days {
        let mut new_fish = Vec::new();
        for f in fish.iter_mut(){
            if *f == 0{
                new_fish.push(8);
                *f = 7;
            }
            *f -= 1;
            new_fish.push(*f);
        }
        fish = new_fish;
    }
    println!("{}", fish.len());
}
    


