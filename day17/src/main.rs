
use std::fs;

fn main() {
    part_one_and_two();
}


fn part_one_and_two() {
    let (x_min, x_max, y_min, y_max) = get_input("input.txt");

    // Find the initial velocity that causes the probe to reach the highest y position and still eventually be within the target area after any step. What is the highest y position it reaches on this trajectory?
    
    let mut max_y = 0;
    let mut hits = 0;
    for vel_y in y_min..-y_min {
        for vel_x in 1..x_max+1 {
            let mut x = 0;
            let mut y = 0;
            let mut local_x_vel = vel_x;
            let mut local_y_vel = vel_y;
            while x < x_max && y > y_min {
                x += local_x_vel;
                y += local_y_vel;
                if y > max_y {
                    max_y = y;
                }
                if x >= x_min && x <= x_max && y >= y_min && y <= y_max {
                    hits += 1;
                    break;
                }
                local_x_vel += if local_x_vel > 0 { -1 } else if local_x_vel < 0 { 1 } else { 0 };
                local_y_vel += - 1;
            }

        }
    }
    println!("{}", max_y);
    println!("{}", hits);    
}

fn get_input(file: &str) -> (i32, i32, i32, i32) {
    let input = fs::read_to_string(file).expect("Failed to read input");
    //get x range and y range from "target area: x=20..30, y=-10..-5")
    let x_range = input.trim().split("x=").nth(1).unwrap().split("..").collect::<Vec<&str>>();
    let y_range = input.trim().split("y=").nth(1).unwrap().split("..").collect::<Vec<&str>>();
    let x_min = x_range[0].parse::<i32>().unwrap();
    let x_max = x_range[1].split(",").nth(0).unwrap().parse::<i32>().unwrap();
    let y_min = y_range[0].parse::<i32>().unwrap();
    let y_max = y_range[1].trim().parse::<i32>().unwrap();
    (x_min, x_max, y_min, y_max)
}



