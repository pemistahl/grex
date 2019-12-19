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

    pub(crate) fn new(grapheme: Grapheme) -> Self {
        Self {
            graphemes: vec![grapheme],
        }
    }

    pub(crate) fn convert_repetitions(&mut self) {
        let mut repetitions = HashMap::<String, Vec<(Range<usize>, u32)>>::new();

        for n in 1..=self.graphemes.len() / 2 {
            for start in 0..n {
                let mut chunks = vec![];

                for chunk in self.graphemes[start..].chunks(n) {
                    if chunk.len() == n {
                        let substr = chunk.iter().map(|it| it.value()).join("");
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

        let mut sorted_repetitions = vec![];

        for substr in repetitions.keys() {
            for (range, count) in repetitions.get(substr).unwrap() {
                sorted_repetitions.push(Some((substr, range.clone(), *count)));
            }
        }
        sorted_repetitions.sort_by_key(|it| match it {
            Some((_, range, _)) => range.start,
            None => 0,
        });

        let mut indices = vec![];

        for ((first_idx, first_tup), (second_idx, second_tup)) in
            sorted_repetitions.iter().enumerate().tuple_windows()
        {
            if let (
                Some((first_substr, first_range, first_count)),
                Some((second_substr, second_range, second_count)),
            ) = (first_tup, second_tup)
            {
                if first_range.contains(&second_range.start)
                    && first_range.contains(&second_range.end)
                {
                    indices.push(second_idx);
                } else if first_range.contains(&second_range.start) {
                    let first_chars_to_remove =
                        (first_count - 1) * first_substr.chars().count() as u32;
                    let second_chars_to_remove =
                        (second_count - 1) * second_substr.chars().count() as u32;

                    if indices.contains(&first_idx) || indices.contains(&second_idx) {
                        continue;
                    }

                    if first_chars_to_remove < second_chars_to_remove {
                        indices.push(first_idx);
                    } else {
                        indices.push(second_idx);
                    }
                }
            }
        }

        for i in indices.iter() {
            sorted_repetitions[*i].take();
        }

        for elem in sorted_repetitions.iter().filter(|it| it.is_some()).rev() {
            if let Some((substr, range, count)) = elem {
                self.graphemes_mut().splice(
                    range.clone(),
                    [Grapheme::new(substr.to_string(), *count, *count)]
                        .iter()
                        .cloned(),
                );
            }
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

    pub(crate) fn char_count(&self) -> usize {
        self.graphemes.iter().map(|it| it.char_count()).sum()
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.graphemes.is_empty()
    }
}

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Grapheme {
    value: String,
    min: u32,
    max: u32,
}

impl Grapheme {
    pub(crate) fn from(s: &str) -> Self {
        Self {
            value: s.to_string(),
            min: 1,
            max: 1,
        }
    }

    pub(crate) fn new(value: String, min: u32, max: u32) -> Self {
        Self { value, min, max }
    }

    pub(crate) fn value(&self) -> &String {
        &self.value
    }

    pub(crate) fn minimum(&self) -> u32 {
        self.min
    }

    pub(crate) fn maximum(&self) -> u32 {
        self.max
    }

    pub(crate) fn char_count(&self) -> usize {
        self.value.chars().count()
    }
}

impl Display for Grapheme {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let is_single_char = self.value.chars().count() == 1;
        let is_range = self.min < self.max;
        let is_repetition = self.min > 1;

        let result = if !is_range && is_repetition && is_single_char {
            format!("{}{{{}}}", self.value, self.min)
        } else if !is_range && is_repetition && !is_single_char {
            format!("({}){{{}}}", self.value, self.min)
        } else if is_range && is_single_char {
            format!("{}{{{},{}}}", self.value, self.min, self.max)
        } else if is_range && !is_single_char {
            format!("({}){{{},{}}}", self.value, self.min, self.max)
        } else {
            self.value.clone()
        };
        write!(f, "{}", result)
    }
}
