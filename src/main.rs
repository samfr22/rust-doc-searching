use std::mem::drop;
use std::{io, process};

use doc_search::searching;

fn main() {
    // Get user input about the number of buckets to use with the hashmap
    let mut input = String::new();

    println!("Enter the number of buckets for the hashmap");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let num_buckets: usize = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Invalid bucket input"),
    };

    // Setup the 'engine'
    match searching::setup(num_buckets) {
        Ok(mut config) => {
            // Setup successful - starting queries
            loop {
                // Get the user input for search queries
                println!("Search query:");

                let mut user_in = String::new();

                io::stdin()
                    .read_line(&mut user_in)
                    .expect("Failed to read query");

                let query: String = match user_in.trim().parse() {
                    Ok(phrase) => phrase,
                    Err(_) => panic!("Invalid query input"),
                };

                // Check if query is the exit command
                if query == "x" || query == "X" {
                    println!("Exiting...");
                    drop(config);
                    println!("Total output can be seen in search_scores.txt file");
                    // Hashmap will be dropped by the cleanup of the code
                    process::exit(0);
                }

                // Otherwise, run the query through the ranking algorithm
                match searching::read_and_rank(&mut config, &query) {
                    Ok(()) => continue,
                    Err(e) => panic!("Error in ranking: {}", e),
                }
            }
        }
        Err(e) => panic!("Error setting up: {}", e),
    }
}
