use std::collections::HashMap;

use crate::{dictionary::Dictionary, traits::Search};


pub struct Document{
    title: String,
    dictionary: Dictionary,
}

impl Document{
    pub fn new(title: &str) -> Document{
        Document{
            title: title.to_string(),
            dictionary: Dictionary::new(),
        }
    }

    pub fn title(&self) -> &str{
        self.title.as_str()
    }

    pub fn dictionary(&self) -> &Dictionary{
        &self.dictionary
    }

    pub fn build_dictionary(&mut self, words: Vec<String>){
        self.dictionary.construct_from(words);
    }
}

impl Search<usize> for Document{
    fn search(&self, word: &str) -> Option<usize> {
        self.dictionary.get(&word).cloned()
    }
}

