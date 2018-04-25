// Project Euler
// Problem 60 - 

struct Sieve {
    array: Vec<bool>,
}

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

        if i > self.array.len() {
            Sieve::backup_is_prime(i)
        } else {
            self.array[i]
        }
    }

    fn backup_is_prime(i: usize) -> bool {
        if i < 2 {
            return false;
        } else if i == 2 {
            return true;
        }

        let mut a = 3;
        while a < ((i as f32).sqrt() as usize) {
            if i % a == 0 {
                return false;
            }

            a += 2;
        }

        true
    }
}

fn combine(a: usize, b: usize) -> usize {
    let size = if b < 10 {
        10
    } else if b < 100 {
        100
    } else if b < 1000 {
        1000
    } else if b < 10000 {
        10000
    } else {
        100000
    };
    
    a * size + b
}

fn main() {
    const MAX: usize = 10_000;
    let sieve = Sieve::with_capacity(1_100_000);

    for a in 2..MAX {
        if !sieve.is_prime(a) {
            continue;
        }
        for b in a..MAX {
            if !sieve.is_prime(b) 
                || !sieve.is_prime(combine(a, b))
                || !sieve.is_prime(combine(b, a))
            {
                continue;
            }
            for c in b..MAX {
                if !sieve.is_prime(c) 
                    || !sieve.is_prime(combine(a, c))
                    || !sieve.is_prime(combine(c, a))
                    || !sieve.is_prime(combine(b, c))
                    || !sieve.is_prime(combine(c, b))
                {
                    continue;
                }
                for d in c..MAX {
                    if !sieve.is_prime(d) 
                        || !sieve.is_prime(combine(a, d))
                        || !sieve.is_prime(combine(d, a))
                        || !sieve.is_prime(combine(b, d))
                        || !sieve.is_prime(combine(d, b))
                        || !sieve.is_prime(combine(d, c))
                        || !sieve.is_prime(combine(c, d))
                    {
                        continue;
                    }
                    for e in d..MAX {
                        if !sieve.is_prime(e) {
                            continue;
                        }
                        if sieve.is_prime(combine(a, e)) && sieve.is_prime(combine(e, a))
                            && sieve.is_prime(combine(b, e))
                            && sieve.is_prime(combine(e, b))
                            && sieve.is_prime(combine(c, e))
                            && sieve.is_prime(combine(e, c))
                            && sieve.is_prime(combine(d, e))
                            && sieve.is_prime(combine(e, d))
                        {
                            println!(
                                "{} = {} + {} + {} + {} + {}",
                                a + b + c + d + e,
                                a,
                                b,
                                c,
                                d,
                                e
                            );
                            return;
                        }
                    }
                }
            }
        }
    }
}
