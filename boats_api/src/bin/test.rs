use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish() % 3
}

fn main() {
    let hashs: Vec<u64> = vec![
        "0000002AR",
        "0000008AR",
        "0000011AR",
        "0000019AR",
        "0001143AR",
        "0001157AR",
        "0001173AR",
        "0001337AR",
        "0001352AR",
        "0001354AR",
        "0001484AR"
    ]
    .iter()
    .map(|id| calculate_hash(id))
    .collect();

    println!("{:#?}", hashs);
}


