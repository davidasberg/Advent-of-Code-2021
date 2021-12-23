
use std::collections::HashSet;
use std::fs;

fn main() {
    part_one_and_two();
}

fn part_one_and_two() {
    let instructions = get_input("input.txt");

    let mut activated: HashSet<Cuboid> = HashSet::new();

    for (cuboid, state) in instructions {
     
        let mut new_activated = HashSet::new();
        for current in activated.iter() {
            let diff: HashSet<Cuboid> = current.remove_intersection(&cuboid);
            new_activated = new_activated.union(&diff).cloned().collect();
        }
        if state {
            new_activated.insert(cuboid.clone());
        }
        activated = new_activated.clone();
        //println!("{}", count);
    }
    
    let count = activated.iter().filter(|x| x.is_small()).map(|c| c.get_volume() as i128).sum::<i128>();
    println!("Part 1: {}", count);
    let count = activated.iter().map(|c| c.get_volume() as i64).sum::<i64>();
    println!("Part 2: {}", count);

}

fn get_input(file: &str) -> Vec<Instruction> {
    let contents = fs::read_to_string(file).expect("Something went wrong reading the file");
    let mut instructions: Vec<Instruction> = Vec::new();
    for line in contents.lines() {
        let mut parts = line.split(' ');
        
        let state = match parts.next().unwrap() {
            "on" => true,
            "off" => false,
            _ => panic!("Invalid state"),
        };

        let mut values = parts.next().unwrap().split(',');

        let mut x = values.next().unwrap().split_once('=').unwrap().1.split("..");
        let x_min = x.next().unwrap().parse::<i64>().unwrap();
        let x_max = x.next().unwrap().parse::<i64>().unwrap();

        let mut y = values.next().unwrap().split_once('=').unwrap().1.split("..");
        let y_min = y.next().unwrap().parse::<i64>().unwrap();
        let y_max = y.next().unwrap().parse::<i64>().unwrap();

        let mut z = values.next().unwrap().split_once('=').unwrap().1.split("..");
        let z_min = z.next().unwrap().parse::<i64>().unwrap();
        let z_max = z.next().unwrap().parse::<i64>().unwrap();

        instructions.push((
            Cuboid::new(x_min,x_max, y_min,y_max,z_min,z_max), 
            state
        ));
    }
    instructions
}
type Instruction = (Cuboid, bool);

#[derive(Clone, PartialEq, Eq, std::hash::Hash)]
struct Cuboid {
    x_min: i64,
    x_max: i64,
    y_min: i64,
    y_max: i64,
    z_min: i64,
    z_max: i64,
}


// impl std::hash::Hash for Cuboid {
//     fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
//         self.x_min.hash(state);
//         self.x_max.hash(state);
//         self.y_min.hash(state);
//         self.y_max.hash(state);
//         self.z_min.hash(state);
//         self.z_max.hash(state);
//     }
// }

impl Cuboid {
    fn new(x_min: i64, x_max: i64, y_min: i64, y_max: i64, z_min: i64, z_max: i64) -> Cuboid {
        Cuboid {
            x_min,
            x_max,
            y_min,
            y_max,
            z_min,
            z_max,
        }
    }

    fn is_small(&self) -> bool {
        self.x_min > -50 && self.x_max < 50 &&
        self.y_min > -50 && self.y_max < 50 &&
        self.z_min > -50 && self.z_max < 50
    }

    fn get_volume(&self) -> i64 {
        (self.x_max - self.x_min + 1) * (self.y_max - self.y_min + 1) * (self.z_max - self.z_min + 1)
    }

    fn is_intersecting(&self, other: &Cuboid) -> bool {
        self.x_min <= other.x_max && 
        self.x_max >= other.x_min && 
        self.y_min <= other.y_max && 
        self.y_max >= other.y_min && 
        self.z_min <= other.z_max && 
        self.z_max >= other.z_min
    }

  
    fn get_intersection(&self, other: &Cuboid) -> Option<Cuboid> {
        if !self.is_intersecting(other) {
            return None;
        }
        Some(Cuboid::new(
            self.x_min.max(other.x_min),
            self.x_max.min(other.x_max),
            self.y_min.max(other.y_min),
            self.y_max.min(other.y_max),
            self.z_min.max(other.z_min),
            self.z_max.min(other.z_max),
        ))
    }

    fn remove_intersection(&self, other: &Cuboid) -> HashSet<Cuboid> {
        let intersection = self.get_intersection(other);
        match intersection {
            Some(intersection) => {
                if intersection == *self {
                    return HashSet::new();
                }
                else {
                    return self.remove(&intersection);
                }
            },
            None => {
                let mut set = HashSet::new();
                set.insert((*self).clone());
                return set;
            }
        }
    }
    

    // Removes an internal segment of this cuboid by
    //     returning (up to) 6 mutually exclusive
    //     segments

    fn remove(&self, other: &Cuboid) -> HashSet<Cuboid> {
        let mut new_parts = HashSet::new();
        let mut z_min = other.z_max + 1;
        if z_min <= self.z_max {
            new_parts.insert(Cuboid::new(
                self.x_min,
                self.x_max,
                self.y_min,
                self.y_max,
                z_min,
                self.z_max,
            ));
            z_min -= 1;
        }
        else {
            z_min = self.z_max;
        }

        let mut z_max = other.z_min - 1;
        if z_max >= self.z_min {
            new_parts.insert(Cuboid::new(
                self.x_min,
                self.x_max,
                self.y_min,
                self.y_max,
                self.z_min,
                z_max,
            ));
            z_max += 1;
        }
        else {
            z_max = self.z_min;
        }

        let mut x_min = other.x_max + 1;
        if x_min <= self.x_max {
            new_parts.insert(Cuboid::new(
                x_min,
                self.x_max,
                self.y_min,
                self.y_max,
                z_max,
                z_min,
            ));
            x_min -= 1;
        }
        else {
            x_min = self.x_max;
        }

        let mut x_max = other.x_min - 1;
        if x_max >= self.x_min {
            new_parts.insert(Cuboid::new(
                self.x_min,
                x_max,
                self.y_min,
                self.y_max,
                z_max,
                z_min,
            ));
            x_max += 1;
        }
        else {
            x_max = self.x_min;
        }

        let y_min = other.y_max + 1;
        if y_min <= self.y_max {
            new_parts.insert(Cuboid::new(
                x_max,
                x_min,
                y_min,
                self.y_max,
                z_max,
                z_min,
            ));
        }

        let y_max = other.y_min - 1;
        if y_max >= self.y_min {
            new_parts.insert(Cuboid::new(
                x_max,
                x_min,
                self.y_min,
                y_max,
                z_max,
                z_min,
            ));
        }
    
        new_parts
    }
}




//tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cuboid_count() {
        assert_eq!(Cuboid::new(10, 12, 10, 12, 10, 12).get_volume(), 27);
    }

    #[test]
    fn test_cuboid_intersects(){
        assert_eq!(Cuboid::new(10, 12, 10, 12, 10, 12).is_intersecting(&Cuboid::new(10, 12, 9, 12, 10, 12)), true);
        assert_eq!(Cuboid::new(10, 12, 10, 12, 10, 12).is_intersecting(&Cuboid::new(20, 22, 20, 22, 20, 22)), false);
    }

    #[test]
    fn test_cuboid_get_intersection() {
        assert_eq!(
            Cuboid::new(10, 12, 10, 12, 10, 12)
                .get_intersection(&Cuboid::new(12, 22, 12, 22, 12, 22)).unwrap().get_volume(),
            1
        );
    }
}