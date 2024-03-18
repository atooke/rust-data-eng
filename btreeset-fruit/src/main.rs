use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::BTreeSet;

fn main() {
    let fruits = vec![
        "apple",
        "banana",
        "cherry",
        "date",
        "elderberry",
        "fig",
        "grape",
        "honeydew",
        "kiwi",
        "lemon",
        "mango",
        "nectarine",
        "orange",
        "pear",
        "quince",
        "raspberry",
        "strawberry",
        "tangerine",
        "ugli",
        "valencia",
        "watermelon",
        "xigua",
        "yellow watermelon",
        "zucchini",

    ];
    //let amounts = [1, 3, 5, 7, 9];
    // rust version of list comphrension
    let amounts: Vec<usize> = (0..fruits.len()).map(|x| x ).collect();

    let mut rng = thread_rng();

    for amount in amounts.iter() {
        let mut fruit_set = BTreeSet::new();
        let mut shuffled_fruits = fruits.clone();
        shuffled_fruits.shuffle(&mut rng);

        for fruit in shuffled_fruits {
            fruit_set.insert(fruit);
            if fruit_set.len() >= *amount {
                break;
            }
        }

        println!("{}: {:?}", amount, fruit_set);
    }
}