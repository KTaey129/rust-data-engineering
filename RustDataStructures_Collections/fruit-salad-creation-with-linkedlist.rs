/*
As with the VecDeque example, this code starts by creating a LinkedList of fruits,
converts it to a Vec for shuffling, and then converts it back to a LinkedList.
After the shuffling, it adds "Pomegranate", "Fig", and "Cherry" to the end of the list.
Finally, it prints out the final fruit salad.

This example shows how to use a LinkedList, but remember that LinkedList
has a higher memory overhead and worse cache locality than Vec or VecDeque,
so it's typically not the best choice unless you have a specific need for the properties
of a linked list. In Rust, it's usually better to use a Vec or VecDeque.

A LinkedList is a doubly-linked list, which means that each element in the list
has a pointer to the next element and the previous element.
A great example of when to use a LinkedList is when you need to insert or remove elements
from the middle of the list.
*/

use core::str;
use rand::seq::SliceRandom; // rand is a random number generation library in Rust
use rand::thread_rng;
use std::collections::LinkedList;
use std::io;

fn main() {
    let mut fruit: LinkedList<&str> = LinkedList::new();
    fruit.push_back("Arbutus");
    fruit.push_back("Loquat");
    fruit.push_back("Strawberry Tree Berry");

    /*
    Please note that converting a LinkedList to a Vec and back to a LinkedList
    isn't a common operation in practice. I included
    it in this example to keep the code as similar as possible
    to the original VecDeque example.
     */

    // Scramble (shuffle) the fruit
    let mut rng = thread_rng();
    let mut fruit: Vec<_> = fruit.into_iter().collect();
    fruit.shuffle(&mut rng);

    // Convert it back to LinkedList
    let mut fruit: LinkedList<_> = fruit.into_iter().collect();

    // Add fruits to the both ends of the list after shuffling
    fruit.push_front("Pomegranate");
    fruit.push_back("Fig");
    fruit.push_back("Cherry");

    // Print out the fruit salad
    println!("Fruit Salad:");
    for (i, item) in fruit.iter().enumerate() {
        if i != fruit.len() - 1 {
            print!("{}, ", item);
        } else {
            println!("{}", item);
        }
    }

    // A user adds fruits at any position in the LinkedList

    println!("Enter the fruit and the position to add in to the salad(ex., fruit, position): ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let input: Vec<&str> = input.trim().split(", ").collect();
    if input.len() != 2 {
        println!("Invalid input format. Please use 'fruit, position'.");
        return;
    }
    let input_fruit: String = input[0].to_string();
    let input_position: usize = match input[1].parse() {
        Ok(pos) => pos,
        Err(_) => {
            println!("Invalid position. Please enter a number.");
            return;
        }
    };
    // A container to store user-provided fruits so we can borrow &str
    let mut user_fruits: Vec<String> = vec![input_fruit]; // Store the input string
    let input_fruit_ref: &str = &user_fruits[0]; // Get a &str reference

    // Insert the fruit at the specified position
    let mut new_list = LinkedList::new();
    let mut index = 0;

    while let Some(item) = fruit.pop_front() {
        if index == input_position {
            new_list.push_back(input_fruit_ref);
            // Immutable Iteration for Insertion:
            // The for loop iterates over the LinkedList immutably.
            // You cannot use push_back or modify the list during iteration.
        }
        new_list.push_back(item);
        index += 1;
    }

    // If position is out of bounds, append to the end
    if input_position >= index {
        new_list.push_back(input_fruit_ref);
    }

    println!(
        "Fruit Salad after adding a fruit at position {}: ",
        input_position
    );

    for (i, item) in new_list.iter().enumerate() {
        if i != new_list.len() - 1 {
            print!("{}, ", item);
        } else {
            println!("{}", item);
        }
    }
}
