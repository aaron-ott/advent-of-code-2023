use std::fs;

fn absolute_distance(a: usize, b: usize) -> usize {
    let value: i32 = (a as i32) - (b as i32);
    return value.abs() as usize
}

fn between(a: usize, b: usize, x: usize) -> bool {
    if a < b {
        return a < x && x < b
    } else {
        return b < x && x < a
    }
}

fn main() {
    let mut input_string: String = fs::read_to_string("../../input.txt").expect("Couldn't find file");
    input_string.pop();

    let mut map: Vec<Vec<char>> = vec![];

    let mut expanded_rows: Vec<usize> = vec![];

    // Create map and also expand in y direction
    for line in input_string.split("\n") {
        map.push(line.chars().collect());
        if line.chars().collect::<Vec<char>>().iter().all(|&c| c == '.') {
            expanded_rows.push(map.len() - 1);
        }
    }

    // Expand in x direction
    let mut expanded_cols: Vec<usize> = vec![];
    for col in (0..(map[0].len() - 1)).rev() {
        if map.iter().all(|x| x[col] == '.') {
            expanded_cols.push(col);
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
            sum += expanded_rows.iter().filter(|&&x| between(galaxies[idx].0, galaxies[other_idx].0, x)).count() * 999999;
            sum += expanded_cols.iter().filter(|&&x| between(galaxies[idx].1, galaxies[other_idx].1, x)).count() * 999999;
        }
    }

    println!("Total: {}", sum);

}
