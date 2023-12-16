use std::fs;

fn absolute_distance(a: usize, b: usize) -> usize {
    let value: i32 = (a as i32) - (b as i32);
    return value.abs() as usize
}

fn main() {
    let mut input_string: String = fs::read_to_string("../../input.txt").expect("Couldn't find file");
    input_string.pop();

    let mut map: Vec<Vec<char>> = vec![];

    // Create map and also expand in y direction
    for line in input_string.split("\n") {
        map.push(line.chars().collect());
        if line.chars().collect::<Vec<char>>().iter().all(|&c| c == '.') {
            map.push(line.chars().collect());
        }
    }

    // Expand in x direction
    for col in (0..(map[0].len() - 1)).rev() {
        if map.iter().all(|x| x[col] == '.') {
            // println!("Insert new col at index {}", col);
            map.iter_mut().for_each(|x| x.insert(col, '.'));
        }
    }

    println!("Final Map Size: {} x {}", map.len(), map[0].len());

    // Find all the galaxies
    let mut galaxies: Vec<(usize, usize)> = vec![];
    for row in 0..map.len() {
        for col in 0..map[0].len() {
            if map[row][col] == '#' {
                galaxies.push((row, col));
            }
        }
    }

    // println!("Galaxy Locations:");
    // for g in &galaxies {
    //     println!("    ({}, {})", g.0, g.1);
    // }

    let mut sum = 0;
    for idx in 0..galaxies.len() {
        for other_idx in (idx + 1)..galaxies.len() {
            sum += absolute_distance(galaxies[idx].0, galaxies[other_idx].0) + absolute_distance(galaxies[idx].1, galaxies[other_idx].1);
        }
    }

    println!("Total: {}", sum);

}
