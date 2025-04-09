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
    let mut rng = thread_rng();

    let mut fruit_2 = vec!["Mango", "Banana", "Grapes", "Oranges", "Strawberry"];

    println!("Please enter the name of the fruit you want to add to the basket: ");
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    println!("Please enter the number of fruits you want to add to the basket: (maximum 3)");
    let mut user_input_2 = String::new();
    io::stdin()
        .read_line(&mut user_input_2)
        .expect("Failed to read line");
    let user_input_2: i32 = user_input_2.trim().parse().expect("Please enter a number!");

    if user_input_2 > 3 {
        println!("You can only add a maximum of 3 fruits.");
        return;
    }

    let random_fruits: Vec<_> = fruit_2
        .choose_multiple(&mut rng, user_input_2 as usize)
        .collect();
    println!("Randomly selected fruits: {:?}", random_fruits);

    fruit.extend(random_fruits);

    fruit.push(user_input.trim()); // Add user input to the fruit vector

    // Scramble (shuffle) the fruit
    let mut rng = thread_rng();
    fruit.shuffle(&mut rng);

    if let Some(random_fruit) = fruit.choose(&mut rng) {
        println!("Random fruit: {}", random_fruit);
    } else {
        println!("No fruit available.");
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
}
