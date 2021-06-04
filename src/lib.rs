/// The hashmap struct for the document searching module. It holds information
///  about the number of words and documents currently tracked as well as the
///  actual hashtable itself
pub struct Hashmap {
    /// Number of buckets allocated for the map
    buckets: usize,
    /// Number of words currently in the hashmap
    words: u32,
    /// Number of documents currently in use for the hashmap
    num_documents: u32,
    /// The hashtable itself
    table: Vec<WordNode>,
}

impl Hashmap {
    // Return a pointer to the allocated hashmap that has the number of 
    //  buckets specified by the user at run-time
    pub fn new(buckets: usize) -> Box<Hashmap> {    
        Box::new(Hashmap {
            buckets,
            words: 0,
            num_documents: 0,
            table: Vec::with_capacity(buckets),
        })
    }

    /// The function for adding to the hashmap. If the given word is not present
    /// already. A new WordNode is added for it. If it is present, the frequency
    /// is increased and, if necessary, a new DocNode is added to the internal
    /// documents list or the already present document will have it's freq increased
    pub fn add(&mut self, word: String, doc: String) -> Result<(), &'static str> {
        
    }

    /// The function for removing a word from the hashmap. When the word is
    /// found, the node is removed and the list adjusted to cover the hole if
    /// necessary. Once the node has been removed, the method returns
    /// immediately. If there's an error at any point, an error message is 
    /// returned to the caller
    pub fn remove(&mut self, word: String) -> Result<(), &'static str> {
        
    }

    pub fn get_doc_freq(&self, word: String) -> Result<i32, &'static str> {
        
    }

    pub fn get_term_freq(&self, word: String, doc: String) -> Result<i32, &'static str> {       
        
    }

    /// Get the hashed index for a given word based on the capacity of the hashmap
    /// For every character in the string, it will alternate between doing 
    /// multiplication or addition with a final division hash from
    /// the number of allocated buckets
    fn hash(&self, word: String) -> u64 {
        // The indicator for whether or not it's multiplication or division op
        // 0 -> Multiplication; 1 -> Division
        let mut op_variant: u8 = 0;
        // The final hashed index that will be returned by the function
        // Starts at 1 to let multiplication be the first operation
        let mut hashed_index: u64 = 1;
        for c in word.chars() {
            // Figure out what operation is being used for this character
            if op_variant == 0 {
                // Mutliplication
                hashed_index *= c as u64;
                // Update op_variant to be the opposite now
                op_variant = 1;
            } else {
                // Division
                hashed_index += c as u64;
                // Update operation for next pass
                op_variant = 0;
            }
        }
        // Once finished iterating over the characters, do a final modulus with
        //  the given hashmap's capacity
        hashed_index %= self.buckets as u64;

        hashed_index
    }

    /// Removal function -> may not be really needed due to the automatic
    /// cleanup provided. But for practice's sake, keep it present, and also
    /// implement the Deref trait for the struct
    pub fn destroy_map(self) -> Result<(), &'static str> {

    }
}

/// Implementing the drop trait for the hashmap to allow the direct control
/// over the memory operations for the sake of practice
/// This will be called when the struct goes out of scope at the end of the
/// destroy_map function
impl Drop for Hashmap {
    fn drop(&mut self) {

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
        let node = WordNode {
            word,
            doc_freq: 1,
            documents: Vec::with_capacity(4),
        };

        node.documents[0] = DocNode::new(doc);

        node
    }

    fn update_word(word: String, doc: String) -> Result<(), &'static str> {
        
    }
}

struct DocNode {
    // The name of the document referencing the word
    document_name: String,
    // The number of times the word occurs in this document
    term_freq: u32,
}

impl DocNode {
    pub fn new(doc: String) -> DocNode {
        DocNode {
            document_name: doc,
            term_freq: 1,
        }
    }

    fn update_doc(doc: String) -> Result<(), &'static str> {

    }
}