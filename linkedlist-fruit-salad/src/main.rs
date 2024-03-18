use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::LinkedList;


fn main() {
    let mut fruit: LinkedList<&str> = LinkedList::new();
    fruit.push_back("apple");
    fruit.push_back("banana");
    fruit.push_back("cherry");

    //unusual operation of converting linked list to vec
    //scramble fruit
    let mut rng = thread_rng();
    //convert to vec
    let mut fruit_vec: Vec<_> = fruit.into_iter().collect();
    //shuffle
    fruit_vec.shuffle(&mut rng);

    //convert back to linked list
    let mut fruits2: LinkedList<&str> = fruit_vec.into_iter().collect();


    //add fruits after shuffling to front/back of vec
    fruits2.push_front("date");
    fruits2.push_back("elderberry");

    
    //print linkedlist
    println!("Fruit salad:");
    for (i, item) in fruits2.iter().enumerate(){
        println!("{}", item);
    }
    

}
