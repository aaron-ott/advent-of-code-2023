use std::collections::HashMap;
use std::fs;

fn hand_to_score(hand: &str) -> u64 {
    let card_hash_map: HashMap<char, u8> = HashMap::from([
        ('A', 14),
        ('K', 13),
        ('Q', 12),
        ('J', 11),
        ('T', 10),
        ('9', 9),
        ('8', 8),
        ('7', 7),
        ('6', 6),
        ('5', 5),
        ('4', 4),
        ('3', 3),
        ('2', 2)
    ]);

    let mut count_cards: HashMap<char, u8> = HashMap::new();
    for c in hand.chars() {
        if count_cards.contains_key(&c) {
            count_cards.insert(c, count_cards[&c] + 1);
        } else {
            count_cards.insert(c, 1);
        }
    }
    let mut card_count_vec: Vec<(char, u8)> = count_cards.iter().map(|(&c, &count)| (c, count)).collect();
    card_count_vec.sort_by_key(|x| x.1);
    card_count_vec = card_count_vec.into_iter().rev().take(5).collect();

    let mut score_bytes: [u8; 8] = [0; 8];
    for (idx, c) in hand.chars().enumerate() {
        score_bytes[idx + 1] = card_hash_map[&c];
    }
    
    if card_count_vec[0].1 == 5 {
        score_bytes[0] = 7;
    } else if card_count_vec[0].1 == 4 {
        score_bytes[0] = 6;
    } else if card_count_vec[0].1 == 3 && card_count_vec[1].1 == 2 {
        score_bytes[0] = 5;
    } else if card_count_vec[0].1 == 3 {
        score_bytes[0] = 4;
    } else if card_count_vec[0].1 == 2 && card_count_vec[1].1 == 2 {
        score_bytes[0] = 3;
    } else if card_count_vec[0].1 == 2 {
        score_bytes[0] = 2;
    } else {
        score_bytes[0] = 1;
    }

    return u64::from_be_bytes(score_bytes);
}

fn main() {

    

    let mut input_text: String = fs::read_to_string("../../input.txt").expect("Could not read file!");
    input_text.pop();
    let lines: Vec<&str> = input_text.split('\n').collect();

    let mut hands: Vec<(&str, usize)> = vec![];
    for l in &lines {
        let (hand, bid) = l.split_once(" ").unwrap();
        hands.push((hand, bid.parse::<usize>().unwrap()));
    }
    
    hands.sort_by_key(|(hand, _)| hand_to_score(&hand));

    let total: usize = hands.iter().enumerate().map(|(idx, (_, bid))| bid * (idx + 1)).sum();

    println!("Total bid: {}", total);

}
