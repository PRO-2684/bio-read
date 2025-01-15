//! # Bio Read Library
//!
//! The `bio-read` library is an open-source implementation of the Bionic Reading method. Taking inspiration from [text-vide](https://github.com/Gumball12/text-vide/blob/main/HOW.md), this library ports the Bionic Reading method to Rust and provides a CLI for bio-reading text files right from the terminal.

use anstyle::Style;
use std::{
    collections::VecDeque,
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
}

impl BioReader {
    /// Create a new BioReader object.
    pub fn new() -> Self {
        let bold = Style::new().bold();
        let dim = Style::new().dimmed();
        Self {
            emphasize: [format!("{bold}"), format!("{bold:#}")],
            de_emphasize: [format!("{dim}"), format!("{dim:#}")],
            reverse_fixation_boundaries: Self::reverse_fixation_boundaries(3),
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
    /// assert_eq!(reader.bio_read_text("hello world").unwrap(), "<em>hel</em>lo <em>wor</em>ld");
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
    /// assert_eq!(reader.bio_read_text("hello world").unwrap(), "hel<de>lo</de> wor<de>ld</de>");
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
    /// assert_eq!(reader.bio_read_text("pneumonoultramicroscopicsilicovolcanoconiosis").unwrap(), "**pneumonoultramicroscopicsilicovolcano**coniosis");
    /// let reader = BioReader::new()
    ///     .emphasize(markdownBold.clone(), markdownBold.clone())
    ///     .de_emphasize(empty.clone(), empty.clone())
    ///     .fixation_point(5); // Set fixation point to 5
    /// assert_eq!(reader.bio_read_text("pneumonoultramicroscopicsilicovolcanoconiosis").unwrap(), "**pneumonoult**ramicroscopicsilicovolcanoconiosis");
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
        self.reverse_fixation_boundaries = Self::reverse_fixation_boundaries(fixation_point);
        self
    }

    /// Do bio-reading on `reader` and write the result to `writer`.
    ///
    /// # Performance
    ///
    /// This method guarantees linear time complexity and constant memory usage.
    ///
    /// # Example
    ///
    /// ```rust
    /// use bio_read::BioReader;
    /// use std::io::Write;
    /// let reader = BioReader::new()
    ///     .emphasize(String::from("<em>"), String::from("</em>"))
    ///     .de_emphasize(String::from("<de>"), String::from("</de>"));
    /// let mut output_buffer = Vec::new();
    /// reader.bio_read("hello world".as_bytes(), &mut output_buffer).unwrap();
    /// let output = String::from_utf8(output_buffer).unwrap();
    /// assert_eq!(output, "<em>hel</em><de>lo</de> <em>wor</em><de>ld</de>");
    /// ```
    ///
    /// # See also
    ///
    /// [`BioReader::bio_read_text`]: A simple wrapper around [`BioReader::bio_read`] for processing short strings.
    pub fn bio_read(&self, reader: impl Read, writer: &mut impl Write) -> std::io::Result<()> {
        let mut state = State {
            read: 0,
            written: 0,
        };
        // The buffer size is at most `self.reverse_fixation_boundaries.last().unwrap()`
        let rev_boundaries = &self.reverse_fixation_boundaries;
        let last = rev_boundaries.last().expect("Invalid fixation boundaries");
        let mut buffer = VecDeque::with_capacity(*last);
        // Iterate over the reader
        for c in reader.bytes() {
            let c = c? as char;
            if c.is_ascii_alphabetic() {
                // A letter
                state.read += 1;
                if state.read == 1 {
                    // Start of a word
                    // Write emphasize start
                    writer.write_all(self.emphasize[0].as_bytes())?;
                } else {
                    // Middle of a word
                    self.try_write(writer, &mut buffer, &mut state)?;
                }
                buffer.push_back(c);
            } else {
                // Not a letter - special character
                if state.read != 0 {
                    // End of a word
                    self.try_write(writer, &mut buffer, &mut state)?;
                    // Write emphasize end
                    writer.write_all(self.emphasize[1].as_bytes())?;
                    self.de_emphasize_buffer(writer, &mut buffer)?;
                    state.read = 0;
                    state.written = 0;
                }
                // Write the special character
                writer.write_all(&[c as u8])?;
            }
        }
        // Write the unfinished word
        if state.read > 0 {
            // Write emphasize end
            writer.write_all(self.emphasize[1].as_bytes())?;
            self.de_emphasize_buffer(writer, &mut buffer)?;
        }
        Ok(())
    }
    /// Do bio-reading on a piece of text. This is a simple wrapper for processing short strings. If you intend to process large files or work with streams, use [`BioReader::bio_read`] instead.
    ///
    /// # Example
    ///
    /// ```rust
    /// use bio_read::BioReader;
    /// let reader = BioReader::new()
    ///     .emphasize(String::from("<em>"), String::from("</em>"))
    ///     .de_emphasize(String::from("<de>"), String::from("</de>"));
    /// let output = reader.bio_read_text("hello world").unwrap();
    /// assert_eq!(output, "<em>hel</em><de>lo</de> <em>wor</em><de>ld</de>");
    /// ```
    ///
    /// # See also
    ///
    /// [`BioReader::bio_read`]: Do bio-reading on `reader` and write the result to `writer`.
    pub fn bio_read_text(&self, text: &str) -> Result<String, std::io::Error> {
        let mut output_buffer = Vec::new();
        self.bio_read(text.as_bytes(), &mut output_buffer)?;
        Ok(String::from_utf8(output_buffer).unwrap())
    }

    /// Get the fixation boundaries given a fixation point. A word of length `fixation_boundaries[i]` or less will be emphasized except for the last `i` characters. If the word is longer than `fixation_boundaries.last()`, `fixation_boundaries.len()` will be used (one more than the last boundary).
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
    /// Get the reverse fixation boundaries given a fixation point. A word of length `i` or less will be emphasized except for the last `reverse_fixation_boundaries[i]` characters. If the word is longer than `reverse_fixation_boundaries.len()`, `reverse_fixation_boundaries.last().unwrap() + 1` will be used (one more than the last).
    fn reverse_fixation_boundaries(fixation_point: usize) -> Vec<usize> {
        let fixation_boundaries = Self::fixation_boundaries(fixation_point);
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
    /// Get the fixation length from the last character of a word. A word of length `len` or less will be emphasized except for the last `return_value` characters.
    fn get_fixation_length_from_last(&self, len: usize) -> usize {
        if len < self.reverse_fixation_boundaries.len() {
            self.reverse_fixation_boundaries[len]
        } else {
            *self.reverse_fixation_boundaries.last().unwrap() + 1 // Longer words default to the last plus one
        }
    }
    /// Write the buffer wrapped with de-emphasize tags
    fn de_emphasize_buffer(&self, writer: &mut impl Write, buffer: &mut VecDeque<char>) -> std::io::Result<()> {
        // Skip if the buffer is empty
        if buffer.is_empty() {
            return Ok(());
        }
        // Write de-emphasize start
        writer.write_all(self.de_emphasize[0].as_bytes())?;
        // Write unwritten word characters
        let to_write = buffer.drain(..).map(|c| c as u8).collect::<Vec<_>>();
        writer.write_all(&to_write)?;
        // Write de-emphasize end
        writer.write_all(self.de_emphasize[1].as_bytes())?;
        Ok(())
    }
    /// Try to write a part of the buffer, with respect to the current state
    fn try_write(&self, writer: &mut impl Write, buffer: &mut VecDeque<char>, state: &mut State) -> std::io::Result<()> {
        let fixation_length_from_last = self.get_fixation_length_from_last(state.read);
        // At least `least_emphasize_length` characters should be emphasized
        let least_emphasize_length = state.read - fixation_length_from_last;
        if state.written < least_emphasize_length {
            // Write word[written, least_emphasize_length], which should be buffer[0, least_emphasize_length - written]
            let to_write = buffer.drain(0..least_emphasize_length - state.written).map(|c| c as u8).collect::<Vec<_>>();
            writer.write_all(&to_write)?;
            state.written = least_emphasize_length;
        }
        Ok(())
    }
}

/// Current state. Used internally for [`BioReader::bio_read`].
struct State {
    /// How many letters of the current word have been read.
    read: usize,
    /// How many letters of the current word have been written.
    written: usize,
}
