/*
 * Copyright © 2019 Peter M. Stahl pemistahl@gmail.com
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
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};
use std::ops::Range;
use unicode_segmentation::UnicodeSegmentation;

type Repetition = (Vec<String>, Range<usize>, u32);

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

    pub(crate) fn convert_repetitions(&mut self) {
        let repetitions = self.collect_repeated_substrings();
        let mut sorted_repetitions = self.sort_repetitions(repetitions);
        self.filter_out_overlapping_repetitions(&mut sorted_repetitions);
        self.replace_graphemes_with_repetitions(&sorted_repetitions);
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

    fn collect_repeated_substrings(&self) -> HashMap<Vec<String>, Vec<(Range<usize>, u32)>> {
        let mut repetitions = HashMap::new();

        for n in 1..=self.graphemes.len() / 2 {
            for start in 0..n {
                let mut chunks = vec![];

                for chunk in self.graphemes[start..].chunks(n) {
                    if chunk.len() == n {
                        let substr = chunk.iter().map(|it| it.value()).collect_vec();
                        chunks.push(substr);
                    }
                }

                if chunks.len() < 2 {
                    continue;
                }

                for ((first_idx, first_chunk), (second_idx, second_chunk)) in
                    chunks.iter().enumerate().tuple_windows()
                {
                    if first_chunk != second_chunk {
                        continue;
                    }

                    let start_idx = first_idx * n + start;
                    let end_idx = second_idx * n + start + n;
                    let current_range = start_idx..end_idx;

                    if !repetitions.contains_key(first_chunk) {
                        repetitions.insert(first_chunk.clone(), vec![(current_range, 2)]);
                    } else {
                        let ranges = repetitions.get_mut(first_chunk).unwrap();
                        let mut contains_start = false;

                        for (range, count) in ranges.iter_mut() {
                            if range.contains(&current_range.start) {
                                contains_start = true;
                                range.end = current_range.end;
                                *count += 1;
                            }
                        }

                        if !contains_start {
                            ranges.push((current_range, 2));
                        }
                    }
                }
            }
        }
        repetitions
    }

    fn sort_repetitions(
        &self,
        repetitions: HashMap<Vec<String>, Vec<(Range<usize>, u32)>>,
    ) -> Vec<Option<Repetition>> {
        let mut sorted_repetitions = vec![];

        for substr in repetitions.keys() {
            for (range, count) in repetitions.get(substr).unwrap() {
                sorted_repetitions.push(Some((substr.clone(), range.clone(), *count)));
            }
        }
        sorted_repetitions.sort_by_key(|it| match it {
            Some((_, range, _)) => range.start,
            None => 0,
        });

        sorted_repetitions
    }

    fn filter_out_overlapping_repetitions(&self, sorted_repetitions: &mut Vec<Option<Repetition>>) {
        let mut indices = vec![];
        let mut last_valid_range: Option<&Range<usize>> = None;

        for ((first_idx, first_tup), (second_idx, second_tup)) in
            sorted_repetitions.iter().enumerate().tuple_windows()
        {
            if let (
                Some((first_substr, first_range, first_count)),
                Some((second_substr, second_range, second_count)),
            ) = (first_tup, second_tup)
            {
                if last_valid_range.is_none() {
                    last_valid_range = Some(first_range);
                }

                let valid_range = last_valid_range.unwrap();

                if valid_range.contains(&second_range.start)
                    && valid_range.contains(&second_range.end)
                {
                    indices.push(second_idx);
                } else if valid_range.contains(&second_range.start) {
                    let first_chars_to_remove = (first_count - 1) * first_substr.len() as u32;
                    let second_chars_to_remove = (second_count - 1) * second_substr.len() as u32;

                    if first_chars_to_remove < second_chars_to_remove {
                        indices.push(first_idx);
                        last_valid_range = Some(second_range);
                    } else {
                        indices.push(second_idx);
                    }
                } else {
                    last_valid_range = Some(second_range);
                }
            }
        }

        for i in indices.iter() {
            sorted_repetitions[*i].take();
        }
    }

    fn replace_graphemes_with_repetitions(&mut self, sorted_repetitions: &[Option<Repetition>]) {
        for elem in sorted_repetitions.iter().filter(|it| it.is_some()).rev() {
            if let Some((substr, range, count)) = elem {
                self.graphemes_mut().splice(
                    range.clone(),
                    [Grapheme::new(substr.clone(), *count, *count)]
                        .iter()
                        .cloned(),
                );
            }
        }
    }
}

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
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
