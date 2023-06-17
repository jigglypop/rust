use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(1, 10);
    scores.insert(2, 20);
    println!("{:?}", scores);
}
