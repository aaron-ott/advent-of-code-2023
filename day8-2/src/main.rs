use std::fs;
use std::collections::HashMap;
use regex::Regex;

fn main() {
    let mut input_text: String = fs::read_to_string("../../input.txt").expect("Couldn't read file");
    input_text.pop();
    let lines: Vec<&str> = input_text.split("\n").collect();

    let directions: &str = lines[0];

    let map_regex: Regex = Regex::new(r"^([A-Z]{3}) = \(([A-Z]{3}), ([A-Z]{3})\)$").unwrap();
    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();

    for (_, [entry, left, right]) in lines[2..].iter().map(|&l| map_regex.captures(l).map(|c| c.extract()).unwrap()) {
        map.insert(entry, (left, right));
    }

    let mut counter: u32 = 0;
    let mut current_direction_idx: usize = 0;


    let mut current_elements: Vec<&str> = map.keys().into_iter().filter(|&&x| x.chars().nth(2) == Some('A')).collect::<Vec<&&str>>().iter().map(|&&x| x).collect();
    let mut periods: Vec<i64> = vec![];
    let mut last_indices: Vec<u32> = vec![];
    for _ in &current_elements {
        periods.push(0);
        last_indices.push(0);
    }


    while ! current_elements.iter().all(|&x| x.chars().nth(2) == Some('Z')) {
        for idx in 0..current_elements.len() {
            let next_directions = map[current_elements[idx]];
            current_elements[idx] = match directions.chars().nth(current_direction_idx).unwrap() {
                'L' => next_directions.0,
                'R' => next_directions.1,
                _ => panic!("What?")
            };
        }
        counter += 1;
        current_direction_idx += 1;
        if current_direction_idx == directions.len() {
            current_direction_idx = 0;
        }

        for (idx, &e) in current_elements.iter().enumerate() {
            if e.chars().nth(2) == Some('Z') {
                periods[idx] = i64::from(counter) - i64::from(last_indices[idx]);
                last_indices[idx] = counter;
            }
        }

        if periods.iter().all(|&x| x != 0) {
            break;
        }
    }

    for &p in periods.iter() {
        println!("{}", p);
    }
    
}
