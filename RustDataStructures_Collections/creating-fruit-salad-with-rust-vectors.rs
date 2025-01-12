/*
This program creates a fruit salad by scrambling (shuffling) a list of fruit.
A vector is a growable array. It can grow or shrink in size and is one of the most
useful data structures in Rust. A vector is represented using the Vec<T> type.
*/

use rand::seq::SliceRandom; // rand is a random number generation library in Rust
use rand::thread_rng;
use std::io;

fn main() {
    let mut fruit = vec![
        "Orange",
        "Fig",
        "Pomegranate",
        "Cherry",
        "Apple",
        "Pear",
        "Peach",
    ];

    // show the current fruit vector
    println!("{:?}", fruit);

    // accept fruits from the user and then add the to the fruit salad
    let mut input = String::new();
    println!("Enter fruits to add to the salad (comma-separated):");
    io::stdin().read_line(&mut input).unwrap();

    // split the input by commas and add to the vector
    let user_fruits: Vec<&str> = input.trim().split(',').map(|s| s.trim()).collect();
    fruit.extend(user_fruits);

    // Scramble (shuffle) the fruit
    let mut rng = thread_rng();
    fruit.shuffle(&mut rng);

    // add a specific number of random fruits to the salad
    let extra_fruits = ["Banana", "Grapes", "Strawberry", "Blueberry"];
    let num_to_add = 2;
    for _ in 0..num_to_add {
        if let Some(random_fruit) = extra_fruits.choose(&mut rng) {
            fruit.push(random_fruit);
        }
    }

    // Print out the fruit salad
    println!("Fruit Salad:");
    for (i, item) in fruit.iter().enumerate() {
        if i != fruit.len() - 1 {
            print!("{}, ", item);
        } else {
            println!("{}", item);
        }
    }
    // using choose method to get a random fruit
    if let Some(random_fruit) = fruit.choose(&mut rng) {
        println!("Random fruit: {}", random_fruit);
    }
}
