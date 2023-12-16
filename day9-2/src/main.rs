use std::fs;

fn main() {
    let mut input_text = fs::read_to_string("../../input.txt").expect("Couldn't find file");
    input_text.pop();
    let lines: Vec<&str> = input_text.split("\n").collect();

    let mut total: i64 = 0;

    for l in lines {
        let mut sequences: Vec<Vec<i64>> = vec![];
        sequences.push(l.split_whitespace().into_iter().map(|x| x.parse::<i64>().unwrap()).collect());
        while sequences[sequences.len() - 1].iter().any(|&x| x != 0) {
            sequences.push((1..sequences[sequences.len() - 1].len()).into_iter().map(|x| sequences[sequences.len() - 1][x] - sequences[sequences.len() - 1][x - 1]).collect())
        }
        let mut extrapolation: i64 = 0;
        for idx in (0..sequences.len() - 1).rev() {
            extrapolation = sequences[idx][0] - extrapolation;
        }
        total += extrapolation;
        println!("Line Extrapolation: {}", extrapolation);
    }

    println!("Total: {}", total);
}
