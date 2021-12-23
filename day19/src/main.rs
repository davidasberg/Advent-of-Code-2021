

//this solution was taken from https://github.com/pavel1269/advent-of-code-2021/blob/main/src/day19/mod.rs
//this was too hard to solve, so I skipped it and used the solution from reddit

use std::fs;
use std::collections::{HashMap, HashSet};

type Point = (i64, i64, i64);
type Distance = Point;
type Scanner = HashSet<Point>;

fn main() {
    part_one();
    part_two();
}

fn get_input(file: &str) -> Vec<Scanner> {
    let input = fs::read_to_string(file).expect("Something went wrong reading the file");
    let mut id_counter = 0;
    let scanners: Vec<Scanner> = input.split("\n\n")
        .map(|s| {
            let beacons = s.lines().skip(1)
                .map(|line| {
                    id_counter += 1;
                    let mut line = line.trim().split(",");
                    let x = line.next().unwrap().parse::<i64>().unwrap();
                    let y = line.next().unwrap().parse::<i64>().unwrap();
                    let z = line.next().unwrap().parse::<i64>().unwrap();
                    (x,y,z)
                })
                .collect::<Vec<Point>>();
            let mut scanner = HashSet::new();
            for (x,y,z) in beacons {
                scanner.insert((x,y,z));
            }
            scanner
        }).collect::<Vec<Scanner>>();
    scanners
}

fn part_one() {
    let mut scanners = get_input("input.txt");
    println!("{}", count_beacons(&mut scanners));
}

fn part_two() {
    let mut scanners = get_input("input.txt");
    println!("{}", calculate_scanner_distance(&mut scanners));
}

fn calculate_scanner_distance(scanners: &mut Vec<Scanner>) -> i64 {
    let mut distances = Vec::new();
    while scanners.len() > 1 {
        match try_merge(&scanners) {
            Some((base_scanner_index, merged_scanner_index, distance, merged_scanner)) => {
                distances.push(distance);
                scanners.remove(merged_scanner_index);
                *scanners.get_mut(base_scanner_index).unwrap() = merged_scanner;
            },
            None => panic!(),
        }
    }

    let mut distance_max = 0;
    for (index, point1) in distances.iter().copied().enumerate() {
        for point2 in distances.iter().skip(index + 1).copied() {
            let distance = (point1.0 - point2.0).abs() + (point1.1 - point2.1).abs() + (point1.2 - point2.2).abs();
            if distance > distance_max {
                distance_max = distance;
            }
        }
    }

    return distance_max;
}

fn count_beacons(scanners: &mut Vec<Scanner>) -> i64 {

    while scanners.len() > 1 {
        match try_merge(&scanners) {
            Some((base_scanner_index, merged_scanner_index, _, merged_scanner)) => {
                // println!("  removing: {}", merged_scanner_index);
                scanners.remove(merged_scanner_index);
                *scanners.get_mut(base_scanner_index).unwrap() = merged_scanner;
            },
            None => panic!(),
        }
    }

    assert_eq!(1, scanners.len());
    let scanner = scanners.first().unwrap();
    return scanner.len() as i64;
}

fn try_merge(scanners: &Vec<Scanner>) -> Option<(usize,usize,Distance,Scanner)> {
    let mut merge_with = None;
    for (base_scanner_index, base_scanner) in scanners.iter().enumerate() {
        // println!("base scanner: {} ({}) vs ({})", base_scanner_index, base_scanner.len(), scanners.len());
        for (scanner2_index, scanner2) in
            scanners.iter().enumerate().skip(base_scanner_index + 1)
        {
            // println!("  testing: {}", scanner2_index);
            for scanner2 in rotate_scanner(&scanner2) {
                match try_match_points(base_scanner, &scanner2) {
                    None => continue,
                    Some(distance) => {
                        // println!("  match with {}", scanner2_index);
                        let mut moved_scanner = move_scanner(&scanner2, distance);
                        for point in base_scanner {
                            moved_scanner.insert(*point);
                        }
                        // println!("      {} + {} = {}", base_scanner.len(), scanner2.len(), moved_scanner.len());
                        merge_with = Some((base_scanner_index, scanner2_index, distance, moved_scanner));
                        break;
                    },
                }
            }

            if merge_with.is_some() {
                break;
            }
        }

        if merge_with.is_some() {
            break;
        }
    }
    return merge_with;
}

fn try_match_points(scanner1: &Scanner, scanner2: &Scanner) -> Option<Distance> {
    let mut distances_point: HashMap<Distance, HashSet<Point>> = HashMap::new();
    for point2 in scanner2.iter().copied() {
        for point1 in scanner1.iter().copied() {
            let diff = (
                point1.0 - point2.0,
                point1.1 - point2.1,
                point1.2 - point2.2,
            );
            distances_point.entry(diff).or_default().insert(point2);
        }
    }

    let matching_distances = distances_point
        .iter()
        .filter(|(_, points)| points.len() >= 12)
        .map(|(distance, _)| *distance)
        .collect::<Vec<Distance>>();

    if matching_distances.is_empty() {
        return None;
    }

    let distance = matching_distances.first().unwrap();
    return Some(*distance);
}

fn move_scanner(scanner: &Scanner, distance: Distance) -> Scanner {
    scanner
        .iter()
        .map(|(x, y, z)| (x + distance.0, y + distance.1, z + distance.2))
        .collect()
}

fn rotate_scanner(scanner: &Scanner) -> Vec<Scanner> {
    // Rotations -> all axises can be swaped (6 combinations) and neither or exactly two are negative (4 combinations)
    // rotating by x upwards x,y,z-> x,z,y -> x,-y,-z -> x,-z,-y -> x,y,z
    // rotating by z towards right x,y,z-> y,-x,z -> -x,-y,z -> x,y,z
    // rotating by y towards right x,y,z-> -z,y,x ->

    let rotations_axis = [
        |(x, y, z): Point| (x, y, z),
        |(x, y, z): Point| (x, z, y),
        |(x, y, z): Point| (y, x, z),
        |(x, y, z): Point| (y, z, x),
        |(x, y, z): Point| (z, x, y),
        |(x, y, z): Point| (z, y, x),
    ];
    let rotations_rotate = [
        |(x, y, z): Point| (x, y, z),
        |(x, y, z): Point| (-x, -y, z),
        |(x, y, z): Point| (x, -y, -z),
        |(x, y, z): Point| (-x, y, -z),
        |(x, y, z): Point| (-x, -y, -z),
        |(x, y, z): Point| (-x, y, z),
        |(x, y, z): Point| (x, -y, z),
        |(x, y, z): Point| (x, y, -z),
    ];

    let mut rotations = Vec::new();
    for axis_rotation in rotations_axis {
        for rotate_rotation in rotations_rotate {
            let rotated = scanner
                .iter()
                .copied()
                .map(|point| axis_rotation(rotate_rotation(point)))
                .collect::<Scanner>();
            rotations.push(rotated);
        }
    }
    return rotations;
}
