use clap::Parser;
use cli_salad::create_fruit_salad;

#[derive(Parser)]
#[clap(version = "4.5.2", author = "Your Name", about = "A basic example")]

struct Opts {
    #[clap(short, long, default_value = "3")]
    num_fruits: usize,
}

fn main() {
    let opts: Opts = Opts::parse();
    let num_fruits = opts.num_fruits;

    let salad = create_fruit_salad(num_fruits);
    println!("Here's your fruit salad: {:?}", salad);
}
