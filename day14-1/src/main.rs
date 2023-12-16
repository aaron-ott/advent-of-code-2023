use std::fs;

fn main() {
    let mut input_text: String = fs::read_to_string("../../input.txt").expect("Couldn't find file");
    input_text.pop();
    let mut map: Vec<Vec<char>> = input_text.split("\n").map(|x| x.chars().collect()).collect();

    println!("Width: {}, Height: {}", map[0].len(), map.len());

    let mut weight: u32 = 0;

    for row in 0..map.len() {
        for col in 0..map[0].len() {
            if map[row][col] == 'O' {
                let mut final_row = row;
                map[final_row][col] = '.';
                while final_row > 0 && map[final_row - 1][col] == '.' {
                    final_row -= 1;
                }
                map[final_row][col] = 'O';
                weight += (map.len() - final_row) as u32;
            }
        }
    }

    println!("Final Weight: {}", weight);
}
