use std::fs;
use std::collections::HashMap;

fn main() {
    let mut raw_text: String = fs::read_to_string("../../input.txt").expect("Could not read file!");
    raw_text.pop(); // Pop trailing newline so when we split by line things are simpler
    let lines: Vec<String> = raw_text.split("\n").map(str::to_string).collect();

    let mut card_map: HashMap<usize, Vec<usize>> = HashMap::new();

    for (idx, l) in lines.iter().enumerate() {
        let mut num_matches: usize = 0;

        let split_name: Vec<String> = l.split(": ").map(str::to_string).collect();

        let numbers_split: Vec<String> = split_name[1].split(" | ").map(str::to_string).collect();

        let lucky_numbers: Vec<i32> = numbers_split[0].split(' ').filter(|&x| x.len() > 0).map(|x| -> i32{x.parse().unwrap()}).collect();

        let target_numbers: Vec<i32> = numbers_split[1].split(' ').filter(|&x| x.len() > 0).map(|x| -> i32{x.parse().unwrap()}).collect();

        for lucky in lucky_numbers {
            if target_numbers.contains(&lucky) {
                num_matches += 1;
            }
        }

        if num_matches == 0 {
            card_map.insert(idx + 1, vec![]);
        } else {
            card_map.insert(idx + 1, (1..=num_matches).collect());
        }
    }

    let mut total_scratchcards: Vec<usize> = (1..=lines.len()).collect::<Vec<usize>>();

    let mut idx: usize = 0;
    while idx < total_scratchcards.len() {
        total_scratchcards.append(&mut card_map.entry(total_scratchcards[idx]).or_insert(vec![]).iter().map(|x| total_scratchcards[idx] + x).collect());
        idx += 1;
    }

    println!("Total Scratchcards: {}", total_scratchcards.len());


   

}
