fn main() {
    println!("Common Rust Collections:");

    // Sequences
    println!("\n\tSequences:");
    println!("\t\tVec: https://doc.rust-lang.org/std/vec/struct.Vec.html");
    println!("\t\tVecDeque: https://doc.rust-lang.org/std/collections/struct.VecDeque.html");
    println!("\t\tLinkedList: https://doc.rust-lang.org/std/collections/struct.LinkedList.html");

    // Maps
    println!("\n\tMaps:");
    println!("\t\tHashMap: https://doc.rust-lang.org/std/collections/struct.HashMap.html");
    println!("\t\tBTreeMap: https://doc.rust-lang.org/std/collections/struct.BTreeMap.html");

    // Sets
    println!("\n\tSets:");

    println!("\t\tHashSet: https://doc.rust-lang.org/std/collections/struct.HashSet.html");
    println!("\t\tBTreeSet: https://doc.rust-lang.org/std/collections/struct.BTreeSet.html");

    // Misc
    println!("\n\tMisc:");
    println!("\t\tBinaryHeap: https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html");

    // Example program: the user choose which type of collectio nto use, then add or remove elements
    // while the program prints out the state of the collection after each operation
    use std::collections::{HashMap, HashSet};
  
    println!("Choose a collection: 1. Vec  2. HashMap  3. HashSet");
    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice).unwrap();

    match choice.trim() {
        "1" => {
            let mut vec = Vec::new();
            vec.push(42);
            println!("Vector: {:?}", vec);
        }
        "2" => {
            let mut map = HashMap::new();
            map.insert("key", "value");
            println!("HashMap: {:?}", map);
        }
        "3" => {
            let mut set = HashSet::new();
            set.insert(42);
            println!("HashSet: {:?}", set);
        }
        _ => println!("Invalid choice!"),
    }
}

// Makefile reference
// format:
// 	cargo fmt --quiet

// lint:
// 	cargo clippy --quiet

// test:
// 	cargo test --quiet

// run:
// 	cargo run 

// all: format lint test run
