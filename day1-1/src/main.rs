use std::fs;

fn get_digits(line: &str) -> i32 {

    let mut first_digit = 0;
    let mut second_digit = 0;

    for index in 0..line.len() {
        if (line.as_bytes()[index] as char).is_numeric() {
            first_digit = (line.as_bytes()[index] as char).to_string().parse::<i32>().unwrap();
            break;
        }
    }

    for index in (0..line.len()).rev() {
        if (line.as_bytes()[index] as char).is_numeric() {
            second_digit = (line.as_bytes()[index] as char).to_string().parse::<i32>().unwrap();
            break;
        }
    }

    return first_digit * 10 + second_digit;
}

fn main() {
    let file_contents = fs::read_to_string("../../input.txt").expect("Couldn't read file");
    let lines = file_contents.split("\n");

    let mut total = 0;

    for l in lines {
        total += get_digits(l);
    }

    println!("{}", total);
}
