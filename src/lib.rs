//! # Bio Read Library
//!
//! The `bio-read` library is an open-source implementation of the Bionic Reading method. Taking inspiration from [text-vide](https://github.com/Gumball12/text-vide/blob/main/HOW.md) and [a bionic reading userscript](https://github.com/yitong2333/Bionic-Reading/blob/main/%E4%BB%BF%E7%94%9F%E9%98%85%E8%AF%BB(Bionic%20Reading)-1.6.user.js), this library ports the Bionic Reading method to Rust and provides a CLI for bio-reading text files right from the terminal.

use colored::Colorize;
use std::collections::HashSet;

/// A BioReader object, allowing for customizing the bio-reading experience.
pub struct BioReader {
    /// The fucntion to emphasize part of a word. Default is bold.
    emphasize: fn(&str) -> String,
    /// The function to de-emphasize part of a word. Default is dimmed.
    de_emphasize: fn(&str) -> String,
    /// Fixation boundary list. A word of length `fixation_boundaries[i]` or less will be emphasized except for the last `i` characters. If the word is longer than `fixation_boundaries.last()`, `fixation_boundaries.len()` will be used (one more than the last boundary).
    fixation_boundaries: Vec<usize>,
    /// Fixation point. Should be in range [1, 5]. Default is 3.
    // fixation_point: usize,
    /// Common words. Only the first letter of these words will be emphasized.
    common_words: HashSet<String>,
}

impl BioReader {
    /// Create a new BioReader object.
    pub fn new() -> Self {
        Self {
            emphasize: |s| s.bold().to_string(),
            de_emphasize: |s| s.dimmed().to_string(),
            fixation_boundaries: Self::fixation_boundaries(3),
            common_words: [
                // https://github.com/yitong2333/Bionic-Reading/blob/acaecfc852f9778a58af89863b80b56bcd4eb637/%E4%BB%BF%E7%94%9F%E9%98%85%E8%AF%BB(Bionic%20Reading)-1.6.user.js#L33-L38
                "the", "and", "in", "on", "at", "by", "with", "about", "against", "between", "into",
                "through", "during", "before", "after", "above", "below", "to", "from", "up",
                "down", "over", "under", "again", "further", "then", "once", "here", "there",
                "when", "where", "why", "how", "all", "any", "both", "each", "few", "more", "most",
                "other", "some",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect(),
        }
    }

    /// Set the function to emphasize part of a word. Default to bold.
    ///
    /// # Example
    ///
    /// See [BioReader::fixation_point] for a comprehensive example.
    pub fn emphasize(mut self, f: fn(&str) -> String) -> Self {
        self.emphasize = f;
        self
    }
    /// Set the function to de-emphasize part of a word. Default to dimmed.
    ///
    /// # Example
    ///
    /// See [BioReader::fixation_point] for a comprehensive example.
    pub fn de_emphasize(mut self, f: fn(&str) -> String) -> Self {
        self.de_emphasize = f;
        self
    }
    /// Set the fixation point. Should be in range \[1, 5\], default to 3.
    ///
    /// # Example
    ///
    /// ```rust
    /// use bio_read::BioReader;
    /// let reader = BioReader::new()
    ///    .emphasize(|s| s.to_uppercase()) // Emphasize by uppercasing
    ///    .de_emphasize(|s| s.to_lowercase()) // De-emphasize by lowercasing
    ///    .fixation_point(1); // Set fixation point to 1
    /// assert_eq!(reader.bio_read_word("hElLo"), "HELlo");
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if `fixation_point` is not in range \[1, 5\].
    pub fn fixation_point(mut self, fixation_point: usize) -> Self {
        assert!(1 <= fixation_point && fixation_point <= 5, "Fixation point should be in range [1, 5], but got {}", fixation_point);
        self.fixation_boundaries = Self::fixation_boundaries(fixation_point);
        self
    }

    /// Do bio-reading on a word.
    pub fn bio_read_word(&self, word: &str) -> String {
        if self.common_words.contains(&word.to_lowercase()) {
            return format!(
                "{}{}",
                (self.emphasize)(&word[..1]),
                (self.de_emphasize)(&word[1..])
            );
        }
        let len = word.len();
        let fixation_boundaries = &self.fixation_boundaries;
        let fixation_length_from_last = fixation_boundaries
            .iter()
            .enumerate() // (index, value), representing (boundary, length)
            .find(|(_, length)| len <= **length) // Find the first boundary that is larger than the word length
            .map_or(fixation_boundaries.len(), |(boundary, _)| boundary); // If not found, use the last boundary
        let fixation_boundary = word.len() - fixation_length_from_last;
        let (prefix, suffix) = word.split_at(fixation_boundary);
        format!(
            "{}{}",
            (self.emphasize)(prefix),
            (self.de_emphasize)(suffix)
        )
    }
    /// Do bio-reading on a piece of text.
    pub fn bio_read_text(&self, text: &str) -> String {
        let mut result = String::with_capacity(text.len());
        let mut word = String::new();
        for c in text.chars() {
            if c.is_alphabetic() {
                // A letter
                word.push(c);
            } else {
                // Not a letter - separator
                if !word.is_empty() {
                    result.push_str(&self.bio_read_word(&word));
                    word.clear();
                }
                result.push(c);
            }
        }
        if !word.is_empty() {
            // In case the text ends with a word
            result.push_str(&self.bio_read_word(&word));
        }
        result
    }

    /// Get the fixation boundaries given a fixation point.
    fn fixation_boundaries(fixation_point: usize) -> Vec<usize> {
        match fixation_point - 1 { // `fixation_point` is 1-based
            // data from https://github.com/Gumball12/text-vide/blob/main/packages/text-vide/src/getFixationLength.ts#L1-L16
            0 => vec![0, 4, 12, 17, 24, 29, 35, 42, 48],
            1 => vec![
                1, 2, 7, 10, 13, 14, 19, 22, 25, 28, 31, 34, 37, 40, 43, 46, 49,
            ],
            2 => vec![
                1, 2, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31, 33, 35, 37, 39, 41, 43,
                45, 47, 49,
            ],
            3 => vec![
                0, 2, 4, 5, 6, 8, 9, 11, 14, 15, 17, 18, 20, 0, 21, 23, 24, 26, 27, 29, 30, 32, 33,
                35, 36, 38, 39, 41, 42, 44, 45, 47, 48,
            ],
            4 => vec![
                0, 2, 3, 5, 6, 7, 8, 10, 11, 12, 14, 15, 17, 19, 20, 21, 23, 24, 25, 26, 28, 29,
                30, 32, 33, 34, 35, 37, 38, 39, 41, 42, 43, 44, 46, 47, 48,
            ],
            _ => vec![0, 4, 12, 17, 24, 29, 35, 42, 48], // Default to 0
        }
    }
}
