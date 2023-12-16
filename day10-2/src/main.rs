use std::fs;

fn safe_add(a: usize, b: i32) -> usize {
    return ((a as i32) + b) as usize
}

fn possible_directions(map: &Vec<Vec<char>>, position: &(usize, usize)) -> Option<Vec<(usize, usize)>> {
    return match map[position.0][position.1] {
        'S' => Some(vec![
            (safe_add(position.0, -1), safe_add(position.1,  0)),
            (safe_add(position.0,  0), safe_add(position.1,  1)),
            (safe_add(position.0,  1), safe_add(position.1,  0)),
            (safe_add(position.0,  0), safe_add(position.1, -1))]),
        '-' => Some(vec![
            (safe_add(position.0,  0), safe_add(position.1, -1)),
            (safe_add(position.0,  0), safe_add(position.1,  1))]),
        '|' =>Some(vec![
            (safe_add(position.0, -1), safe_add(position.1,  0)),
            (safe_add(position.0,  1), safe_add(position.1,  0))]),
        'L' =>Some(vec![
            (safe_add(position.0, -1), safe_add(position.1,  0)),
            (safe_add(position.0,  0), safe_add(position.1,  1))]),
        'F' =>Some(vec![
            (safe_add(position.0,  0), safe_add(position.1,  1)),
            (safe_add(position.0,  1), safe_add(position.1,  0))]),
        '7' =>Some(vec![
            (safe_add(position.0,  1), safe_add(position.1,  0)),
            (safe_add(position.0,  0), safe_add(position.1, -1))]),
        'J' =>Some(vec![
            (safe_add(position.0,  0), safe_add(position.1, -1)),
            (safe_add(position.0, -1), safe_add(position.1,  0))]),
        '.' => None,
        _ => None
    }
}

fn main() {
    let mut input_text: String = fs::read_to_string("../../input.txt").expect("Couldn't find file");
    input_text.pop();

    // Split text into grid
    let map: Vec<Vec<char>> = input_text.split("\n").map(|line| line.chars().collect::<Vec<char>>()).collect();

    let initial_row: usize = map.iter().position(|v| v.iter().any(|&c| c == 'S')).unwrap();
    let initial_col: usize = map[initial_row].iter().position(|&c| c == 'S').unwrap();

    let mut route: Vec<(usize, usize)> = vec![(initial_row, initial_col)];

    route.push(*possible_directions(&map, route.iter().last().unwrap())
                .unwrap()
                .iter()
                .find(|&pos| possible_directions(&map, pos)
                                                .unwrap()
                                                .iter()
                                                .any(|pos2| pos2 == route.iter().last().unwrap())
                    ).unwrap()
                );

    while route.iter().last() != route.first() {
        route.push(*possible_directions(&map, route.iter().last().unwrap())
                    .unwrap()
                    .iter()
                    .find(|&&pos| pos != route[route.len() - 2])
                    .unwrap());
    }

    let mut inside_counter = 0;

    for cell_x in 0..map.len() {
        let mut crossings_counter: f32 = 0.0;
        for cell_y in 0..map[0].len() {
            if route.iter().any(|&p| p == (cell_x, cell_y)) {
                // Update crossings_counter
                crossings_counter += match map[cell_x][cell_y] {
                    'S' => -0.5,
                    '-' =>  0.0,
                    '|' =>  1.0,
                    'L' =>  0.5,
                    'F' => -0.5,
                    '7' =>  0.5,
                    'J' => -0.5,
                    '.' =>  0.0,
                    _   =>  0.0
                }
            } else {
                // If odd number of crossings, add to inside counter
                if crossings_counter % 2.0 != 0.0 {
                    inside_counter += 1;
                }
            }
            
        }
    }

    println!("Inside Count: {}", inside_counter);


}
