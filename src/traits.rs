pub trait Search<T>{
    fn search(&self, word: &str) -> Option<T>;
}