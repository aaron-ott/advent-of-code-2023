use std::fs;

fn main() {
    let input: String = fs::read_to_string("../../input.txt").expect("Couldn't read file!");
    let lines: Vec<&str> = input.split('\n').collect();
    let times:  Vec<i32> = lines[0].split(':').nth(1).unwrap().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
    let distances: Vec<i32> = lines[1].split(':').nth(1).unwrap().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();

    println!("Times: ");
    for t in &times {
        println!("    {}", t);
    }
    println!("Distances: ");
    for s in &distances {
        println!("    {}", s);
    }

    let mut score: usize = 1;

    for idx in 0..times.len() {
        let match_score: usize = (0..=times[idx]).collect::<Vec<i32>>().iter().zip((0..=times[idx]).collect::<Vec<i32>>().iter().rev()).filter(|&a| a.0 * a.1 > distances[idx]).collect::<Vec<(&i32, &i32)>>().len();
        score = score * match_score;
    }

    println!("Score: {}", score);
}
