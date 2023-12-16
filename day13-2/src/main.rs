use std::fs;

fn horizontal_mirror(region: &Vec<Vec<char>>) -> u64 {

    for starting_point in 0..(region[0].len() - 1) {
        let mut smudges: u32 = 0;
        for size in 1..region[0].len() {        // Could be smaller, but will always exit before it finishes so whatever
            // Check each row for a match
            for row in region {
                if row[starting_point + 1 - size] != row[starting_point + size] {
                    smudges += 1;
                }
            }
            // If not a match, break out of this loop and move on to the next starting point
            if smudges > 1 {
                break;
            }

            // If match, check if this row hits the edges of the map
            if (starting_point + 1 - size) == 0 || (starting_point + size) == (region[0].len() - 1) {
                // If there was also exactly one smudge
                if smudges == 1 {
                    // If so, return this starting point 
                    return (starting_point + 1) as u64
                } else {
                    break;
                }
                
            }

            // If not, continue with the next size up
        }

    }

    return 0;
}

fn vertical_mirror(region: &Vec<Vec<char>>) -> u64 {
    for starting_point in 0..(region.len() - 1) {
        let mut smudges: u32 = 0;
        for size in 1..region.len() {        // Could be smaller, but will always exit before it finishes so whatever
            // Check each column for a match
            for col in 0..region[0].len() {
                if region[starting_point + 1 - size][col] != region[starting_point + size][col] {
                    smudges += 1;
                }
            }
            // If not a match, break out of this loop and move on to the next starting point
            if smudges > 1 {
                break;
            }

            // If match, check if this row hits the edges of the map
            if (starting_point + 1 - size) == 0 || (starting_point + size) == (region.len() - 1) {
                // If there was also exactly one smudge
                if smudges == 1 {
                    // If so, return this starting point 
                    return (starting_point + 1) as u64
                } else {
                    break;
                }
            }

            // If not, continue with the next size up
        }

    }

    return 0;
}

fn main() {
    let mut input_string: String = fs::read_to_string("../../input.txt").expect("Couldn't find file");
    input_string.pop();

    let regions: Vec<&str> = input_string.split("\n\n").collect();

    let mut total: u64 = 0;

    for region in regions {
        // Build vec array
        let map: Vec<Vec<char>> = region.split("\n").collect::<Vec<&str>>().iter().map(|x| x.chars().collect::<Vec<char>>()).collect();

        // Find horizontal mirror
        let horizontal: u64 = horizontal_mirror(&map);
        // Find vertical mirror
        let vertical:   u64 = vertical_mirror(  &map);

        // Add to total
        total += 100 * vertical + horizontal;
    }

    println!("Total: {}", total);
}
