use std::{cmp, collections::HashMap};

mod data;

#[derive(Clone, Debug)]
pub struct Counter {
    dictionary: HashMap<&'static str, usize>,
    cache: HashMap<String, usize>,
}

impl Counter {
    pub fn new() -> Self {
        let dictionary: HashMap<&'static str, usize> = data::SYLLABLE_DATA
            .iter()
            .map(|&(word, count)| (word, count))
            .collect();

        Self {
            dictionary,
            cache: HashMap::new(),
        }
    }

    /// Count the number of syllables in a word.
    ///
    /// Returns zero in the event of a problem with the word.
    pub fn count(&mut self, word: &str) -> usize {
        let word = word
            .trim_matches(|u: char| u.is_ascii_punctuation())
            .to_ascii_lowercase();

        if word.is_empty() || word.bytes().any(|u| !u.is_ascii_alphabetic()) {
            return 0;
        }

        if let Some(known_count) = self.cached_count(&*word) {
            return known_count;
        }

        let syllable_count = get_syllable_count(&word);
        self.cache.insert(word, syllable_count);
        syllable_count
    }

    fn cached_count(&self, word: &str) -> Option<usize> {
        self.dictionary
            .get(word)
            .or_else(|| self.cache.get(word))
            .copied()
    }
}

impl Default for Counter {
    fn default() -> Self {
        Counter::new()
    }
}

fn get_syllable_count(word: &str) -> usize {
    // Original syllapy count algo copied for reference:
    //
    // syllable_count = 0
    // vowels = "aeiouy"
    // if word[0] in vowels:
    //     syllable_count += 1
    // for index in range(1, len(word)):
    //     if word[index] in vowels and word[index - 1] not in vowels:
    //         syllable_count += 1
    // if word.endswith("e"):
    //     syllable_count -= 1
    // if word.endswith("le") and len(word) > 2 and word[-3] not in vowels:
    //     syllable_count += 1
    // if syllable_count == 0:
    //     syllable_count += 1
    // return syllable_count

    fn is_vowel(u: char) -> bool {
        match u {
            'a' | 'e' | 'i' | 'o' | 'u' | 'y' => true,
            _ => false,
        }
    }

    let characters: Vec<_> = word.chars().collect();

    let mut syllable_count = 0;

    if is_vowel(characters[0]) {
        syllable_count += 1;
    }

    for window in characters.windows(2) {
        let left = window[0];
        let right = window[1];
        if is_vowel(right) && !is_vowel(left) {
            syllable_count += 1;
        }
    }

    if word.ends_with('e') {
        syllable_count -= 1;
    }

    if word.ends_with("le") && word.len() > 2 && !is_vowel(characters[word.len() - 4]) {
        syllable_count += 1;
    }

    cmp::max(1, syllable_count)
}

#[cfg(test)]
mod tests {
    use crate::Counter;

    #[test]
    fn can_initialize() {
        let _ = Counter::new();
    }

    #[test]
    fn can_count() {
        // FIXME: missing test cases for some punctuation
        static TEST_CASES: &[(&'static str, usize)] = &[
            ("dog!!!!!", 1),
            ("d0g", 0),
            ("4dog", 0),
            ("dog123", 0),
            ("", 0),
            (" ", 0),
            ("because", 2),
            ("woman", 2),
            ("international", 5),
            ("ostentatious", 4),
            ("Norway", 2),
            ("norway", 2),
            ("Ohio", 3),
            ("ohio", 3),
        ];

        let mut counter = Counter::new();
        for &(word, expected) in TEST_CASES {
            let actual = counter.count(word);
            assert_eq!(
                actual, expected,
                "{} (actual: {}; expected: {})",
                word, actual, expected
            );
        }
    }
}
