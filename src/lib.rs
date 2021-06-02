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

    pub fn add(&mut self, word: String, doc: String) -> Result<(), &'static str> {
        
    }

    pub fn remove(&mut self, word: String) -> Result<(), &'static str> {
        
    }

    pub fn get_doc_freq(&self, word: String) -> Result<i32, &'static str> {
        
    }

    pub fn get_term_freq(&self, word: String, doc: String) -> Result<i32, &'static str> {       
        
    }

    /// Get the hash code for the passed in word. Possibly sum up all of the
    /// ascii/utf-8 values and do a modulus operation as a simple version or 
    /// implement a more professional version
    fn hash(&self, word: String) -> u32 {
        
    }

    /// Removal function -> may not be really needed due to the automatic
    /// cleanup provided. But for practice's sake, keep it present, and also
    /// implement the Deref trait for the struct
    pub fn destroy_map(self) -> Result<(), &'static str> {

    }
}

/// Implementing the drop trait for the hashmap to allow the direct control
/// over the memory operations for the sake of practice
/// This will be called by the destroy_map function
impl Drop for Hashmap {
    fn drop(&mut self) {

    }
}

struct WordNode {
    word: String,
    doc_freq: i32,
    documents: Vec<DocNode>,
}

impl WordNode {
    /// For when a new word is being added to the hashmap. A new node is made
    /// holding the word and sets all the fields to the default levels along
    /// with a new DocNode with the specified document saved
    pub fn new(word: String, doc: String) -> Box<WordNode> {

    }

    fn update_word(word: String, doc: String) -> Result<(), &'static str> {
        
    }
}

struct DocNode {
    document_name: String,
    occurrences: u32,
}

impl DocNode {
    pub fn new(doc: String) -> Box<WordNode> {

    }

    fn update_doc(doc: String) -> Result<(), &'static str> {

    }
}