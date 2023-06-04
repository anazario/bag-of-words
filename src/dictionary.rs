use std::collections::{hash_map::Keys, HashMap};
use std::ops::AddAssign;

use crate::traits;

pub enum Sort{
    Ascending,
    Descending,
    Alphabetical,
}

pub struct Dictionary{
    contents: HashMap<String, usize>,
}

impl Dictionary{

    pub fn new() -> Self{
        Self{
            contents: HashMap::new()
        }
    }

    pub fn contents(&self) -> &HashMap<String, usize>{
        &self.contents
    }

    pub fn insert(&mut self, key: &str, value: usize) {
        self.contents.insert(key.to_string(), value);
    }

    pub fn get(&self, key: &str) -> Option<&usize> {
        self.contents.get(key)
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.contents.contains_key(key)
    }

    pub fn is_empty(&self) -> bool{
        self.contents.is_empty()
    }

    pub fn keys(&self) -> Keys<String,usize>{
        self.contents.keys()
    }

    pub fn index(&self) -> Dictionary{
        let indexed_dict = self.contents
            .keys()
            .enumerate()
            .map(|(index, key)| (key.clone(), index))
            .collect();

        Dictionary {
            contents: indexed_dict,
        }
    }

    pub fn sorted_words(&self, sort_order: Sort) -> Vec<(&String, &usize)>{

        //convert key/value pairs in dictionary into a Vec<(&String, &usize)>
        let mut sorted_words: Vec<(&String, &usize)> = self
            .contents()
            .iter()
            .collect();

        match sort_order {//choose sorting scheme given by sort_order
            Sort::Ascending    => sorted_words.sort_by(|a, b| a.1.cmp(b.1)),
            Sort::Descending   => sorted_words.sort_by(|a, b| b.1.cmp(a.1)),
            Sort::Alphabetical => sorted_words.sort()
        }

        sorted_words
    }

    pub fn construct_from(&mut self, words: Vec<String>) {
        for word in words {
            self.contents
                .entry(word)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
    }

    pub fn print_index(&self){

        let dict = &self.index();
        let sorted_words = dict.sorted_words(Sort::Ascending);

        for (key, value) in sorted_words {
            println!("{}", format!("{}. {}", value, key));
        }
    }

    pub fn print_top_words(&self, amount: usize){
        //convert dictionary into a vector sorted in descending order
        let mut sorted_entries: Vec<(&String, &usize)> = self.sorted_words(Sort::Descending);

        //define top {amount} entries
        let top_entries = &sorted_entries[..amount];
        println!("Top {amount} words:");//print {amount} entries starting at highest
        for (index, (&ref name, &count)) in top_entries.iter().enumerate() {
            print!("{}. {}: {} \n", index+1, name, count);
        }
    }

    fn merge(&mut self, other: &Dictionary) {

        for word in other.contents().keys().cloned().collect::<Vec<String>>() {
            let val = other.contents().get(&word).unwrap();

            self.contents
                .entry(word)
                .and_modify(|count| *count += *val)
                .or_insert(*val);
        }
    }
}

impl traits::Search<Dictionary> for Dictionary{
    fn search(&self, word: &str) -> Option<Dictionary>{

        let mut dict = Dictionary::new();

        if self.contains_key(&word){
            dict.insert(word, *self.contents.get(word).unwrap());
            return Some(dict);
        }

        None
    }
}

impl AddAssign<&Dictionary> for Dictionary {
    fn add_assign(&mut self, other: &Dictionary) {
        self.merge(other);
    }
}