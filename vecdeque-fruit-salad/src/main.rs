use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::VecDeque;

fn main(){
    let mut fruit: VecDeque<&str> = VecDeque::new();
    fruit.push_back("apple");
    fruit.push_back("banana");
    fruit.push_back("cherry");

    //scramble fruit
    let mut rng = thread_rng();
    //convert to vec
    let mut fruit_vec: Vec<_> = fruit.into_iter().collect();
    //shuffle
    fruit_vec.shuffle(&mut rng);

    //convert back to vecdeque
    let mut fruits2: VecDeque<&str> = fruit_vec.into_iter().collect();

    // add fruits to both ends:
    fruits2.push_front("date");
    fruits2.push_back("elderberry");

    //print fruits 2
    println!("Fruit salad:");
    for (i, item) in fruits2.iter().enumerate(){
        println!("{}", item);
    }
}


// fix value of type `Vec<&str>` cannot be built from `std::iter::Iterator<Item=&&str>`
