use clap::Parser;
use rand::prelude::*;
use std::process;

#[derive(Parser)]
struct Args {
    #[arg(short)]
    n: u32,
    #[arg(short, default_value_t = 1)]
    m: u32,
}

fn create_number(d: u32, rng: &mut ThreadRng) -> i64 {
    let mut res = 0;

    for i in 0..d {
        let n: i64 = rng.random_range((if i == 0 { 1 } else { 0 })..10);

        res *= 10;
        res += n;
    }

    return res;
}

fn mod_pow(mut a: i128, mut n: i128, m: i128) -> i128 {
    let mut res = 1;

    while n != 0 {
        if n % 2 != 0 {
            res = (res * a) % m;
        }
        a = (a * a) % m;
        n >>= 1
    }
    return res;
}

fn check_prime(n: i128) -> bool {
    if n == 2 {
        return true;
    } else if n == 1 || (n & 1) == 0 {
        return false;
    }

    let mut s = 0;
    let mut d = n - 1;

    while d % 2 == 0 {
        s += 1;
        d >>= 1;
    }

    let check_list: Vec<i128> = vec![2, 325, 9375, 28178, 450775, 9780504, 1795265022];
    for p in check_list {
        if p > n {
            break;
        }

        let mut t = 0;
        let mut x = mod_pow(p, d, n);

        if x == 1 {
            continue;
        }

        while t < s {
            if x == n - 1 {
                break;
            }

            x = (x * x) % n;
            t += 1;
        }

        if t == s {
            return false;
        }
    }

    return true;
}

#[cfg(test)]
mod tests {
    use crate::check_prime;
    #[test]
    fn check_prime_test() {
        assert_eq!(check_prime(2), true);
        assert_eq!(check_prime(341), false);
        assert_eq!(check_prime(998244353), true);
    }
}

fn main() {
    let args = Args::parse();
    let mut rng = rand::rng();

    if args.n >= 18 {
        println!("this tool supports generating prime numbers up to 17 digits");
        process::exit(1);
    }

    for _ in 0..args.m {
        println!("{}", create_number(args.n, &mut rng))
    }
}
