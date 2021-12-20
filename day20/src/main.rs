
use std::fs;
use std::collections::HashMap;


type ImageAlg = Vec<bool>;
type Image = HashMap<(i32,i32),bool>;

fn main() {
    part_one();
}

fn get_input(file: &str) -> (ImageAlg, Image) {
    // Read the file as a String
    let contents = fs::read_to_string(file).expect("Something went wrong reading the file");
    //split into two parts at \n\n
    let mut parts = contents.split("\n\n");
    //iterate over part 1 and if value is # then add that index to hashset
    let image_alg = parts.next().unwrap().chars().enumerate().map(|(_, c)| (c == '#')).collect::<ImageAlg>();
    let mut image: Image = HashMap::new();
    for (y, line) in parts.next().unwrap().lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                image.insert((x as i32, y as i32), true);
            }
            else {
                image.insert((x as i32, y as i32), false);
            }
        }
    }
    (image_alg, image)
}

fn part_one() {
    let (image_alg, mut image) = get_input("input.txt");
    image = enhance_picture(&image_alg, &image, 2);
    println!("{}", count_pixels(&image));
}

fn count_pixels(image: &Image) -> i32 {
    //count keys in hashmap that are true
    image.iter().filter(|(_, &v)| v).count() as i32
}

fn enhance_picture(image_alg: &ImageAlg, image: &Image, steps: i32) -> Image {
    //create copy of image with 1 extra element in each direction
    let mut image = image.clone();
    print_image(&image);
    //get min and max x and y values
    
    for i in 0..steps {
        let mut improved_image: Image = HashMap::new();
        let (mut min, mut max) = (std::i32::MAX,std::i32::MIN);
        for (x, y) in image.keys() {
            min = min.min(*x.min(y));
            max = max.max(*x.max(y));
        }
        for y in min-2..=max+2{
            for x in min-2..=max+2 {
                
                let neighbors = get_neighbors((x, y));
                let pixel_index = binary_from_neighbors(neighbors, &image, image_alg[0] && i%2 != 0, &min, &max);
                let mut pixel_value = image_alg[pixel_index as usize];

                improved_image.insert((x, y), pixel_value);
            }
        }

        // let edges = vec![min-3, min-2, max+2, max+3];
        // if i%2 == 0 {
        //     for y in min-3..max+3{
        //         for x in 0..edges.len() {
        //             improved_image.insert((edges[x], y), true);
        //             improved_image.insert((y, edges[x]), true);
        //         }
        //     }
        // }
        
       
    
        image = improved_image;
        print_image(&image);
    }
    image
}

fn binary_from_neighbors(neighbors: Vec<(i32,i32)>, image: &Image, outside: bool, min: &i32, max: &i32 ) -> u16 {
    let mut binary = 0;
    for (i,(x,y)) in neighbors.iter().rev().enumerate() {
        if outside && (x < min || x > max || y < min || y > max) {
            binary |= 1 << i
        }  
        if image.get(&(*x,*y)) == Some(&true) {
            binary |= 1 << i
        }
    }
    binary
}

//get all neighbors of a point, including itself, order is each row is filled first then next row etc
fn get_neighbors(p: (i32,i32)) -> Vec<(i32,i32)> {
    //get all 9 neighbours of p (including itself)
    let mut neighbours = vec![];
    for y in p.1-1..=p.1+1 {
        for x in p.0-1..=p.0+1 {
            neighbours.push((x,y));
        }
    }
    neighbours
}

fn print_image(image: &Image) {
    let (mut x_min, mut x_max, mut y_min, mut y_max) = (std::i32::MAX,std::i32::MIN,std::i32::MAX,std::i32::MIN);
    for (x, y) in image.keys() {
        x_min = x_min.min(*x);
        x_max = x_max.max(*x);
        y_min = y_min.min(*y);
        y_max = y_max.max(*y);
    }
    for y in y_min-1..=y_max+1 {
        for x in x_min-1..=x_max+1 {
            print!("{}", if image.get(&(x,y)) == Some(&true) { "#" } else { "." });
        }
        println!("");
    }
}



#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn get_neighbors_test() {
        assert_eq!(vec![(0,0),(1,0),(2,0),(0,1),(1,1),(2,1),(0,2),(1,2),(2,2)], get_neighbors((1,1)));
    }

    #[test]
    fn binary_from_neighbors_test() {
        //create set with (0..2,0..2) as false
        let mut image = HashMap::new();
        for y in 0..2 {
            for x in 0..2 {
                image.insert((x,y), false);
            }
        }
        //set to true
        image.insert((0,0), true);
        assert_eq!(0b100000000, binary_from_neighbors(vec![(0,0),(1,0),(2,0),(0,1),(1,1),(2,1),(0,2),(1,2),(2,2)], &image));
    }

    #[test]
    fn example_test() {
        let (image_alg, mut image) = get_input("example.txt");
        image = enhance_picture(&image_alg, &image, 2);
        assert_eq!(35, count_pixels(&image));
    }
}

