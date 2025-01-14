//! # Bio Read Library
//!
//! The `bio-read` library is an open-source implementation of the Bionic Reading method. Taking inspiration from [text-vide](https://github.com/Gumball12/text-vide/blob/main/HOW.md) and [a bionic reading userscript](https://github.com/yitong2333/Bionic-Reading/blob/main/%E4%BB%BF%E7%94%9F%E9%98%85%E8%AF%BB(Bionic%20Reading)-1.6.user.js), this library ports the Bionic Reading method to Rust and provides a CLI for bio-reading text files right from the terminal.

use anstyle::Style;
use std::{
    collections::HashSet,
    io::{Read, Write},
};

/// A BioReader object, allowing for customizing the bio-reading experience.
pub struct BioReader {
    /// The strings to be wrapped around the emphasized part of a word.
    emphasize: [String; 2],
    /// The strings to be wrapped around the de-emphasized part of a word.
    de_emphasize: [String; 2],
    /// Reverse map of fixation boundaries for quick lookup. A word of length `i` or less will be emphasized except for the last `reverse_fixation_boundaries[i]` characters. If the word is longer than `reverse_fixation_boundaries.len()`, `reverse_fixation_boundaries.last().unwrap() + 1` will be used (one more than the last).
    reverse_fixation_boundaries: Vec<usize>,
    /// Common words. Only the first letter of these words will be emphasized.
    common_words: HashSet<String>,
}

