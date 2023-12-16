use std::fs;
use itertools::Itertools;
use std::iter;
use tqdm::tqdm;

fn stupid_sum(x: &Vec<&usize>) -> usize {
    let mut total: usize = 0;
    for &&i in x {
        total += i;
    }
    return total
}

fn count_number_arrangements(map: Vec<char>, springs: Vec<u32>) -> usize {
    let map_size = map.len();
    let min_springs: usize = (springs.iter().sum::<u32>() as usize) + springs.len() - 1;
    let extra_springs = map_size - min_springs;

    if extra_springs == 0 {
        return 1
    }

    let mut permutations_helper: Vec<usize> = iter::repeat(0).take(min_springs).collect::<Vec<usize>>();
    for i in 1..=extra_springs {
        permutations_helper.append(&mut iter::repeat(i).take(extra_springs / i).collect::<Vec<usize>>());
    }
    

    let mut num_arrangements: usize = 0;

    println!("Map: {:?}; Springs: {:?}", map, springs);       //////////////////////////////////////////////////////////

    // Iterate over all permutations
    for idxs in permutations_helper.iter()
                            .permutations(springs.len() + 1)
                            .filter(|x| stupid_sum(x) == extra_springs)
                            .unique() {

        println!("    {:?}", idxs);   ////////////////////////////////////////////////////////////////
        // Generate new potential string
        let mut potential_string: Vec<char> = vec!['.'; map_size];
        let mut p_s_idx: usize = 0;
        for i in 0..springs.len() {
            p_s_idx += idxs[i];
            for _ in 0..springs[i] {
                potential_string[p_s_idx] = '#';
                p_s_idx += 1;
            }
            if i != springs.len() - 1 {
                potential_string[p_s_idx] = '.';
                p_s_idx += 1;
            }
        }

        // Check if string works in the map
        let mut is_match: bool = true;
        for idx in 0..potential_string.len() {
            if *map.iter().nth(idx).unwrap() != potential_string[idx] && *map.iter().nth(idx).unwrap() != '?' {
                is_match = false;
                break;
            }
        }

        if is_match {
            num_arrangements += 1;
            println!("        Match: {:?}", potential_string);   /////////////////////////////////////////////////////////////
        }
    }

    return num_arrangements
}

fn main() {
    let mut input_string: String = fs::read_to_string("../../input_example.txt").expect("Couldn't find file");
    input_string.pop();

    let mut count: usize = 0;

    let lines: Vec<&str> = input_string.split("\n").collect_vec();

    for line in tqdm(lines.iter()) {
        let (map, springs) = line.split_once(" ").unwrap();
        count += count_number_arrangements(map.chars().collect::<Vec<char>>(), springs.split(",").map(|n| n.parse::<u32>().unwrap()).collect::<Vec<u32>>());
    }

    println!("Total: {}", count);
}
