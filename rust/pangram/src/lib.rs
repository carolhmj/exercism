/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    (b'a'..=b'z').map(char::from).all(|c| sentence.to_lowercase().find(c).is_some())
}
