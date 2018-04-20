// Daily Programmer
// Challenge #357 - Intermediate
// https://www.reddit.com/r/dailyprogrammer/comments/8df7sm/

fn kolakoski_brute(n: usize) -> (u32, u32) {
    let mut count = 1;
    let mut i = 1;
    let mut j = 1;
    let mut a: Vec<u8> = vec![1];

    while i < n {
        let next = if a[i - 1] == 1 { 2 } else { 1 };
        if j > 1 && a[j] == 1 {
            a.push(next);
            i += 1;
            j += 1;
            if next == 1 {
                count += 1;
            }
        } else {
            a.push(next);
            a.push(next);
            i += 2;
            j += 1;
            if next == 1 {
                count += 2;
            }
        }
    }

    // If i goes over n by one
    if i == n + 1 {
        if a[i - i] == 1 {
            count -= 1;
        }
    }

    (count, n as u32 - count)
}

// David Eppstein
// https://11011110.github.io/blog/2016/10/14/kolakoski-sequence-via.html
fn kolakoski_fast(n: u64) -> (u64, u64) {
    let mut count: u64 = 0;
    let mut x = u64::max_value();
    let mut y = u64::max_value();
    for _ in 0..n {
        count += x & 1;
        let f = y & !(y + 1);
        x ^= f;
        y = (y + 1) | (f & (x >> 1));
    }

    (count, n - count)
}

fn main() {
    let n = 1_000_000_000_000;
    //let c = kolakoski_brute(n as usize);
    //println!("{}:{}", c.0, c.1);
    let c = kolakoski_fast(n);
    println!("{}:{}", c.0, c.1);
}


#[cfg(test)]
mod tests {
    use kolakoski_brute;
    use kolakoski_fast;

    #[test]
    fn n_100() {
        assert_eq!(kolakoski_brute(29), (15, 14));
        assert_eq!(kolakoski_fast(29), (15, 14));
    }

    #[test]
    fn n_1000() {
        assert_eq!(kolakoski_brute(1000), (502, 1000 - 502));
        assert_eq!(kolakoski_fast(1000), (502, 1000 - 502));
    }

    #[test]
    fn n_1000000() {
        assert_eq!(kolakoski_brute(1000000), (499986, 1000000 - 499986));
        assert_eq!(kolakoski_fast(1000000), (499986, 1000000 - 499986));
    }
}
