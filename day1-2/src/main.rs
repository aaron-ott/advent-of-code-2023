use std::fs;

fn get_digits(line: &str) -> u32 {

    let spelled_numbers: Vec<String> = vec![
        // "zero".to_string(),
        "one".to_string(),
        "two".to_string(),
        "three".to_string(),
        "four".to_string(),
        "five".to_string(),
        "six".to_string(),
        "seven".to_string(),
        "eight".to_string(),
        "nine".to_string()
    ];

    let mut first_digit: u32 = 10;
    let mut last_digit: u32 = 0;

    for index in 0..line.len() {
        if (line.as_bytes()[index] as char).is_numeric() {
            if first_digit > 9 {
                first_digit = (line.as_bytes()[index] as char).to_string().parse::<u32>().unwrap();
            }
            last_digit = (line.as_bytes()[index] as char).to_string().parse::<u32>().unwrap();
        } else {
            for (i, num) in spelled_numbers.iter().enumerate() {
                if index + num.len() - 1 < line.len() && &line[index..index+num.len()] == num {
                    if first_digit > 9 {
                        first_digit = (i + 1) as u32;
                    }
                    last_digit = (i + 1) as u32;
                }
            }
        }
    }

    println!("Line: {}, Digits: {}, {}, {}", line, first_digit, last_digit, first_digit * 10 + last_digit);
    return first_digit * 10 + last_digit;
}

fn main() {
    let mut file_contents = fs::read_to_string("../../input.txt").expect("Couldn't read file");
    file_contents.pop();
    let lines = file_contents.split("\n");

    let mut total = 0;

    for l in lines {
        total += get_digits(l);
    }

    println!("{}", total);
}

