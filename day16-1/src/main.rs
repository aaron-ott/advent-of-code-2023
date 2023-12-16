use std::fs;
use std::collections::{LinkedList, HashSet};

enum Direction {
    North,
    East,
    South,
    West
}

fn beam_to_string(pos_x: usize, pos_y: usize, direction: &Direction) -> String {
    let d = match direction {
        Direction::North => "n",
        Direction::East => "e",
        Direction::South => "s",
        Direction::West => "w"
    };

    return String::from(format!("{},{}:{}", pos_x, pos_y, d));
}

fn main() {
    let mut input_string: String = fs::read_to_string("../../input.txt").expect("Couldn't find file");
    input_string.pop();

    let map: Vec<Vec<char>> = input_string.split("\n").map(|x| x.chars().collect()).collect();
    let mut energized: Vec<Vec<bool>> = vec![vec![false; map[0].len()]; map.len()];

    let mut beams: LinkedList<(usize, usize, Direction)> = LinkedList::from([(0, 0, Direction::East)]);

    let mut previous_paths: HashSet<String> = HashSet::new();

    while beams.len() > 0{
        let (pos_x, pos_y, direction) = beams.pop_front().unwrap();
        energized[pos_x][pos_y] = true;

        if previous_paths.contains(&beam_to_string(pos_x, pos_y, &direction)) {
            continue;
        } else {
            previous_paths.insert(beam_to_string(pos_x, pos_y, &direction));
        }

        match map[pos_x][pos_y] {
            '.' => {
                // Step forward once
                match &direction {
                    Direction::North => {
                        if pos_x > 0 {
                            beams.push_back((pos_x - 1, pos_y, direction));
                        } else {
                            continue;
                        }
                    },
                    Direction::East => {
                        if pos_y < map[0].len() - 1 {
                            beams.push_back((pos_x, pos_y + 1, direction));
                        } else {
                            continue;
                        }
                    },
                    Direction::South => {
                        if pos_x < map.len() - 1 {
                            beams.push_back((pos_x + 1, pos_y, direction));
                        } else {
                            continue;
                        }
                    },
                    Direction::West => {
                        if pos_y > 0 {
                            beams.push_back((pos_x, pos_y - 1, direction));
                        } else {
                            continue;
                        }
                    }
                }
            },
            '-' => {
                match &direction {
                    Direction::North | Direction::South => {
                        if pos_y > 0 {
                            beams.push_back((pos_x, pos_y - 1, Direction::West));
                        }
                        if pos_y < map[0].len() - 1 {
                            beams.push_back((pos_x, pos_y + 1, Direction::East));
                        }
                    },
                    Direction::East => {
                        if pos_y < map[0].len() - 1 {
                            beams.push_back((pos_x, pos_y + 1, Direction::East));
                        }
                    },
                    Direction::West => {
                        if pos_y > 0 {
                            beams.push_back((pos_x, pos_y - 1, Direction::West));
                        }
                    }
                }
            },                      
            '|' => {
                match &direction {
                    Direction::East | Direction::West => {
                        if pos_x > 0 {
                            beams.push_back((pos_x - 1, pos_y, Direction::North));
                        }
                        if pos_x < map.len() - 1 {
                            beams.push_back((pos_x + 1, pos_y, Direction::South));

                        }
                    },
                    Direction::North => {
                        if pos_x > 0 {
                            beams.push_back((pos_x - 1, pos_y, Direction::North));
                        }
                    },
                    Direction::South => {
                        if pos_x < map.len() - 1 {
                            beams.push_back((pos_x + 1, pos_y, Direction::South));
                        }
                    }
                }
            },
            '/' => {
                match &direction {
                    Direction::North => {
                        if pos_y < map[0].len() - 1 {
                            beams.push_back((pos_x, pos_y + 1, Direction::East));
                        }
                    },
                    Direction::East => {
                        if pos_x > 0 {
                            beams.push_back((pos_x - 1, pos_y, Direction::North));
                        }
                    },
                    Direction::South => {
                        if pos_y > 0 {
                            beams.push_back((pos_x, pos_y - 1, Direction::West));
                        }
                    },
                    Direction::West => {
                        if pos_x < map.len() - 1 {
                            beams.push_back((pos_x + 1, pos_y, Direction::South));
                        }
                    }
                }
            },
            '\\' => {
                match &direction {
                    Direction::North => {
                        if pos_y > 0 {
                            beams.push_back((pos_x, pos_y - 1, Direction::West));
                        }
                    },
                    Direction::East => {
                        if pos_x < map.len() - 1 {
                            beams.push_back((pos_x + 1, pos_y, Direction::South));
                        }
                    },
                    Direction::South => {
                        if pos_y < map[0].len() - 1 {
                            beams.push_back((pos_x, pos_y + 1, Direction::East));

                        }
                    },
                    Direction::West => {
                        if pos_x > 0 {
                            beams.push_back((pos_x - 1, pos_y, Direction::North));

                        }
                    }
                }
            },
            _ => {}
        }
    }

    println!("Total: {}", energized.iter().map(|x| x.iter().filter(|&&y| y).count() as i32).sum::<i32>())

}
