use std::collections::HashMap;

fn main() {
    let text = "hello world hello";

    let mut freqs = HashMap::new();

    for word in text.split_whitespace() {
        match freqs.get_mut(word) {
            //first mutable borrow to map at get_mut()
            Some(value) => *value += 1,
            None => {
                freqs.insert(word, 1); // 2nd mutable borrow at insert()
            }
        }
    }

    println!("1. word frequencies: {:#?}", freqs);

    let mut freqs_two = HashMap::new();
    for word in text.split_whitespace() {
        *freqs_two.entry(word).or_insert(0) += 1; // other way of doing thing using entry api.
    }

    println!("2. word frequencies: {:#?}", freqs);
}
