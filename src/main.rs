mod traits;
mod preprocess;
mod document;
mod vocabulary;
mod dictionary;
mod bow;


fn main() -> std::io::Result<()> {
    let folder_path: String = String::from("../../documents/");
    let vocabulary = vocabulary::build_vocabulary(&folder_path)?;

    let total = 10;
    vocabulary.print_document_list();
    vocabulary.print_top_words(total);

    println!("Unique token count: {}", vocabulary.word_count());

    if let Some((name, count)) = vocabulary.dictionary_contents().iter().max_by_key(|(_, &value)| value){
        println!("{}" , format!("Most common word: {} ({} counts)", name, count));
    }

    Ok(())
}

