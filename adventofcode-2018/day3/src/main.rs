use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();
    let claim_list = process_claim_list(include_str!("input.txt"));

    if args.len() == 2 {
        match args[1].as_str() {
            "1" => part_one(claim_list),
            "2" => part_two(claim_list),
            _ => println!("Only two parts of problem"),
        }
    } else {
        println!("Run Syntax:");
        println!("day3 <problem-part-number>");
    }
}

struct Claim {
    id: u32,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

fn process_claim_list(list: &str) -> Vec<Claim> {
    let mut claims = vec![];
    for line in list.lines() {
        let numbers = line
            .split(|c: char| !c.is_numeric())
            .filter_map(|s| s.parse::<u32>().ok())
            .collect::<Vec<u32>>();

        claims.push(Claim {
            id: numbers[0],
            x: numbers[1],
            y: numbers[2],
            width: numbers[3],
            height: numbers[4],
        });
    }

    return claims;
}

fn make_claims(claims: &Vec<Claim>) -> Vec<Vec<u8>> {
    let mut fabric = vec![vec![0u8; 1000]; 1000];
    for c in claims {
        for x in c.x..(c.x + c.width) {
            for y in c.y..(c.y + c.height) {
                fabric[y as usize][x as usize] += 1;
            }
        }
    }

    fabric
}

fn part_one(claims: Vec<Claim>) {
    let fabric = make_claims(&claims);

    let total = fabric.iter().flatten()
        .filter(|c| **c >= 2)
        .count();

    println!("Overlapping inches of fabric: {}", total);
}

fn part_two(claims: Vec<Claim>) {
    let fabric = make_claims(&claims);

    'outer: for c in &claims {
        for x in c.x..(c.x + c.width) {
            for y in c.y..(c.y + c.height) {
                if fabric[y as usize][x as usize] != 1 {
                    continue 'outer;
                }
            }
        }
        println!("Found uncontested claim!: #{}", c.id);
    }
}
