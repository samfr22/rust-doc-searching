use std::io;

use doc_search::structures::Hashmap;

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

    // Create the hashmap with the specified number of buckets
    let mut words = Hashmap::new(num_buckets);

    // Read in the subdirectory of text documents and set up the hashmap

    loop {
        // Get the user input for search queries and use searching functions from the library
        print!("Search query:\n>");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read query");

        let query: String = match input.trim().parse() {
            Ok(phrase) => phrase,
            Err(_) => panic!("Invalid query input"),
        };

        // Call the searching functions
    }
}
