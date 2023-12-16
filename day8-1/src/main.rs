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
    let mut current_element: &str = "AAA";
    
    while current_element != "ZZZ" {
        let next_directions = map[current_element];
        current_element = match directions.chars().nth(current_direction_idx).unwrap() {
            'L' => next_directions.0,
            'R' => next_directions.1,
            _ => panic!("What?")
        };
        counter += 1;
        current_direction_idx += 1;
        if current_direction_idx == directions.len() {
            current_direction_idx = 0;
        }
    }

    println!("Total Number of Steps: {}", counter);
    
}
