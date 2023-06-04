use std::{fs, io, collections::HashMap};
use std::ops::Index;

use crate::{traits::Search, dictionary::Sort, vocabulary};


enum VectorType{
    Frequency(usize),
    TFIDF(f32),
}

struct BagOfWords<T>{
    indexing: HashMap<String, usize>,
    bag_of_words: HashMap<String, Vec<T>>,
}

impl<T> BagOfWords<T>{
    pub fn new(&self) ->Self{
        Self{
            indexing: HashMap::new(),
            bag_of_words: HashMap::new(),
        }
    }
}

pub fn transform(document_directory : &String) -> Result<HashMap<String, Vec<usize>>, io::Error>{
    let vocabulary = vocabulary::build_vocabulary(&document_directory)?;
    let index = vocabulary.dictionary().index();

    for document in vocabulary.documents(){
        let mut bow = vec![0; vocabulary.word_count()];

        for word in document.dictionary().keys(){
            if let Some(_) = vocabulary.search(word) {
                //let err_message = format!("Word '{}' in {} not found in vocabulary!", word, document.title());
                bow[*index.get(word).unwrap()] += document.search(word).unwrap();
            }
        }

        println!("Document: {}.txt", document.title());
        for e in &bow[..10]{
            println!("{e}");
        }
    }

    Ok(HashMap::new())
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_transform(){
        let document_directory = "../../documents".to_string();
        let _t_vec = transform(&document_directory);
    }
}