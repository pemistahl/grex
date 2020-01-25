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

use crate::regexp::Feature;
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};
use std::ops::Range;
use unic_ucd_category::GeneralCategory;
use unicode_segmentation::UnicodeSegmentation;

const CHARS_TO_ESCAPE: [&str; 14] = [
    "(", ")", "[", "]", "{", "}", "+", "*", "-", ".", "?", "|", "^", "$",
];

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct GraphemeCluster {
    graphemes: Vec<Grapheme>,
}

impl GraphemeCluster {
    pub(crate) fn from(s: &str) -> Self {
        Self {
            graphemes: UnicodeSegmentation::graphemes(s, true)
                .flat_map(|it| {
                    let starts_with_backslash = it.chars().count() == 2 && it.starts_with('\\');
                    let contains_combining_mark =
                        it.chars().any(|c| GeneralCategory::of(c).is_mark());

                    if starts_with_backslash || contains_combining_mark {
                        it.chars()
                            .map(|c| Grapheme::from(&c.to_string()))
                            .collect_vec()
                    } else {
                        vec![Grapheme::from(it)]
                    }
                })
                .collect_vec(),
        }
    }

    pub(crate) fn from_graphemes(graphemes: Vec<Grapheme>) -> Self {
        Self { graphemes }
    }

    pub(crate) fn new(grapheme: Grapheme) -> Self {
        Self {
            graphemes: vec![grapheme],
        }
    }

    pub(crate) fn convert_to_char_classes(&mut self, conversion_features: &[Feature]) {
        let is_digit_converted = conversion_features.contains(&Feature::Digit);
        let is_space_converted = conversion_features.contains(&Feature::Space);
        let is_word_converted = conversion_features.contains(&Feature::Word);

        for grapheme in self.graphemes.iter_mut() {
            grapheme.chars = grapheme
                .chars
                .iter()
                .map(|it| {
                    it.chars()
                        .map(|c| {
                            if is_word_converted && c.is_alphanumeric() {
                                "\\w".to_string()
                            } else if is_digit_converted
                                && c.is_numeric()
                                && GeneralCategory::of(c) != GeneralCategory::LetterNumber
                            {
                                "\\d".to_string()
                            } else if is_space_converted && c.is_whitespace() {
                                "\\s".to_string()
                            } else {
                                c.to_string()
                            }
                        })
                        .join("")
                })
                .collect_vec();
        }
    }

    pub(crate) fn convert_repetitions(&mut self) {
        let mut repetitions = vec![];
        convert_repetitions(self.graphemes(), repetitions.as_mut());
        if !repetitions.is_empty() {
            self.graphemes = repetitions;
        }
    }

    pub(crate) fn merge(first: &GraphemeCluster, second: &GraphemeCluster) -> Self {
        let mut graphemes = vec![];
        graphemes.extend_from_slice(&first.graphemes);
        graphemes.extend_from_slice(&second.graphemes);
        Self { graphemes }
    }

    pub(crate) fn graphemes(&self) -> &Vec<Grapheme> {
        &self.graphemes
    }

    pub(crate) fn graphemes_mut(&mut self) -> &mut Vec<Grapheme> {
        &mut self.graphemes
    }

    pub(crate) fn size(&self) -> usize {
        self.graphemes.len()
    }

    pub(crate) fn char_count(&self, is_non_ascii_char_escaped: bool) -> usize {
        self.graphemes
            .iter()
            .map(|it| it.char_count(is_non_ascii_char_escaped))
            .sum()
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.graphemes.is_empty()
    }
}

#[derive(Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub(crate) struct Grapheme {
    chars: Vec<String>,
    repetitions: Vec<Grapheme>,
    min: u32,
    max: u32,
}

impl Grapheme {
    pub(crate) fn from(s: &str) -> Self {
        Self {
            chars: vec![s.to_string()],
            repetitions: vec![],
            min: 1,
            max: 1,
        }
    }

