use std::io::prelude::*;
use std::fs::File;

fn main() {
    let mut f = File::open("p059_cipher.txt").expect("missing file");
    let mut encrypted = String::new();
    f.read_to_string(&mut encrypted)
        .expect("unable to read file");
    let encrypted: Vec<u8> = encrypted
        .split(",")
        .map(|n| n.parse::<u8>().expect("parse error"))
        .collect();

    for a in 97..123 {
        for b in 97..123 {
            for c in 97..123 {
                let word = vec![a, b, c];
                let mut word_iter = word.iter().cycle();
                let mut message = String::new();

                let mut good_count = 0;
                for c in &encrypted {
                    let new_char: u8 = c ^ word_iter.next().unwrap().clone();

                    if (new_char > 64 && new_char < 91) 
                        || (new_char > 96 && new_char < 123)
                        || new_char == 32
                    {
                        good_count += 1;
                    }

                    message.push(new_char as char);
                }
                if (good_count as f32) / (message.len() as f32) > 0.90 {
                    println!(
                        "{} {} {} -> {}",
                        a as u8 as char, b as u8 as char, c as u8 as char, message
                    );
                    
                    let mut answer = 0;
                    for c in message.chars() {
                        answer += c as u32;
                    }
                    println!("Answer: {}", answer);

                    return;
                }
            }
        }
    }
}
