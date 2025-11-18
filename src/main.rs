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
