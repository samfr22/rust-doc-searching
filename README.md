# text-doc-search
Searches a subdirectory of text documents for the most relevant docs based on a search query

Based on a project previously completed in C for CS 2461 - Computer Architecture I.

This project was done as a way to learn Rust by porting an existing project to the language. Additionally, the hash function used for this implementation is different than the original project, and a new search algorithm will be used in the future. For now though, a tf-idf ranking is performed, as was done in the original implementation.

Takes in user input to determine the size of a hashmap used to store all of the words from the text files in a subdirectory. Once setup, the user can continously enter search queries and receive the most relevant of the text documents for the given query. 
