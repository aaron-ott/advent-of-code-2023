use std::fs;
use std::cmp::min;

fn main() {
    let mut input_text = fs::read_to_string("../../input.txt").expect("Couldn't find file!");
    input_text.pop();
    let sections: Vec<&str> = input_text.split("\n\n").collect();

    // Build massive data structures to house these rules
    let seeds: Vec<u64> = sections[0].split_whitespace().collect::<Vec<&str>>()[1..].into_iter().map(|&x| x.parse::<u64>().unwrap()).collect();

    let mut megaladon: Vec<Vec<(u64, u64, u64)>> = vec![];
    for &section in sections[1..].iter() {
        megaladon.push(section
            .split("\n")
            .collect::<Vec<&str>>()[1..]
            .iter()
            .map(|&x| {
                let parts: Vec<u64> = x.split_whitespace().map(|x| x.parse::<u64>().unwrap()).collect();
                (parts[0], parts[1], parts[2])
            })
            .collect()
        );
    }
    
    // Pass seeds through the monolith
    let mut lowest_output_location: u64 = u64::MAX;

    for &seed in seeds.iter() {
        let mut current_value = seed;
        for section in megaladon.iter() {
            println!("{}", current_value);
            for rule in section {
                if current_value >= rule.1 && current_value < (rule.1 + rule.2) {
                    current_value = rule.0 + (current_value - rule.1);
                    break;
                }
            }
        }
        lowest_output_location = min(current_value, lowest_output_location);    
    }

    println!("Lowest Output Location: {}", lowest_output_location);


}
