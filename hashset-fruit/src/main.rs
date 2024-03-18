// run via cargo run -- --num-fruits 557.  Max fruits is 8 because of the array of fruits only has 8 distinct elements.
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashSet;
use clap::Parser;


#[derive(Parser)]
#[clap(version = "4.5.2", author = "Your Name", about = "A basic example")]

struct Opts {
    #[clap(short, long, default_value = "3")]
    num_fruits: usize,
}

fn generate_fruit() -> &'static str {
    let fruits = [
        "Apple",
        "Banana",
        "Cherry",
        "Date",
        "Elderberry",
        "Fig",
        "Grape",
        "Honeydew",
    ];
    let mut rng = thread_rng();
    // choose returns an Option, so we use unwrap to get the actual value
    fruits.choose(&mut rng).unwrap()
}

fn main() {

    // get user input from cli
    let opts: Opts = Opts::parse();
    print!("Number of fruits: {}", opts.num_fruits);

    let mut fruit_set = HashSet::new();
    let mut fruit_vec = Vec::new();
    println!("Generating 100 random fruits...");
    for _ in 0..opts.num_fruits {
        // keep track of unique fruits
        fruit_set.insert(generate_fruit());
        // add all fruits to vec
        fruit_vec.push(generate_fruit());
    }

    println!("Number of unique fruits generated: {}", fruit_set.len());
    println!("Total number of fruits generated: {}", fruit_vec.len());
}