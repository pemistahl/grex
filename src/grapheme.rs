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

use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};
use std::ops::Range;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct GraphemeCluster {
    graphemes: Vec<Grapheme>,
}

impl GraphemeCluster {
    pub(crate) fn from(s: &str) -> Self {
        Self {
            graphemes: UnicodeSegmentation::graphemes(s, true)
                .map(|it| Grapheme::from(it))
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

    pub(crate) fn convert_digits(&mut self) {
        for grapheme in self.graphemes.iter_mut() {
            grapheme.chars = grapheme
                .chars
                .iter()
                .map(|it| {
                    it.chars()
                        .map(|c| {
                            if c.is_numeric() {
                                "\\d".to_string()
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
        let repeated_substrings = self.collect_repeated_substrings();
        let repetitions = self.create_ranges_of_repetitions(repeated_substrings);
        let coalesced_repetitions = self.coalesce_repetitions(repetitions);
        self.replace_graphemes_with_repetitions(coalesced_repetitions);
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

    fn collect_repeated_substrings(&self) -> HashMap<Vec<String>, Vec<usize>> {
        let mut map = HashMap::new();

        for i in 0..self.graphemes.len() {
            let suffix = &self.graphemes[i..];
            for j in 1..=self.graphemes.len() / 2 {
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
        &self,
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
        &self,
        repetitions: Vec<(Range<usize>, Vec<String>)>,
    ) -> Vec<(Range<usize>, Vec<String>)> {
        repetitions
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
        &mut self,
        repetitions: Vec<(Range<usize>, Vec<String>)>,
    ) {
        for (range, substr) in repetitions.iter() {
            if range.end > self.graphemes.len() {
                break;
            }

            let count = ((range.end - range.start) / substr.len()) as u32;
            let joined_substr = substr.iter().join("").repeat(count as usize);
            let graphemes_slice = self.graphemes[range.clone()]
                .iter()
                .map(|it| it.value())
                .join("");

            if graphemes_slice != joined_substr {
                break;
            }

            self.graphemes_mut().splice(
                range.clone(),
                [Grapheme::new(substr.clone(), count, count)]
                    .iter()
                    .cloned(),
            );
        }
    }
}

#[derive(Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub(crate) struct Grapheme {
    chars: Vec<String>,
    min: u32,
    max: u32,
}

impl Grapheme {
    pub(crate) fn from(s: &str) -> Self {
        Self {
            chars: vec![s.to_string()],
            min: 1,
            max: 1,
        }
    }

    pub(crate) fn new(chars: Vec<String>, min: u32, max: u32) -> Self {
        Self { chars, min, max }
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
        let value = self.value();

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
