use clap::Parser;
use std::process;

#[derive(Parser)]
struct Args {
    #[arg(short)]
    n: u32,
    #[arg(short, default_value_t = 1)]
    m: u32,
}

fn main() {
    let args = Args::parse();

    if args.n >= 18 {
        println!("this tool supports generating prime numbers up to 17 digits");
        process::exit(1);
    }
}
