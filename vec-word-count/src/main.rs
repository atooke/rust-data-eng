use std::collections::HashMap;

// reads text from a file into a HashMap
fn load_words() -> HashMap<String, u32> {
    let text = include_str!("words.txt").to_string();
    let mut word_count = HashMap::new();
    for word in text.split_whitespace() {
        let count = word_count.entry(word.to_string()).or_insert(0);
        *count += 1;
    }
    word_count
}

//read text using &str
fn load_words_v2() -> HashMap<String, u32> {
    let text = include_str!("words.txt");
    let mut word_count = HashMap::new();
    for word in text.split_whitespace() {
        let count = word_count.entry(word.to_string()).or_insert(0);
        *count += 1;
    }
    word_count
}


fn main() {
    let words = load_words();
    println!("{:?}", words);
    let words2 = load_words_v2();
    println!("{:?}", words2);    
}
