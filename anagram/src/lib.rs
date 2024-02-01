/*
What i'm going to implement
- this is a compare two strings that have same charactor with different orders
 */
use std::str;
use std::collections::HashSet;

/**
 * Main problem is there is a lifetime for this function + result
 * we have to foce to add 'a lifetime to possible_anagrams
 */
pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut result: HashSet<&'a str> = HashSet::new();

    // Prepare the target word for comparison by sorting its characters
    let mut sorted_word = word.to_lowercase().chars().collect::<Vec<char>>();
    sorted_word.sort_unstable();

    for &candidate in possible_anagrams {
        if candidate.to_lowercase() == word.to_lowercase() {
            continue;
        }

        // Sort the characters of the candidate word
        let mut sorted_candidate = candidate.to_lowercase().chars().collect::<Vec<char>>();
        sorted_candidate.sort_unstable();

        if sorted_candidate == sorted_word {
            result.insert(candidate);
        }
    }

    result
}
