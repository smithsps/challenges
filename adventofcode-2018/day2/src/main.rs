use std::env;
use std::collections::HashMap;

fn main() {
    let args: Vec<_> = env::args().collect();
    let id_list = include_str!("input.txt").lines().collect();

    if args.len() == 2 {
        match args[1].as_str() {
            "1" => part_one(id_list),
            "2" => part_two(id_list),
            _ => println!("Only two parts of problem"),
        }
    } else {
        println!("Run Syntax:");
        println!("day2 <problem-part-number>");
    }
}

fn part_one(id_list: Vec<&str>) {
    let mut double_total = 0;
    let mut triple_total = 0;

    for id in id_list {
        let mut letter_count: HashMap<char, u8> = HashMap::with_capacity(26);

        for character in id.chars() {
            letter_count.entry(character)
                .and_modify(|count| { *count += 1 })
                .or_insert(1);
        }

        if letter_count.values().find(|total| **total == 2).is_some() {
            double_total += 1;
        }

        if letter_count.values().find(|total| **total == 3).is_some() {
            triple_total += 1;
        }
    }
    
    println!("Checksum is {} * {} = {}", double_total, triple_total, double_total * triple_total);
} 

fn part_two(id_list: Vec<&str>) {
    for word1 in &id_list {
        for word2 in &id_list {
            let (distance, matching_letters) = levenshtein(word1, word2);
            if distance == 1 {
                println!("Matching Letters from ID: {}", matching_letters);
                return
            }
        }
    }
}

fn levenshtein(w1: &str, w2: &str) -> (usize, String) {
    let matching_letters: String = w1.chars().zip(w2.chars())
        .filter(|(c1, c2)| c1 == c2)
        .map(|(c1, _)| c1)
        .collect();

    (w1.len() - matching_letters.len(), matching_letters)
} 