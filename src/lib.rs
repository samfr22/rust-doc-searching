mod structures {
    /// The hashmap struct for the document searching module. It holds information
    ///  about the number of words and documents currently tracked as well as the
    ///  actual hashtable itself
    use std::mem::drop;

    pub struct Hashmap {
        /// Number of buckets allocated for the map
        buckets: usize,
        /// Number of words currently in the hashmap
        words: u32,
        /// Number of docs in use
        pub num_docs: i32,
        /// The hashtable itself
        table: Vec<Vec<WordNode>>,
    }

    impl Hashmap {
        // Return a pointer to the allocated hashmap that has the number of 
        //  buckets specified by the user at run-time
        pub fn new(buckets: usize) -> Box<Hashmap> { 
            Box::new(Hashmap {
                buckets,
                words: 0,
                num_docs: 0,
                table: Vec::with_capacity(buckets),
            })
        }

        /// The function for adding to the hashmap. If the given word is not present
        /// already. A new WordNode is added for it. If it is present, the frequency
        /// is increased and, if necessary, a new DocNode is added to the internal
        /// documents list or the already present document will have it's freq increased
        pub fn add(&mut self, word: String, doc: String) {
            // Get the hash index for this word
            let index = self.hash(&word);

            // First check to see if the corresponding vector already has the word
            for node in self.table[index].iter_mut() {
                if node.word == word {
                    // Word found - increase doc_freq and add a new doc if necessary
                    for document in node.documents.iter_mut() {
                        if document.document_name == doc {
                            // Document already present, update term frequency and return
                            document.term_freq += 1;
                            return;
                        }
                    }
                    // Document not found - add a new document to the list for the word
                    node.documents.push(DocNode::new(doc));
                    // Word added - no need to continue checking for the word
                    return;
                }
            }

            // Word was not already found - add a new WordNode
            self.table[index].push(WordNode::new(word, doc));
            self.words += 1;
        }

        /// The function for removing a word from the hashmap. When the word is
        /// found, the node is removed and the list adjusted to cover the hole if
        /// necessary. Once the node has been removed, the method returns
        /// immediately. If there's an error at any point, an error message is 
        /// returned to the caller
        pub fn remove(&mut self, word: String) -> Option<()> {
            // Get the hash index for the word
            let index = self.hash(&word);

            // Iterate through the given list and check to see if it's there
            let mut internal_index: usize = 0; // For tracking where the node is found
            for node in self.table[index].iter_mut() {
                // Compare each node's word to check if it matches
                if node.word == word {
                    // Since it's in a vector, can just drop the node here
                    drop(node);
                    // Check if there's another node past this one
                    // First make sure this was not the end of the vector
                    if internal_index >= self.table[index].capacity() {
                        // Nothing beyond this - Just return early since no need
                        //  to handle moving the rest of the vector over
                        return Some(());
                    }
                    // Check to see whether or not there's another node in the list
                    if self.table[index][internal_index + 1].word.len() != 0 {
                        // There is a node still after the removed node. Iterate
                        //  through the rest of the vector and move it over one
                        // let mut i: usize = internal_index + 1;
                        // while i < self.table[index].capacity() {
                        //     // Move element over
                        //     self.table[index][i - 1] = self.table[index][i - 1];
                            
                        //     i += 1;
                        // }
                    }
                    // Removal done - return a success indication
                    self.words -= 1;
                    return Some(());
                }
                internal_index += 1;
            }

            // Node was not found - return error
            None
        }

        /// Get the document frequency for a given word; This represents the
        /// number of documents that the word appears in
        pub fn get_doc_freq(&self, word: &str) -> Option<i32> {
            // Get the hash index for the given word
            let hashed_index = self.hash(&word.to_string());

            // Check to make sure that the list at the given index is valid
            if self.table[hashed_index][0].word.len() == 0 {
                // Not allocated
                return None;
            }

            // Otherwise the list has been allocated - iterate through the given
            //  given list and try to find the word
            for node in self.table[hashed_index].iter() {
                if node.word == word {
                    // Word found - return the document frequency for this word
                    return Some(node.doc_freq);
                }
            }

            // Could not find the word - return error
            None
        }   

        /// Gets the term frequency for a given word in a given document; This
        /// represents the number of times that the word appears in the doc
        pub fn get_term_freq(&self, word: &str, doc: String) -> Result<i32, &'static str> { 
            // Get the hash index for the given word
            let hashed_index = self.hash(&word.to_string());    
            
            // Iterate through the given list and try and find the word
            for node in self.table[hashed_index].iter() {
                if node.word == word {
                    // Word found - now get the term frequency for the specified
                    //  doc, if possible
                    for document in node.documents.iter() {
                        if document.document_name == doc {
                            // Document found - return the tf of doc for this word
                            return Ok(document.term_freq);
                        }
                    }

                    // Document not found for the word
                    return Err("Document not found for given word");
                }
            }

            // Word not found in map
            Err("Word not found in hashmap")
        }

        /// Get the hashed index for a given word based on the capacity of the hashmap
        /// For every character in the string, it will alternate between doing 
        /// multiplication or addition with a final division hash from
        /// the number of allocated buckets
        fn hash(&self, word: &String) -> usize {
            // The indicator for whether or not it's multiplication or division op
            // 0 -> Multiplication; 1 -> Division
            let mut op_variant: u8 = 0;
            // The final hashed index that will be returned by the function
            // Starts at 1 to let multiplication be the first operation
            let mut hashed_index: u64 = 1;
            for c in word.chars() {
                // Figure out what operation is being used for this character
                match op_variant {
                    0 => {
                        // Multiplication
                        hashed_index *= c as u64;
                        op_variant = 1;
                    },
                    1 => {
                        // Division
                        hashed_index += c as u64;
                        op_variant = 0;
                    },
                    _ => panic!("Unexpected value"),
                }
            }
            // Once finished iterating over the characters, do a final modulus with
            //  the given hashmap's capacity
            hashed_index %= self.buckets as u64;

            hashed_index as usize
        }
    }

    /// Implementing the drop trait for the hashmap to allow the direct control
    /// over the memory operations for the sake of practice
    /// This will be called when the struct goes out of scope at the end of the
    /// destroy_map function
    impl Drop for Hashmap {
        fn drop(&mut self) {
            println!("Dropping the hashmap");
        }
    }

    struct WordNode {
        // The tracked word
        word: String,
        // The number of documents the word appears in
        doc_freq: i32,
        // The list of documents that hold this word
        documents: Vec<DocNode>,
    }

    impl WordNode {
        /// For when a new word is being added to the hashmap. A new node is made
        /// holding the word and sets all the fields to the default levels along
        /// with a new DocNode with the specified document saved
        pub fn new(word: String, doc: String) -> WordNode {
            let mut node = WordNode {
                word,
                doc_freq: 1,
                documents: Vec::with_capacity(4),
            };

            node.documents[0] = DocNode::new(doc);

            node
        }
    }

    struct DocNode {
        // The name of the document referencing the word
        document_name: String,
        // The number of times the word occurs in this document
        term_freq: i32,
    }

    impl DocNode {
        pub fn new(doc: String) -> DocNode {
            DocNode {
                document_name: doc,
                term_freq: 1,
            }
        }
    }
}

