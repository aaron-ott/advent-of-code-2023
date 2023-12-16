use std::fs;
use std::cmp;
use regex::Regex;


fn main() {
    let red_re = Regex::new(r"([0-9]+) red").unwrap();
    let green_re = Regex::new(r"([0-9]+) green").unwrap();
    let blue_re = Regex::new(r"([0-9]+) blue").unwrap();

    let file_contents = fs::read_to_string("../../input.txt").expect("Couldn't read file");
    let lines: Vec<String> = file_contents.split("\n").map(str::to_string).collect();

    let mut total: i32 = 0;

    for l in lines {
        if l.len() == 0 {
            continue;
        }
        let string_parts: Vec<String> = l.split(": ").map(str::to_string).collect();

        let rolls: Vec<String> = string_parts[1].split(";").map(str::to_string).collect();

        let mut min_red: i32 = 0;
        let mut min_green: i32 = 0;
        let mut min_blue: i32 = 0;



        for roll in rolls {
            min_red = match red_re.captures(&roll) {
                None => min_red,
                Some(op) => cmp::max(min_red, op[1].parse::<i32>().unwrap())
            };

            min_green = match green_re.captures(&roll) {
                None => min_green,
                Some(op) => cmp::max(min_green, op[1].parse::<i32>().unwrap())
            };

            min_blue = match blue_re.captures(&roll) {
                None => min_blue,
                Some(op) => cmp::max(min_blue, op[1].parse::<i32>().unwrap())
            };
        }

        let power: i32 = min_red * min_green * min_blue;

        total += power;
    }

    println!("{}", total);
}
