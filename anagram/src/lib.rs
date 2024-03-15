use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {

    let mut result: HashSet<&'a str> = HashSet::new();

    possible_anagrams.iter().for_each(|&anagram| {
        if anagram.len() == word.len() && anagram.to_lowercase() != word.to_lowercase() {
            let mut word_chars: Vec<char> = word.to_lowercase().chars().collect();
            let mut anagram_chars: Vec<char> = anagram.to_lowercase().chars().collect();
            word_chars.sort();
            anagram_chars.sort();
            if word_chars == anagram_chars {
                result.insert(anagram);
            }
        }
    });
    return result;
}
