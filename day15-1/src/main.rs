use std::fs;
use rayon::prelude::*;

fn hash_algorithm(s: &str) -> u64 {
    let mut accumulator: u64 = 0;
    for c in s.chars() {
        accumulator += c as u64;
        accumulator = accumulator * 17;
        accumulator = accumulator % 256;
    }
    return accumulator;
}

fn main() {
    let mut input_text: String = fs::read_to_string("../../input_example.txt").expect("Couldn't find file");
    input_text.pop();
    let steps: Vec<&str> = input_text.split(",").collect();

    let total: u64 = steps.par_iter().map(|x| hash_algorithm(x)).sum();

    println!("Total: {}", total);

}
