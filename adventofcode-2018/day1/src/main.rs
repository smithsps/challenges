use std::env;
use std::collections::HashSet;

fn main() {
    let args: Vec<_> = env::args().collect();
    let freq_list = parse_list(include_str!("input.txt"));

    if args.len() == 2 {
        match args[1].as_str() {
            "1" => part_one(freq_list),
            "2" => part_two(freq_list),
            _ => println!("Only two parts of problem"),
        }
    } else {
        println!("Format");
        println!("day1 <problem-part-number>");
    }
}

fn part_one(freq_list: Vec<i32>) {
    let mut freq_sum = 0;

    for f in freq_list {
        freq_sum += f;
    }

    println!("Chronal Calibration Frequency: {}", freq_sum);
} 

fn part_two(freq_list: Vec<i32>) {
    let mut freq_set: HashSet<i32> = HashSet::with_capacity(4096);
    let mut freq_sum = 0;

    loop {
        for f in &freq_list {
            freq_sum += f;
            // HashSet::insert return false if value is already in set
            if !freq_set.insert(freq_sum) {
                println!("Chronal Calibration Repeated Frequency: {}", freq_sum);
                return;
            }
        }
    }
}

fn parse_list(freq_list: &str) -> Vec<i32> {
    let mut vec = Vec::with_capacity(1036);

    for f_str in freq_list.lines() {
        let f = f_str.parse::<i32>();
        match f {
            Ok(f) => vec.push(f),
            Err(_) => println!("Invalid attempt to parse string: {}", f_str),
        }
    }

    vec
}