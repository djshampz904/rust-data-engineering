/*
This code starts with an initial VecDeque,
converts it to a Vec for shuffling, and then converts it back to a VecDeque.
After that, it pushes "Pomegranate" to the front of the deque, and "Fig" and "Cherry"
to the back of the deque. Finally, it prints out the final fruit salad.

A VecDeque is a double-ended queue, which means that you can push and pop from both ends
of the queue.
*/

use rand::seq::SliceRandom; // rand is a random number generation library in Rust
use rand::{thread_rng, Rng};
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

    // Convert it back to VecDeque
    let mut fruit: VecDeque<_> = fruit.into_iter().collect();

    // Add fruits to the both ends of the queue after shuffling
    fruit.push_front("Pomegranate");
    fruit.push_back("Fig");
    fruit.push_back("Cherry");

    let mut fruit: VecDeque<_> = fruit.into_iter().collect();

    let mut user_input_end = String::new();
    println!("Please enter the name of the fruit you want to add at the end of the vec: ");
    io::stdin()
        .read_line(&mut user_input_end)
        .expect("Failed to read line");

    fruit.push_back(user_input_end.trim());

    let fruit: Vec<_> = fruit.into_iter().collect();

    let random_fruit = fruit.choose(&mut rng).unwrap();
    println!("Random fruit selected: {}", random_fruit);

    let mut fruit: VecDeque<_> = fruit.into_iter().collect();

    // Print out the fruit salad
    println!("Fruit Salad:");
    for (i, item) in fruit.iter().enumerate() {
        if i != fruit.len() - 1 {
            print!("{}, ", item);
        } else {
            println!("{}", item);
        }
    }

    if rng.gen_bool(0.5) {
        println!("{:?}", fruit.pop_back().unwrap())
    } else {
        println!("{:?}", fruit.pop_front().unwrap())
    }
}
