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
    println!(
        "Add fruits to the salad.
        Enter 'front fruit_name' or 'back fruit_name' to add
        fruits to the front or back of the salad respectively."
    );

    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.len() != 2 {
        println!("Invalid input. Try again.");
    }
    let fruit_name = parts[1];
    match parts[0] {
        "front" => fruit.push_front(fruit_name),
        "back" => fruit.push_back(fruit_name),
        _ => println!("Invalid input. Try again."),
    }

    // Print out the fruit salad
    print!("Fruit Salad:");
    for (i, item) in fruit.iter().enumerate() {
        if i != fruit.len() - 1 {
            print!("{}, ", item);
        }
        else {
            println!("{}", item);
        }
    }

    // Chose a random fruit
    let fruit: Vec<_> = fruit.into_iter().collect();
    let random_fruit = fruit.choose(&mut rng).unwrap();
    println!("Random fruit: {}", random_fruit);

    // Remove from front or back
    let mut fruit: VecDeque<_> = fruit.into_iter().collect();
    let mut choice: String = String::new();
    println!("Remove from front or back?");
    io::stdin().read_line(&mut choice).unwrap();
    match choice.trim() {
        "front" => {
            let removed_fruit = fruit.pop_front().unwrap();
            println!("Removed fruit from the front: {}", removed_fruit);
        }
        "back" => {
            let removed_fruit = fruit.pop_back().unwrap();
            println!("Removed fruit from the back: {}", removed_fruit);
        }
        _ => println!("Invalid choice."),
    }

    // Final state of the queue
    let fruit: Vec<_> = fruit.into_iter().collect();
    println!("Fruit Salad:");
    for (i, item) in fruit.iter().enumerate() {
        if i != fruit.len() - 1 {
            println!("{}, ", item);
        } else {
            println!("{}", item);
        }
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
