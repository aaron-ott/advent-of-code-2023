use std::fs;
use regex::Regex;

fn hash_algorithm(s: &str) -> usize {
    let mut accumulator: usize = 0;
    for c in s.chars() {
        accumulator += c as usize;
        accumulator = accumulator * 17;
        accumulator = accumulator % 256;
    }
    return accumulator;
}

fn main() {
    let mut input_text: String = fs::read_to_string("../../input.txt").expect("Couldn't find file");
    input_text.pop();
    let steps: Vec<&str> = input_text.split(",").collect();
    let re = Regex::new("([a-z]*)(-|=[0-9])").unwrap();

    let mut boxes: Vec<Vec<(&str, u8)>> = vec![vec![]; 256];

    for (_, [text, modifier]) in steps.iter().map(|&x| re.captures(x).unwrap().extract()) {

        // println!("Text: {}, Modifier: {}", text, modifier);
        let index: usize = hash_algorithm(text);
        match modifier {
            "-" => {
                match boxes[index].iter().position(|&x| x.0 == text) {
                    Some(i) => {
                        boxes[index].remove(i);
                    },
                    None => {}
                }
            },
            _ => {
                let digit: u8 = modifier.chars().nth(1).unwrap().to_digit(10).unwrap() as u8;
                match boxes[index].iter().position(|&x| x.0 == text) {
                    Some(i) => {
                        boxes[index][i].1 = digit;
                    },
                    None => {
                        boxes[index].push((text, digit));
                    }
                }
            }
        }
    }

    let mut total: u64 = 0;

    for b in 0..boxes.len() {
        for slot in 0..boxes[b].len() {
            total += ((b + 1) as u64) * ((slot + 1) as u64) * (boxes[b][slot].1 as u64);
            // println!("Box: {}, Slot: {}, Focal Length: {}", ((b + 1) as u64), ((slot + 1) as u64), (boxes[b][slot].1 as u64))
        }
    }

    println!("Total: {}", total);

}
