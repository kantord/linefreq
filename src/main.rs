use clap::Parser;
use std::collections::HashMap;
use std::io::{self, BufRead};

static ABOUT: &str =
    "Count the number of times each line occurs in stdin. Does not require lines to be presorted.";

#[derive(Parser, Debug)]
#[command(author, version, about = ABOUT, long_about = None)]
struct Args {
    /// Minimum number of times a line must occur to be printed
    #[arg(short = 'm', long, default_value_t = 1)]
    minimum_count: u128,
}

fn main() {
    let stdin = io::stdin();
    let args = Args::parse();
    let reader = stdin.lock();
    let mut counts: HashMap<String, u128> = HashMap::new();

    for line in reader.lines() {
        let value = counts.entry(line.unwrap()).or_insert(0);
        *value += 1;
    }

    for (key, value) in &counts {
        if *value < args.minimum_count {
            continue;
        }

        println!("{}\t{}", key, value);
    }
}
