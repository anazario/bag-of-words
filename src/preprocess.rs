use std::fs;
use std::io::{self, BufRead};

fn check_stop_words(word: &str) -> Option<&str>{
    let stop_words: Vec<String> = vec![//stop word collection
        "a", "an", "and", "as", "at", "but", "by", "for", "from", "is", "in", "into", "it", "its",
        "of", "on", "or", "that", "the", "to", "with", "you", "your", "his", "he", "while", "this",
        "their", "they", "are", "these", "also", "our", "was", "be", "can", "such", "we",
        "not", "which", "such",
    ]
        .iter()
        .map(|&word| word.to_string())
        .collect();
    //checks if word is in stop_words, if it is: None, otherwise: Some(word)
    stop_words.contains(&word.to_string()).then(|| None).unwrap_or(Some(word))
}

fn deconstruct_and_clean(line: &str) -> Option<Vec<String>> {

    return if line.is_empty() {
        None
    }
    else {
        let words: Vec<String> = line
            .replace('_', " ")
            .split_whitespace()
            .map(|word| word
                .chars()
                .take_while(|&c| c != '\'')
                .filter(|&c| c.is_alphabetic())
                .collect::<String>()
                .to_lowercase())
            .filter(|word| check_stop_words(word).is_some())
            .collect();

        Some(words)
    }
}

pub fn extract_cleaned_words(file_name: &String, handle: &mut fs::File) -> io::Result<Vec<String>> {
    //initialize empty 'clean words' collection
    let mut clean_words: Vec<String> = Vec::new();

    //create buffer reader for a given file handle
    let reader = io::BufReader::new(handle);

    //add cleaned title words to the 'clean words' collection
    clean_words.extend(deconstruct_and_clean(file_name).unwrap().into_iter());

    //for each line in document clean the words and add to collection
    for line in reader.lines() {
        clean_words.extend(deconstruct_and_clean(&line?).unwrap_or_default().into_iter());
    }
    Ok(clean_words)
}