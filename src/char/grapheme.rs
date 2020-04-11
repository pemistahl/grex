/*
 * Copyright © 2019-2020 Peter M. Stahl pemistahl@gmail.com
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either expressed or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use crate::color::colorize;
use crate::regexp::RegExpConfig;
use itertools::Itertools;
use std::fmt::{Display, Error, Formatter, Result};

const CHARS_TO_ESCAPE: [&str; 14] = [
    "(", ")", "[", "]", "{", "}", "+", "*", "-", ".", "?", "|", "^", "$",
];

#[derive(Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct Grapheme {
    pub(crate) chars: Vec<String>,
    pub(crate) repetitions: Vec<Grapheme>,
    min: u32,
    max: u32,
    config: RegExpConfig,
}

impl Grapheme {
    pub(crate) fn from(s: &str, config: &RegExpConfig) -> Self {
        Self {
            chars: vec![s.to_string()],
            repetitions: vec![],
            min: 1,
            max: 1,
            config: config.clone(),
        }
    }

    pub(crate) fn new(chars: Vec<String>, min: u32, max: u32, config: &RegExpConfig) -> Self {
        Self {
            chars,
            repetitions: vec![],
            min,
            max,
            config: config.clone(),
        }
    }

    pub(crate) fn value(&self) -> String {
        self.chars.join("")
    }

    pub(crate) fn chars(&self) -> &Vec<String> {
        &self.chars
    }

    pub(crate) fn chars_mut(&mut self) -> &mut Vec<String> {
        &mut self.chars
    }

    pub(crate) fn has_repetitions(&self) -> bool {
        !self.repetitions.is_empty()
    }

    pub(crate) fn repetitions_mut(&mut self) -> &mut Vec<Grapheme> {
        &mut self.repetitions
    }

    pub(crate) fn minimum(&self) -> u32 {
        self.min
    }

    pub(crate) fn maximum(&self) -> u32 {
        self.max
    }

    pub(crate) fn char_count(&self, is_non_ascii_char_escaped: bool) -> usize {
        if is_non_ascii_char_escaped {
            self.chars
                .iter()
                .map(|it| it.chars().map(|c| self.escape(c, false)).join(""))
                .join("")
                .chars()
                .count()
        } else {
            self.chars.iter().map(|it| it.chars().count()).sum()
        }
    }

    pub(crate) fn escape_non_ascii_chars(&mut self, use_surrogate_pairs: bool) {
        self.chars = self
            .chars
            .iter()
            .map(|it| {
                it.chars()
                    .map(|c| self.escape(c, use_surrogate_pairs))
                    .join("")
            })
            .collect_vec();
    }

    pub(crate) fn escape_regexp_symbols(
        &mut self,
        is_non_ascii_char_escaped: bool,
        is_astral_code_point_converted_to_surrogate: bool,
    ) {
        let characters = self.chars_mut();

        #[allow(clippy::needless_range_loop)]
        for i in 0..characters.len() {
            let mut character = characters[i].clone();

            for char_to_escape in CHARS_TO_ESCAPE.iter() {
                character =
                    character.replace(char_to_escape, &format!("{}{}", "\\", char_to_escape));
            }

            character = character
                .replace("\n", "\\n")
                .replace("\r", "\\r")
                .replace("\t", "\\t");

            if character == "\\" {
                character = "\\\\".to_string();
            }

            characters[i] = character;
        }

        if is_non_ascii_char_escaped {
            self.escape_non_ascii_chars(is_astral_code_point_converted_to_surrogate);
        }
    }

    fn escape(&self, c: char, use_surrogate_pairs: bool) -> String {
        if c.is_ascii() {
            c.to_string()
        } else if use_surrogate_pairs && ('\u{10000}'..'\u{10ffff}').contains(&c) {
            self.convert_to_surrogate_pair(c)
        } else {
            c.escape_unicode().to_string()
        }
    }

    fn convert_to_surrogate_pair(&self, c: char) -> String {
        c.encode_utf16(&mut [0; 2])
            .iter()
            .map(|it| format!("\\u{{{:x}}}", it))
            .join("")
    }
}

impl Display for Grapheme {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let is_single_char = self.char_count(false) == 1
            || (self.chars.len() == 1 && self.chars[0].matches('\\').count() == 1);
        let is_range = self.min < self.max;
        let is_repetition = self.min > 1;
        let value = if self.repetitions.is_empty() {
            self.value()
        } else {
            self.repetitions.iter().map(|it| it.to_string()).join("")
        };

        if let [left_non_capturing_parenthesis, left_capturing_parenthesis, right_parenthesis, left_brace, right_brace, comma, min, max, colored_value] =
            &colorize(
                vec![
                    "(?:",
                    "(",
                    ")",
                    "{",
                    "}",
                    ",",
                    self.min.to_string().as_str(),
                    self.max.to_string().as_str(),
                    value.as_str(),
                ],
                self.config.is_output_colorized,
            )[..]
        {
            let left_parenthesis = if self.config.is_capturing_group_enabled() {
                left_capturing_parenthesis
            } else {
                left_non_capturing_parenthesis
            };

            if !is_range && is_repetition && is_single_char {
                write!(f, "{}{}{}{}", colored_value, left_brace, min, right_brace)
            } else if !is_range && is_repetition && !is_single_char {
                write!(
                    f,
                    "{}{}{}{}{}{}",
                    left_parenthesis,
                    colored_value,
                    right_parenthesis,
                    left_brace,
                    min,
                    right_brace
                )
            } else if is_range && is_single_char {
                write!(
                    f,
                    "{}{}{}{}{}{}",
                    colored_value, left_brace, min, comma, max, right_brace
                )
            } else if is_range && !is_single_char {
                write!(
                    f,
                    "{}{}{}{}{}{}{}{}",
                    left_parenthesis,
                    colored_value,
                    right_parenthesis,
                    left_brace,
                    min,
                    comma,
                    max,
                    right_brace
                )
            } else {
                write!(f, "{}", colored_value)
            }
        } else {
            Err(Error::default())
        }
    }
}