impl BioReader {
    /// Create a new BioReader object.
    pub fn new() -> Self {
        let bold = Style::new().bold();
        let dim = Style::new().dimmed();
        Self {
            emphasize: [format!("{bold}"), format!("{bold:#}")],
            de_emphasize: [format!("{dim}"), format!("{dim:#}")],
            reverse_fixation_boundaries: Self::reverse_fixation_boundaries(&Self::fixation_boundaries(3)),
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

    /// Set the strings to be wrapped around the emphasized part of a word. Default to bold if environment supports it.
    ///
    /// # Example
    ///
    /// ```rust
    /// use bio_read::BioReader;
    /// let reader = BioReader::new()
    ///     .emphasize(String::from("<em>"), String::from("</em>"))
    ///     .de_emphasize(String::from(""), String::from(""));
    /// assert_eq!(reader.bio_read_text("hello world"), "<em>hel</em>lo <em>wor</em>ld");
    /// ```
    ///
    /// # See also
    ///
    /// Other methods that can be used to customize the [`BioReader`]:
    ///
    /// - [`BioReader::de_emphasize`]
    /// - [`BioReader::fixation_point`]
    pub fn emphasize(mut self, left: String, right: String) -> Self {
        self.emphasize = [left, right];
        self
    }
    /// Set the strings to be wrapped around the de-emphasized part of a word. Default to dimmed if environment supports it.
    ///
    /// # Example
    ///
    /// ```rust
    /// use bio_read::BioReader;
    /// let reader = BioReader::new()
    ///    .emphasize(String::from(""), String::from(""))
    ///     .de_emphasize(String::from("<de>"), String::from("</de>"));
    /// assert_eq!(reader.bio_read_text("hello world"), "hel<de>lo</de> wor<de>ld</de>");
    /// ```
    ///
    /// # See also
    ///
    /// Other methods that can be used to customize the [`BioReader`]:
    ///
    /// - [`BioReader::emphasize`]
    /// - [`BioReader::fixation_point`]
    pub fn de_emphasize(mut self, left: String, right: String) -> Self {
        self.de_emphasize = [left, right];
        self
    }
    /// Set the fixation point. The lower the fixation point, the more characters will be emphasized. The `fixation_point` should be in range \[1, 5\], defaulting to 3 when not specified.
    ///
    /// # Example
    ///
    /// ```rust
    /// use bio_read::BioReader;
    /// let markdownBold = String::from("**");
    /// let empty = String::from("");
    /// let reader = BioReader::new()
    ///     .emphasize(markdownBold.clone(), markdownBold.clone())
    ///     .de_emphasize(empty.clone(), empty.clone())
    ///     .fixation_point(1); // Set fixation point to 1
    /// assert_eq!(reader.bio_read_word("pneumonoultramicroscopicsilicovolcanoconiosis"), "**pneumonoultramicroscopicsilicovolcano**coniosis");
    /// let reader = BioReader::new()
    ///     .emphasize(markdownBold.clone(), markdownBold.clone())
    ///     .de_emphasize(empty.clone(), empty.clone())
    ///     .fixation_point(5); // Set fixation point to 5
    /// assert_eq!(reader.bio_read_word("pneumonoultramicroscopicsilicovolcanoconiosis"), "**pneumonoult**ramicroscopicsilicovolcanoconiosis");
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if `fixation_point` is not in range \[1, 5\].
    ///
    /// # See also
    ///
    /// Other methods that can be used to customize the [`BioReader`]:
    ///
    /// - [`BioReader::emphasize`]
    /// - [`BioReader::de_emphasize`]
    pub fn fixation_point(mut self, fixation_point: usize) -> Self {
        assert!(
            1 <= fixation_point && fixation_point <= 5,
            "Fixation point should be in range [1, 5], but got {}",
            fixation_point
        );
        self.reverse_fixation_boundaries = Self::reverse_fixation_boundaries(&Self::fixation_boundaries(fixation_point));
        self
    }

    /// Do bio-reading on a word.
    ///
    /// # See also
    ///
    /// [`BioReader::bio_read_text`]: Do bio-reading on a piece of text.
    pub fn bio_read_word(&self, word: &str) -> String {
        if self.common_words.contains(&word.to_lowercase()) {
            return format!(
                "{}{}{}{}{}{}",
                self.emphasize[0],
                &word[..1],
                self.emphasize[1],
                self.de_emphasize[0],
                &word[1..],
                self.de_emphasize[1]
            );
        }
        let len = word.len();
        let fixation_length_from_last = if len < self.reverse_fixation_boundaries.len() {
            self.reverse_fixation_boundaries[len]
        } else {
            *self.reverse_fixation_boundaries.last().unwrap() + 1 // Default to the last + 1
        };
        let fixation_boundary = word.len() - fixation_length_from_last;
        let (prefix, suffix) = word.split_at(fixation_boundary);
        format!(
            "{}{}{}{}{}{}",
            self.emphasize[0],
            prefix,
            self.emphasize[1],
            self.de_emphasize[0],
            suffix,
            self.de_emphasize[1]
        )
    }
    /// Do bio-reading on a piece of text.
    ///
    /// # See also
    ///
    /// [`BioReader::bio_read_word`]: Do bio-reading on a word.
    pub fn bio_read_text(&self, text: &str) -> String {
        let mut result = String::with_capacity(text.len());
        let mut word = String::new();
        for c in text.chars() {
            if c.is_ascii_alphabetic() {
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
    /// Do bio-reading on `reader` and write the result to `writer`. Note that this method is not implemented yet. <!-- disregards `common_words` for now. -->
    ///
    /// # Performance
    ///
    /// This method guarantees linear time complexity and constant memory usage.
    pub fn bio_read(&self, reader: &impl Read, writer: &impl Write) {
        todo!()
    }

    /// Get the fixation boundaries given a fixation point.
    fn fixation_boundaries(fixation_point: usize) -> Vec<usize> {
        match fixation_point - 1 {
            // `fixation_point` is 1-based
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
    /// Reverse map given fixation boundaries for quick lookup.
    fn reverse_fixation_boundaries(fixation_boundaries: &[usize]) -> Vec<usize> {
        let last = fixation_boundaries.last().expect("Invalid fixation boundaries");
        let mut fixation = 0;
        let mut result = vec![0; *last + 1];
        for i in 0_usize..=*last {
            result[i] = fixation;
            if i >= fixation_boundaries[fixation] {
                fixation += 1;
            }
        }
        result
    }
}

/// Possible text formats.
enum Format {
    Emphasize,
    DeEmphasize,
    Normal,
}

/// Current state. Used internally for [`BioReader::bio_read`].
struct State {
    /// Current text format.
    format: Format,
    /// How many letters of the current word have been read.
    read: usize,
    /// How many letters of the current word have been written.
    written: usize,
}
