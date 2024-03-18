use rand::seq::SliceRandom;
use rand::thread_rng;


fn main() {
    let mut fruits = vec!["apple", "banana", "cherry", "date", "elderberry", "fig", "grape"];
    let mut rng = thread_rng();
    fruits.shuffle(&mut rng);

    println!("Fruit salad:");
    for (i, item) in fruits.iter().enumerate() {
        println!("{}", item);
           }
}
