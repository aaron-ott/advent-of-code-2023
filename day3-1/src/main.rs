use std::fs;

fn get_full_number(character_map: &Vec<Vec<char>>, row: usize, col: usize) -> (u32, usize, usize) {
    let start_col = col;
    let mut end_col = col;
    let mut value = character_map[row][end_col].to_digit(10).unwrap();

    while end_col < character_map[0].len() - 1 && character_map[row][end_col + 1].is_numeric() {
        value = value * 10 + character_map[row][end_col + 1].to_digit(10).unwrap();
        end_col += 1;
    }

    return (value, start_col, end_col);
}

fn check_number(character_map: &Vec<Vec<char>>, row: usize, col: usize) -> (bool, usize, u32) {

    let (value, start_col, end_col) = get_full_number(&character_map, row, col);

    let length: usize = end_col - start_col + 1;

    let i_start: usize = if row > 0 { row - 1 } else { 0 };
    let i_end: usize = if row < character_map.len() - 1 { row + 1 } else { character_map.len() - 1 };

    let j_start: usize = if start_col > 0 { start_col - 1 } else { 0 };
    let j_end: usize = if end_col < character_map[0].len() - 1 { end_col + 1 } else { character_map[0].len() - 1 };

    for i in i_start..=i_end {
        for j in j_start..=j_end {
            if !character_map[i][j].is_alphanumeric() && character_map[i][j] != '.' {
                return (true, length, value);
            }
        }
    }

    return (false, length, 0);
}

fn main() {
    let mut file_contents: String = fs::read_to_string("../../input.txt").expect("Couldn't find file!");
    file_contents.pop(); // Remove trailing newline
    let lines: Vec<String> = file_contents.split("\n").map(str::to_string).collect::<Vec<String>>();

    let character_map : Vec<Vec<char>> = lines.iter().map(|s| s.chars().collect()).collect::<Vec<Vec<char>>>();

    let mut total = 0;

    for row in 0..character_map.len() {
        let mut col: usize = 0;
        

        while col < character_map[0].len() {
            if character_map[row][col].is_numeric() {
                let (is_target, length, value) = check_number(&character_map, row, col);
                if is_target {
                    total += value;
                    col += length;
                    continue;
                } else {
                    col += length;
                }
            }
            col += 1;
        }
    }

    println!("Total: {}", total);
    
}
