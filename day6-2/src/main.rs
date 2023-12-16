use std::fs;

fn main() {
    let input: String = fs::read_to_string("../../input.txt").expect("Couldn't read file!");
    let lines: Vec<&str> = input.split('\n').collect();
    let time:  i64 = lines[0].split(':').nth(1).unwrap().split_whitespace().map(str::to_string).collect::<Vec<String>>().join("").parse::<i64>().unwrap();
    let distance: i64 = lines[1].split(':').nth(1).unwrap().split_whitespace().map(str::to_string).collect::<Vec<String>>().join("").parse::<i64>().unwrap();

    let score: usize = (0..=time).collect::<Vec<i64>>().iter().zip((0..=time).collect::<Vec<i64>>().iter().rev()).filter(|&a| a.0 * a.1 > distance).collect::<Vec<(&i64, &i64)>>().len();

    println!("Score: {}", score);
}
