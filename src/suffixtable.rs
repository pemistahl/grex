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

use crate::grapheme::GraphemeCluster;
use itertools::Itertools;

pub(crate) struct SuffixTable {
    graphemes: Vec<String>,
    suffix_indices: Vec<usize>,
    suffix_lengths: Vec<usize>,
    longest_common_prefix_lengths: Vec<u32>,
}

impl SuffixTable {
    pub(crate) fn from(cluster: &GraphemeCluster) -> Self {
        let graphemes = Self::graphemes(cluster);
        let suffixes = Self::suffixes(&graphemes);
        let suffix_indices = Self::suffix_indices(&suffixes);
        let suffix_lengths = Self::suffix_lengths(&suffixes);
        let lcp_lengths = Self::lcp_lengths(&suffixes);

        Self {
            graphemes,
            suffix_indices,
            suffix_lengths,
            longest_common_prefix_lengths: lcp_lengths,
        }
    }

    pub(crate) fn println(&self) {
        println!("idx | lcp | len | suffix");
        println!("------------------------");
        for i in 0..self.longest_common_prefix_lengths.len() {
            let idx = self.suffix_indices[i];
            let lcp = self.longest_common_prefix_lengths[i];
            let length = self.suffix_lengths[i];
            let suffix = self.graphemes[idx..].iter().join("");
            println!("{}   | {}   | {}   | {}", idx, lcp, length, suffix);
        }
    }

    fn graphemes(cluster: &GraphemeCluster) -> Vec<String> {
        cluster
            .graphemes()
            .iter()
            .map(|it| it.value().clone())
            .collect_vec()
    }

    fn suffixes(graphemes: &Vec<String>) -> Vec<(usize, &[String])> {
        let mut suffixes = vec![];
        for i in 0..graphemes.len() {
            suffixes.push((i, &graphemes[i..]));
        }
        suffixes.sort_by_key(|&(idx, suffix)| suffix);
        suffixes
    }

    fn suffix_indices(suffixes: &Vec<(usize, &[String])>) -> Vec<usize> {
        suffixes.iter().map(|&(idx, suffix)| idx).collect_vec()
    }

    fn suffix_lengths(suffixes: &Vec<(usize, &[String])>) -> Vec<usize> {
        suffixes
            .iter()
            .map(|&(idx, suffix)| suffix.len())
            .collect_vec()
    }

    fn lcp_lengths(suffixes: &Vec<(usize, &[String])>) -> Vec<u32> {
        let mut lcp_lengths = vec![0];

        for ((_, first_suffix), (_, second_suffix)) in suffixes.iter().tuple_windows() {
            let (shortest, longest) = if first_suffix.len() < second_suffix.len() {
                (first_suffix, second_suffix)
            } else {
                (second_suffix, first_suffix)
            };

            let mut length = 0;
            for i in 0..shortest.len() {
                if shortest[i] == longest[i] {
                    length += 1;
                } else {
                    break;
                }
            }

            lcp_lengths.push(length);
        }

        lcp_lengths
    }
}
