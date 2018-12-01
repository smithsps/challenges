use std::collections::HashSet;

struct Cyclical {
    figurates: Vec<Vec<Vec<u32>>>,
}

impl Cyclical {
    fn new() -> Cyclical {
        let mut figurates = vec![Vec::new(); 3];

        // Triangle n(n+1)/2
        figurates.push(Cyclical::make_figurates(10000, |n| (n + 1) * n / 2));
        // Square n^2
        figurates.push(Cyclical::make_figurates(10000, |n| n * n));
        // Penta n(3n−1)/2
        figurates.push(Cyclical::make_figurates(10000, |n| (3 * n - 1) * n / 2));
        // Sexa n(2n−1)
        figurates.push(Cyclical::make_figurates(10000, |n| n * (2 * n - 1)));
        // Septa n(5n−3)/2
        figurates.push(Cyclical::make_figurates(10000, |n| (5 * n - 3) * n / 2));
        // Octa n(3n−2)
        figurates.push(Cyclical::make_figurates(10000, |n| n * (3 * n - 2)));

        Cyclical {
            figurates: figurates,
        }
    }

    fn make_figurates<F>(max: usize, closure: F) -> Vec<Vec<u32>>
    where
        F: Fn(usize) -> usize,
    {
        let mut figs: Vec<Vec<u32>> = vec![Vec::new(); 100];
        for n in 1..1000 {
            let r = closure(n);

            if r >= max {
                break;
            }

            figs[r / 100].push(r as u32);
        }

        figs
    }

    fn lookup(&self, fig: u8, digits: u8) -> Option<&Vec<u32>> {
        if digits < 10 || self.figurates[fig as usize].is_empty() {
            return None;
        }

        Some(&self.figurates[fig as usize][digits as usize])
    }

    fn find_chains(
        &self,
        first_two_digits: u8,
        two_digits: u8,
        unused: HashSet<u8>,
    ) -> (bool, u32) {
        if unused.is_empty() {
            if first_two_digits == two_digits {
                return (true, 0);
            }
            return (false, 0);
        }

        let unused_copy = unused.clone();

        for fig in unused {
            let mut set = unused_copy.clone();
            set.remove(&fig);

            let lookup = self.lookup(fig, two_digits);

            if lookup.is_some() {
                for n in lookup.unwrap() {
                    let (success, sum) =
                        self.find_chains(first_two_digits, (n % 100) as u8, set.clone());
                    if success {
                        println!("{} : {}", fig, n);
                        return (true, sum + n);
                    }
                }
            }
        }

        (false, 0)
    }
}

fn main() {
    let cycl = Cyclical::new();
    let full_set: HashSet<u8> = [3, 4, 5, 6, 7, 8].iter().cloned().collect();

    //println!("{:?}", cycl.figurates);

    for fig in 3..9 {
        let mut set = full_set.clone();
        set.remove(&(fig as u8));

        for n in cycl.figurates[fig].iter().flat_map(|n| n) {
            if *n < 1000 {
                continue;
            }

            let (success, sum) = cycl.find_chains((n / 100) as u8, (n % 100) as u8, set.clone());
            if success {
                println!("{} : {}", fig, n);
                println!("Sum: {}", sum + n);
                return;
            }
        }
    }
}
