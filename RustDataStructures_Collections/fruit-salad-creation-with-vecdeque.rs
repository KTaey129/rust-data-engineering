/*
This code starts with an initial VecDeque,
converts it to a Vec for shuffling, and then converts it back to a VecDeque.
After that, it pushes "Pomegranate" to the front of the deque, and "Fig" and "Cherry"
to the back of the deque. Finally, it prints out the final fruit salad.

A VecDeque is a double-ended queue, which means that you can push and pop from both ends
of the queue.
*/

use rand::seq::SliceRandom; // rand is a random number generation library in Rust
use rand::thread_rng;
use std::collections::VecDeque;
use std::io;

fn main() {
    let mut fruit: VecDeque<&str> = VecDeque::new();
    fruit.push_back("Arbutus");
    fruit.push_back("Loquat");
    fruit.push_back("Strawberry Tree Berry");

    // Scramble (shuffle) the fruit
    let mut rng = thread_rng();
    let mut fruit: Vec<_> = fruit.into_iter().collect();
    fruit.shuffle(&mut rng);

    let mut fruit: VecDeque<&str> = fruit.into_iter().collect();
    fruit.push_front("Pomegranate");
    fruit.push_back("Fig");
    fruit.push_back("Cherry");

    // User interaction to add fruits
    let mut input = String::new();
    println!("Add fruits to the queue. Enter 'front fruit_name' or 'back fruit_name' (or 'exit' to quit):");
    loop {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if input == "exit" {
            break;
        }
        let parts: Vec<&str> = input.splitn(2, ' ').collect();
        if parts.len() == 2{
            match parts[0] {
                "front" => fruit.push_front(parts[1]),
                "back" => fruit.push_back(parts[1]),
                _ => println!("Invalid input. Use 'front' or 'back' to add fruits.")
            }
        }
        else {
            println!("Invalid input. Use 'front fruit_name' or 'back fruit_name' to add fruits.");
        }

    }

    // Chose a random fruit
    let mut fruit: Vec<_> = fruit.into_iter().collect();
    if let Some(random_fruit) = fruit.choose(&mut rng) {
        println!("your random fruit: {}", random_fruit);
    }

    // Remove from front or back
    let mut fruit: VecDeque<_> = fruit.into_iter().collect();
    if let Some(removed) = fruit.pop_front() {
        println!("Removed from front: {}", removed);
    }

    // Final state of the queue
    println!("Final state of the queue:");
    for item in &fruit {
        println!("{}", item);
    }

    // // Convert it back to VecDeque
    // let mut fruit: VecDeque<_> = fruit.into_iter().collect();

    // // Add fruits to the both ends of the queue after shuffling
    // fruit.push_front("Pomegranate");
    // fruit.push_back("Fig");
    // fruit.push_back("Cherry");

    // // Print out the fruit salad
    // println!("Fruit Salad:");
    // for (i, item) in fruit.iter().enumerate() {
    //     if i != fruit.len() - 1 {
    //         print!("{}, ", item);
    //     } else {
    //         println!("{}", item);
    //     }
    // }
}
