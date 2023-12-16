use std::fs;

fn main() {
    let mut raw_text: String = fs::read_to_string("../../input.txt").expect("Could not read file!");
    raw_text.pop(); // Pop trailing newline so when we split by line things are simpler
    let lines: Vec<String> = raw_text.split("\n").map(str::to_string).collect();

    let mut total: i32 = 0;

    for l in lines {
        let mut num_matches: i32 = 0;

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
            continue;
        } else {
            total += i32::pow(2, (num_matches - 1) as u32);
        }
    }

    println!("{}", total);

}
