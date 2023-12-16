use std::fs;
use regex::Regex;


fn main() {
    let target_red = 12;
    let target_green = 13;
    let target_blue = 14;

    let red_re = Regex::new(r"([0-9]+) red").unwrap();
    let green_re = Regex::new(r"([0-9]+) green").unwrap();
    let blue_re = Regex::new(r"([0-9]+) blue").unwrap();

    let file_contents = fs::read_to_string("../../input.txt").expect("Couldn't read file");
    let lines: Vec<String> = file_contents.split("\n").map(str::to_string).collect();

    let mut total: i32 = 0;

    for l in lines {
        // println!("{}", l);

        let game_id_re = Regex::new(r"^Game ([0-9]*):").unwrap();

        let Some(id_str) = game_id_re.captures(&l) else { println!("Oh no!"); continue };
        let id_int: i32 = id_str[1].parse::<i32>().unwrap();
        // println!("{}", &id_str[1]);

        let string_parts: Vec<String> = l.split(": ").map(str::to_string).collect();

        let rolls: Vec<String> = string_parts[1].split(";").map(str::to_string).collect();

        let mut is_valid: bool = true;

        for roll in rolls {
            let red_int: i32 = match red_re.captures(&roll) {
                None => 0,
                Some(op) => op[1].parse::<i32>().unwrap()
            };

            let green_int: i32 = match green_re.captures(&roll) {
                None => 0,
                Some(op) => op[1].parse::<i32>().unwrap()
            };

            let blue_int: i32 = match blue_re.captures(&roll) {
                None => 0,
                Some(op) => op[1].parse::<i32>().unwrap()
            };

            if red_int > target_red || green_int > target_green || blue_int > target_blue {
                is_valid = false;
                break;
            }
        }

        if is_valid {
            total += id_int;
        }
    }

    println!("{}", total);
}