pub mod searching {
    use super::structures::Hashmap;

    use std::fs::{read_dir};

    /// Read a given directory of text files into the hashmap with a user
    /// defined number of buckets for a new Hashmap
    pub fn setup(buckets: usize) -> std::io::Result<Hashmap> {
        // Base setup of hashmap
        let mut map = Hashmap::new(buckets);

        // For later
        let i = 0;
        let mut first_doc_contents = String::default();

        // Iterate through the docs in the given directory
        for doc in read_dir("./docs")? {
            match doc {
                Ok(entry) => {              
                    let file_name = match entry.file_name().into_string() {
                        Ok(str) => str,
                        Err(_) => panic!("Couldn't convert file name into usable string"),
                    };

                    // Use the file path to open the file and save the contents
                    //  to a string
                    let file_contents = std::fs::read_to_string(entry.path())?;

                    if i == 0 {
                        first_doc_contents = file_contents.clone();
                    }

                    // Now can get all of the words from the string and input
                    //  them into the hashmap
                    let words: Vec<&str> = file_contents.rsplit(' ').collect();
                    // Iterate through the words and add them to the hashmap
                    for word in words.iter() {
                        // Need to clone file_name since the document isn't
                        // living long enough in the code to allow references
                        map.add(word.to_string(), file_name.clone());
                    }

                    // Update number of documents in map
                    map.num_docs += 1;
                },
                Err(_) => panic!("Error opening file in directory"),
            }
        }

        // All words from all the text files have been added - now clean up 
        //  stop words that are too common and would mess up the rankings

        // For now, the simple solution is to just remove the words that are
        //  in all the files -> Updated later
        // Using the saved contents of the first file
        let words: Vec<&str> = first_doc_contents.rsplit(' ').collect();
        for word in words.iter() {
            // Check to see what the doc frequency for this word is
            match map.get_doc_freq(&word) {
                Some(freq) => {
                    // Check to see how it compares 
                    if freq == map.num_docs {
                        // In all docs - remove it from the map
                        match map.remove(word.to_string()) {
                            Some(()) => continue,
                            None => panic!("Couldn't remove stop word"),
                        }
                    }
                },
                None => println!("Word not found in map - possibly removed earlier"),
            }
        }
        
        Ok(*map)
    }

    /// The function called every time a user-inputted search query is given
    pub fn read_query(map: &Hashmap, query: &String) {

    }

    /// The function that does the final ranking of the documents based on the
    /// given query
    pub fn rank(map: &Hashmap, query: String) {

    }

}