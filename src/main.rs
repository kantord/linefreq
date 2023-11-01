use clap::Parser;
use std::collections::HashMap;
use std::io::{self, BufRead};

static ABOUT: &str =
    "Count the number of times each line occurs in stdin. Does not require lines to be presorted.";

#[derive(Parser, Debug)]
#[command(author, version, about = ABOUT, long_about = None)]
struct Args {}

fn main() {
    let stdin = io::stdin();
    Args::parse();
    let reader = stdin.lock();
    let mut counts: HashMap<String, u128> = HashMap::new();

    for line in reader.lines() {
        let value = counts.entry(line.unwrap()).or_insert(0);
        *value += 1;
    }

    for (key, value) in &counts {
        println!("{},{}", key, value);
    }
}
