use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn create_fruit_salad(num_fruits: usize) -> Vec<String> {
    let mut fruits = vec![
        "apple".to_string(),
        "banana".to_string(),
        "cherry".to_string(),
        "date".to_string(),
        "elderberry".to_string(),
        "fig".to_string(),
        "grape".to_string(),
    ];

    let mut rng = thread_rng();
    fruits.shuffle(&mut rng);

    //return a slice of the first num_fruits
    fruits.truncate(num_fruits);
    fruits
}
