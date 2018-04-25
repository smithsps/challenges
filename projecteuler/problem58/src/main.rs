// Project Euler
// Problem 57 - Spiral primes

#[allow(dead_code)]
struct Sieve {
    array: Vec<bool>,
}


#[allow(dead_code)]
impl Sieve {
    fn with_capacity(size: usize) -> Sieve {
        let mut new_sieve = Sieve {
            array: vec![true; size],
        };

        new_sieve.array[0] = false;
        new_sieve.array[1] = false;

        let mut i = 3;
        while i < size {

            let mut index = i * i;
            while index < size {
                new_sieve.array[index] = false;
                index += i;
            }

            i += 2;
        }

        println!("Sieve Complete");
        new_sieve
    }

    fn is_prime(&self, i: usize) -> bool {
        if i != 2 && i % 2 == 0 {
            return false;
        }

        self.array[i]
    }
}

fn is_prime(i: usize) -> bool {
    if i < 2 {
        return false;
    } else if i == 2 {
        return true;
    }

    let mut a = 3;
    while a < ((i as f32).sqrt() as usize) {
        if a % 2 == 0 {
            continue;
        }

        if i % a == 0 {
            return false;
        }

        a += 2;
    }

    true
}

fn main() {
    const MAX: usize = 27000;

    //let sieve = Sieve::with_capacity(MAX * MAX);

    let mut count = 0;
    let mut total = 1;

    let mut i = 3;
    while i < MAX {
        if is_prime(i * i - (i - 1) * 1) {
            count += 1;
        }
        if is_prime(i * i - (i - 1) * 2) {
            count += 1;
        }
        if is_prime(i * i - (i - 1) * 3) {
            count += 1;
        }
        total += 4;

        if count * 10 < total {
            println!(
                "{} -> {} = {}/{} - {:0.9}%",
                i,
                i * i,
                count,
                total,
                count as f64 / total as f64
            );
            break;
        }

        i += 2;
    }
}
