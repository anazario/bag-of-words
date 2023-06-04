use std::collections::HashMap;
use std::{fs, io};

use crate::{preprocess, dictionary::Dictionary, document::Document, traits};

pub struct Vocabulary {
    dictionary: Dictionary,
    documents: Vec<Document>,
}

impl Vocabulary {
    //public functions
    pub fn new() -> Self {
        Vocabulary {
            dictionary: Dictionary::new(),
            documents: Vec::new(),
         }
    }

    pub fn dictionary(&self) -> &Dictionary{
        &self.dictionary
    }

    pub fn dictionary_contents(&self) -> &HashMap<String, usize>{
        &self.dictionary.contents()
    }

    pub fn documents(&self) -> &Vec<Document>{
        &self.documents
    }

    pub fn word_count(&self) -> usize{
        self.dictionary.contents().len()
    }

    pub fn print_top_words(&self, amount: usize){
        self.dictionary.print_top_words(amount);
    }

    pub fn print_document_list(&self){
        println!("Documents processed (total: {}): ", self.documents.len());
        for document in &self.documents{
            println!("{}",document.title().replace('_', " "));
        }
    }

    //private
    fn add_document(&mut self, document: Document){
        self.dictionary += &document.dictionary();
        self.documents.push(document);
    }
}

impl traits::Search<Dictionary> for Vocabulary{
    fn search(&self, word: &str) -> Option<Dictionary> {
        let mut title_list: Dictionary = Dictionary::new();

        for document in &self.documents {
            if document.dictionary().contains_key(word) {
                title_list.insert(document.title(),*document.dictionary().get(word).unwrap());
            }
        }

        if title_list.is_empty() {
            return None;
        }
        Some(title_list)
    }
}

pub fn build_vocabulary(document_directory : &String) -> Result<Vocabulary, io::Error> {

    let mut vocabulary = Vocabulary::new();

    let entries = fs::read_dir(document_directory)?;

    //loop over all the files in entries
    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        //check if the file extension is .txt
        if path.extension().unwrap_or_default() == "txt"{
            let file_name = path.file_name()
                                      .and_then(|n| n.to_str())
                                      .unwrap()
                                      .split(".txt")
                                      .collect::<String>();

            //create file handle for current txt document
            let mut file_handle = fs::File::open(&path)?;

            //add words in document to the dictionary, repeated words are counted
            let mut document = Document::new(&file_name);
            document.build_dictionary(preprocess::extract_cleaned_words(&file_name, &mut file_handle)?);
            vocabulary.add_document(document);
        }
    }
    if vocabulary.dictionary_contents().is_empty(){
        panic!("No txt files were found in {document_directory}");
    }
    else {
        Ok(vocabulary)
    }
}