    pub(crate) fn new(chars: Vec<String>, min: u32, max: u32) -> Self {
        Self {
            chars,
            repetitions: vec![],
            min,
            max,
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

    fn char_count(&self, is_non_ascii_char_escaped: bool) -> usize {
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

        let result = if !is_range && is_repetition && is_single_char {
            format!("{}{{{}}}", value, self.min)
        } else if !is_range && is_repetition && !is_single_char {
            format!("({}){{{}}}", value, self.min)
        } else if is_range && is_single_char {
            format!("{}{{{},{}}}", value, self.min, self.max)
        } else if is_range && !is_single_char {
            format!("({}){{{},{}}}", value, self.min, self.max)
        } else {
            value
        };
        write!(f, "{}", result)
    }
}
fn convert_repetitions(graphemes: &[Grapheme], repetitions: &mut Vec<Grapheme>) {
    let repeated_substrings = collect_repeated_substrings(graphemes);
    let ranges_of_repetitions = create_ranges_of_repetitions(repeated_substrings);
    let coalesced_repetitions = coalesce_repetitions(ranges_of_repetitions);
    replace_graphemes_with_repetitions(coalesced_repetitions, graphemes, repetitions)
}

fn collect_repeated_substrings(graphemes: &[Grapheme]) -> HashMap<Vec<String>, Vec<usize>> {
    let mut map = HashMap::new();

    for i in 0..graphemes.len() {
        let suffix = &graphemes[i..];
        for j in 1..=graphemes.len() / 2 {
            if suffix.len() >= j {
                let prefix = suffix[..j].iter().map(|it| it.value()).collect_vec();
                let indices = map.entry(prefix).or_insert_with(Vec::new);
                indices.push(i);
            }
        }
    }
    map
}

fn create_ranges_of_repetitions(
    repeated_substrings: HashMap<Vec<String>, Vec<usize>>,
) -> Vec<(Range<usize>, Vec<String>)> {
    let mut repetitions = Vec::<(Range<usize>, Vec<String>)>::new();

    for (prefix_length, group) in &repeated_substrings
        .iter()
        .filter(|&(_, indices)| indices.len() > 1)
        .sorted_by_key(|&(prefix, _)| prefix.len())
        .rev()
        .group_by(|&(prefix, _)| prefix.len())
    {
        for (prefix, indices) in group.sorted_by_key(|&(_, indices)| indices[0]) {
            let all_even = indices
                .iter()
                .all(|it| it % prefix_length == 0 || it % 2 == 0);
            let all_odd = indices
                .iter()
                .all(|it| it % prefix_length == 1 || it % 2 == 1);

            if all_even || all_odd {
                let ranges = indices
                    .iter()
                    .cloned()
                    .map(|it| it..it + prefix_length)
                    .coalesce(|x, y| {
                        if x.end == y.start {
                            Ok(x.start..y.end)
                        } else {
                            Err((x, y))
                        }
                    })
                    .filter(|it| (it.end - it.start) > prefix_length)
                    .collect_vec();

                for range in ranges {
                    repetitions.push((range, prefix.clone()));
                }
            }
        }
    }
    repetitions
}

fn coalesce_repetitions(
    ranges_of_repetitions: Vec<(Range<usize>, Vec<String>)>,
) -> Vec<(Range<usize>, Vec<String>)> {
    ranges_of_repetitions
        .iter()
        .sorted_by(|&(first_range, _), &(second_range, _)| {
            match second_range.end.cmp(&first_range.end) {
                Ordering::Equal => first_range.start.cmp(&second_range.start),
                other => other,
            }
        })
        .coalesce(|first_tup, second_tup| {
            let first_range = &first_tup.0;
            let second_range = &second_tup.0;

            if (first_range.contains(&second_range.start)
                || first_range.contains(&second_range.end))
                && second_range.end != first_range.start
            {
                Ok(first_tup)
            } else {
                Err((first_tup, second_tup))
            }
        })
        .map(|(range, substr)| (range.clone(), substr.clone()))
        .collect_vec()
}

fn replace_graphemes_with_repetitions(
    coalesced_repetitions: Vec<(Range<usize>, Vec<String>)>,
    graphemes: &[Grapheme],
    repetitions: &mut Vec<Grapheme>,
) {
    if coalesced_repetitions.is_empty() {
        return;
    }

    for grapheme in graphemes {
        repetitions.push(grapheme.clone());
    }

    for (range, substr) in coalesced_repetitions.iter() {
        if range.end > repetitions.len() {
            break;
        }

        let count = ((range.end - range.start) / substr.len()) as u32;
        let joined_substr = substr.iter().join("").repeat(count as usize);
        let graphemes_slice = repetitions[range.clone()]
            .iter()
            .map(|it| it.value())
            .join("");

        if graphemes_slice != joined_substr {
            break;
        }

        repetitions.splice(
            range.clone(),
            [Grapheme::new(substr.clone(), count, count)]
                .iter()
                .cloned(),
        );
    }

    for new_grapheme in repetitions.iter_mut() {
        convert_repetitions(
            &new_grapheme
                .chars
                .iter()
                .map(|it| Grapheme::from(it))
                .collect_vec(),
            new_grapheme.repetitions.as_mut(),
        );
    }
}
