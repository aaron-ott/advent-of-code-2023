use std::fs;


fn tilt_north(map: &mut Vec<Vec<char>>) {
    for row in 0..map.len() {
        for col in 0..map[0].len() {
            if map[row][col] == 'O' {
                let mut final_row = row;
                map[final_row][col] = '.';
                while final_row > 0 && map[final_row - 1][col] == '.' {
                    final_row -= 1;
                }
                map[final_row][col] = 'O';
            }
        }
    }
}

fn tilt_east(map: &mut Vec<Vec<char>>) {
    for col in (0..map[0].len()).rev() {
        for row in 0..map.len() {
            if map[row][col] == 'O' {
                let mut final_col = col;
                map[row][final_col] = '.';
                while final_col < map[0].len() - 1 && map[row][final_col + 1] == '.' {
                    final_col += 1;
                }
                map[row][final_col] = 'O';
            }
        }
    }
}

fn tilt_south(map: &mut Vec<Vec<char>>) {
    for row in (0..map.len()).rev() {
        for col in 0..map[0].len() {
            if map[row][col] == 'O' {
                let mut final_row = row;
                map[final_row][col] = '.';
                while final_row < map.len() - 1 && map[final_row + 1][col] == '.' {
                    final_row += 1;
                }
                map[final_row][col] = 'O';
            }
        }
    }
}

fn tilt_west(map: &mut Vec<Vec<char>>) {
    for col in 0..map[0].len() {
        for row in 0..map.len() {
            if map[row][col] == 'O' {
                let mut final_col = col;
                map[row][final_col] = '.';
                while final_col > 0 && map[row][final_col - 1] == '.' {
                    final_col -= 1;
                }
                map[row][final_col] = 'O';
            }
        }
    }
}

fn spin_cycle(map: &mut Vec<Vec<char>>) {
    tilt_north(map);
    tilt_west(map);
    tilt_south(map);
    tilt_east(map);
}

fn calc_weight(map: &Vec<Vec<char>>) -> u32 {
    let mut weight: u32 = 0;
    for row in 0..map.len() {
        for col in 0..map[0].len() {
            if map[row][col] == 'O' {
                weight += (map.len() - row) as u32;
            }
        }
    }
    return weight
}

fn map_to_str(map: &Vec<Vec<char>>) -> String {
    return map.iter().flatten().collect::<String>()
}

fn main() {
    let mut input_text: String = fs::read_to_string("../../input.txt").expect("Couldn't find file");
    input_text.pop();
    let mut map: Vec<Vec<char>> = input_text.split("\n").map(|x| x.chars().collect()).collect();

    let mut map_states: Vec<String> = vec![];

    let mut start_of_cycle: usize = 1;
    let mut cycle_size: usize = 1;

    for i in 0..1000 {
        spin_cycle(&mut map);
        if map_states.contains(&map_to_str(&map)) {
            start_of_cycle = map_states.iter().position(|x| x == &map_to_str(&map)).unwrap();
            cycle_size = i - start_of_cycle;
            break;
        } else {
            map_states.push(map_to_str(&map));
        }
    }

    let cycle_phase = (1000000000 - start_of_cycle - 1) % cycle_size;

    // for i in 0..=cycle_size {
    //     println!("Phase {}, Weight: {}", i, calc_weight(&map));
    //     spin_cycle(&mut map);
    // }

    for _ in 0..cycle_phase {
        spin_cycle(&mut map);
    }

    println!("Weight: {}", calc_weight(&map));
}
