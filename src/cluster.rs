/*
 * Copyright © 2019-today Peter M. Stahl pemistahl@gmail.com
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

use crate::config::RegExpConfig;
use crate::grapheme::Grapheme;
use crate::unicode_tables::{DECIMAL_NUMBER, WHITE_SPACE, WORD};
use itertools::Itertools;
use lazy_static::lazy_static;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::ops::Range;
use unic_char_range::CharRange;
use unic_ucd_category::GeneralCategory;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GraphemeCluster<'a> {
    graphemes: Vec<Grapheme>,
    config: &'a RegExpConfig,
}

impl<'a> GraphemeCluster<'a> {
    pub(crate) fn from(s: &str, config: &'a RegExpConfig) -> Self {
        Self {
            graphemes: UnicodeSegmentation::graphemes(s, true)
                .flat_map(|it| {
                    let contains_backslash = it.chars().count() == 2 && it.contains('\\');
                    let contains_combining_mark_or_unassigned_chars = it.chars().any(|c| {
                        let category = GeneralCategory::of(c);
                        category.is_mark() || category.is_other()
                    });

                    if contains_backslash || contains_combining_mark_or_unassigned_chars {
                        it.chars()
                            .map(|c| {
                                Grapheme::from(
                                    &c.to_string(),
                                    config.is_capturing_group_enabled,
                                    config.is_output_colorized,
                                )
                            })
                            .collect_vec()
                    } else {
                        vec![Grapheme::from(
                            it,
                            config.is_capturing_group_enabled,
                            config.is_output_colorized,
                        )]
                    }
                })
                .collect_vec(),
            config,
        }
    }

    pub(crate) fn from_graphemes(graphemes: Vec<Grapheme>, config: &'a RegExpConfig) -> Self {
        Self { graphemes, config }
    }

    pub(crate) fn new(grapheme: Grapheme, config: &'a RegExpConfig) -> Self {
        Self {
            graphemes: vec![grapheme],
            config,
        }
    }

    pub(crate) fn convert_to_char_classes(&mut self) {
        let is_digit_converted = self.config.is_digit_converted;
        let is_non_digit_converted = self.config.is_non_digit_converted;
        let is_space_converted = self.config.is_space_converted;
        let is_non_space_converted = self.config.is_non_space_converted;
        let is_word_converted = self.config.is_word_converted;
        let is_non_word_converted = self.config.is_non_word_converted;

        for grapheme in self.graphemes.iter_mut() {
            grapheme.chars = grapheme
                .chars
                .iter()
                .map(|it| {
                    it.chars()
                        .map(|c| {
                            if is_digit_converted && is_digit(c) {
                                "\\d".to_string()
                            } else if is_word_converted && is_word(c) {
                                "\\w".to_string()
                            } else if is_space_converted && is_space(c) {
                                "\\s".to_string()
                            } else if is_non_digit_converted && !is_digit(c) {
                                "\\D".to_string()
                            } else if is_non_word_converted && !is_word(c) {
                                "\\W".to_string()
                            } else if is_non_space_converted && !is_space(c) {
                                "\\S".to_string()
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
        convert_repetitions(self.graphemes(), repetitions.as_mut(), self.config);
        if !repetitions.is_empty() {
            self.graphemes = repetitions;
        }
    }

    pub(crate) fn merge(
        first: &GraphemeCluster,
        second: &GraphemeCluster,
        config: &'a RegExpConfig,
    ) -> Self {
        let mut graphemes = vec![];
        graphemes.extend_from_slice(&first.graphemes);
        graphemes.extend_from_slice(&second.graphemes);
        Self { graphemes, config }
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

fn is_digit(c: char) -> bool {
    lazy_static! {
        static ref VALID_NUMERIC_CHARS: Vec<CharRange> = convert_chars_to_range(DECIMAL_NUMBER);
    }
    VALID_NUMERIC_CHARS.iter().any(|range| range.contains(c))
}

fn is_word(c: char) -> bool {
    lazy_static! {
        static ref VALID_ALPHANUMERIC_CHARS: Vec<CharRange> = convert_chars_to_range(WORD);
    }
    VALID_ALPHANUMERIC_CHARS
        .iter()
        .any(|range| range.contains(c))
}

fn is_space(c: char) -> bool {
    lazy_static! {
        static ref VALID_SPACE_CHARS: Vec<CharRange> = convert_chars_to_range(WHITE_SPACE);
    }
    VALID_SPACE_CHARS.iter().any(|range| range.contains(c))
}

fn convert_repetitions(
    graphemes: &[Grapheme],
    repetitions: &mut Vec<Grapheme>,
    config: &RegExpConfig,
) {
    let repeated_substrings = collect_repeated_substrings(graphemes);
    let ranges_of_repetitions = create_ranges_of_repetitions(repeated_substrings, config);
    let coalesced_repetitions = coalesce_repetitions(ranges_of_repetitions);
    replace_graphemes_with_repetitions(coalesced_repetitions, graphemes, repetitions, config)
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
    config: &RegExpConfig,
) -> Vec<(Range<usize>, Vec<String>)> {
    let mut repetitions = Vec::<(Range<usize>, Vec<String>)>::new();

    for (prefix_length, group) in &repeated_substrings
        .iter()
        .filter(|&(prefix, indices)| {
            indices
                .iter()
                .tuple_windows()
                .all(|(first, second)| (second - first) >= prefix.len())
        })
        .sorted_by_key(|&(prefix, _)| prefix.len())
        .rev()
        .group_by(|&(prefix, _)| prefix.len())
    {
        for (prefix, indices) in group.sorted_by_key(|&(_, indices)| indices[0]) {
            indices
                .iter()
                .map(|it| *it..it + prefix_length)
                .coalesce(|x, y| {
                    if x.end == y.start {
                        Ok(x.start..y.end)
                    } else {
                        Err((x, y))
                    }
                })
                .filter(|range| {
                    let count = ((range.end - range.start) / prefix_length) as u32;
                    count > config.minimum_repetitions
                })
                .for_each(|range| repetitions.push((range, prefix.clone())));
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
    config: &RegExpConfig,
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

        if substr.len() < config.minimum_substring_length as usize {
            continue;
        }

        repetitions.splice(
            range.clone(),
            [Grapheme::new(
                substr.clone(),
                count,
                count,
                config.is_capturing_group_enabled,
                config.is_output_colorized,
            )]
            .iter()
            .cloned(),
        );
    }

    for new_grapheme in repetitions.iter_mut() {
        convert_repetitions(
            &new_grapheme
                .chars
                .iter()
                .map(|it| {
                    Grapheme::from(
                        it,
                        config.is_capturing_group_enabled,
                        config.is_output_colorized,
                    )
                })
                .collect_vec(),
            new_grapheme.repetitions.as_mut(),
            config,
        );
    }
}

fn convert_chars_to_range(chars: &[(char, char)]) -> Vec<CharRange> {
    chars
        .iter()
        .map(|&(start, end)| CharRange::closed(start, end))
        .collect_vec()
}
