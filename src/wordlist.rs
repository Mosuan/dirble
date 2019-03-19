use std::process::exit;
use std::{
    sync::Arc,
    fs::File,
    io::{self, BufRead, BufReader},
};
use percent_encoding::{utf8_percent_encode, DEFAULT_ENCODE_SET};

// Struct for a UriGenerator, it needs the hostname, the suffix to append, a wordlist and an index into that wordlist
pub struct UriGenerator {
    pub hostname: String,
    suffix: String,
    current_index: usize,
    wordlist: Arc<Vec<String>>,
    step_size: usize
}

// Generates a new UriGenerator given various options
impl UriGenerator {
    pub fn new(mut hostname: String, suffix: String, wordlist: Arc<Vec<String>>, index: u32, step: u32) -> UriGenerator{
        // Remove a trailing / characters from the url if there is one
        if hostname.ends_with("/") {
            hostname.pop();
        }
        
        UriGenerator { 
            hostname: hostname,
            suffix: suffix,
            current_index: index as usize,
            wordlist: wordlist,
            step_size: step as usize
        }
    }
}

// Defines iterating over a UriGenerator
impl Iterator for UriGenerator {
    type Item = (String);

    fn next(&mut self) -> Option<Self::Item> {
        
        // If we're at the end of the wordlist then return None
        if self.current_index >= self.wordlist.len() {
            return None;
        }
        // Concatenate the hostname with the current wordlist item and the suffix, then url encode
        let uri = self.hostname.clone() + "/" + &self.wordlist[self.current_index].clone() + &self.suffix;
        let uri = utf8_percent_encode(&uri, DEFAULT_ENCODE_SET).to_string();

        // Maintain the index into the wordlist
        self.current_index += self.step_size;
        // Return the generated Uri
        Some(uri)

    }
}

// Function used to read in lines from the wordlist file
pub fn lines_from_file(filename: String) -> io::Result<Vec<String>>
{
    let file = File::open(filename.clone())
        .unwrap_or_else(|error| { println!("Opening file \"{}\" failed: {}", filename, error); exit(2) });
    BufReader::new(file).lines().collect()
}