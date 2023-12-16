use std::fs;
use itertools::Itertools;
use tqdm::tqdm;

fn strings_match(map: &[char], guess: &[char]) -> bool {
    for idx in 0..map.len() {
        if map[idx] != guess[idx] && map[idx] != '?' {
            return false
        }
    }
    return true
}

fn recursive_search(map: &[char], springs: &[usize], recursive_level: usize) -> usize {
    // println!("{:indent$}Map: {:?}; Springs: {:?}", "", map, springs, indent=((recursive_level + 1) * 2)); ////////////////////////////////////////////////////////
    if springs.len() == 0 {
        for &c in map {
            if c == '#' {
                return 0
            }
        }
        return 1
    }

    let string_length: usize = map.len();
    let spaces_left: usize = string_length + 1 - (springs.iter().sum::<usize>() as usize) - springs.len();

    let mut total = 0;

    for s in 0..=spaces_left {
        let element: usize = springs[0];
        let mut guess: Vec<char> = vec![];
        // Add leading spacers
        for _ in 0..s {
            guess.push('.');
        }
        // Add the current spring length
        for _ in 0..element {
            guess.push('#');
        }
        // Extra spacer if there are more springs to come
        if springs.len() > 1 {
            guess.push('.');
        }
        // println!("{:indent$}Prefix: {:?}", "", guess, indent=((recursive_level + 1) * 2)); //////////////////////////////////////////////////////////////
        // If the prefix works, check all suffixes recursively
        if strings_match(&map[0..guess.len()], guess.as_slice()) {
            total += recursive_search(&map[guess.len()..], &springs[1..], recursive_level + 1);
        }
    }
    
    // println!("{:indent$}Return: {}", "", total, indent=((recursive_level + 1) * 2));  /////////////////////////////////////////////////////////////////////
    return total
}

fn main() {
    let mut input_string: String = fs::read_to_string("../../input.txt").expect("Couldn't find file");
    input_string.pop();

    let mut count: usize = 0;

    let lines: Vec<&str> = input_string.split("\n").collect_vec();

    for line in tqdm(lines.iter()) {
        let (map, springs) = line.split_once(" ").unwrap();
        // println!("String: {}; Springs: {}", map, springs);  ////////////////////////////////////////////////////////////////////////////
        count += recursive_search(&map.chars().collect::<Vec<char>>(), &springs.split(",").map(|n| n.parse::<usize>().unwrap()).collect::<Vec<usize>>(), 0);
    }

    println!("Total: {}", count);
}
