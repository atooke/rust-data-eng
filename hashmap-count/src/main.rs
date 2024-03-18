/*
This example code counts the frequency of each number in the vector.
 */
use std::collections::HashMap;
use std::collections::hash_map::Entry;

fn logic(numbers: Vec<i32>) -> Vec<(i32, u32)> {
    let mut frequencies = HashMap::new();

    for num in numbers {
        let frequency = frequencies.entry(num).or_insert(0);
        *frequency += 1;
    }

    let mut result = Vec::new();

    for (num, frequency) in frequencies {
        result.push((num, frequency));
    }

    result
}

fn count_freq_v2(numbers: Vec<i32>) -> Vec<(i32, i32)> {
    let mut counts: HashMap<i32, i32> = HashMap::default();

    for num in numbers{
        match counts.entry(num) {
            Entry::Occupied(mut entry) => *entry.get_mut() += 1,
            Entry::Vacant(entry) => {
                // Note that even though this may look similar to the common pattern,
                // not using `Entry`, this pattern doesn't pass the key again,
                // so this is more efficient.
                entry.insert(1);
            }
        }
    }
    counts.into_iter().collect()
}



fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 3];
    let result = logic(numbers.clone());
    //print the results in a human readable format that explains what the result is.
    println!(
        "The frequency of each number in the vector is: {:?}",
        result
    );

    let result2 = count_freq_v2(numbers);
    println!(
        "The frequency of each number in the vector is: {:?}",
        result2
    );

}
