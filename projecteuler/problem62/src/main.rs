use std::collections::HashMap;

fn count_vec(n: u64) -> Vec<u32> {
    let mut v = vec![0; 10];

    let mut n = n;

    while n > 0 {
        v[(n % 10) as usize] += 1;
        n /= 10;
    }

    v
}

fn main() {
    let mut cubes: HashMap<Vec<u32>, Vec<u64>> = HashMap::new();

    for i in 0..100000 {
        let cube: u64 = i * i * i;
        let count = count_vec(cube);

        let entry = cubes.entry(count).or_insert(Vec::new());
        entry.push(cube);

        if entry.len() == 5 {
            println!("{:?}\n {}", entry, (entry[0] as f64).cbrt());
            return;
        }
    }
}
