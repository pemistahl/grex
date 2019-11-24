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

use crate::grapheme::Grapheme;
use itertools::Itertools;
use maplit::hashmap;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::ops::RangeInclusive;
use unicode_segmentation::UnicodeSegmentation;

pub(crate) fn conflate_repetitions(graphemes: &mut Vec<Grapheme>) {
    let mut ranges = vec![];
    collect_ranges(&mut ranges, graphemes, 0);
    if ranges.is_empty() {
        return;
    }
    ranges = merge_overlapping_ranges(ranges);
    let repetitions = count_repetitions(ranges, graphemes);
    replace_graphemes(graphemes, repetitions);
}

fn replace_graphemes(
    graphemes: &mut Vec<Grapheme>,
    repetitions: HashMap<usize, HashMap<String, usize>>,
) {
    for (start_idx, graphemes_map) in repetitions.iter().sorted_by(|a, b| Ord::cmp(&b.0, &a.0)) {
        for (grapheme, repetition) in graphemes_map {
            let grapheme_len = UnicodeSegmentation::graphemes(&grapheme[..], true).count();
            let end_idx = start_idx + grapheme_len * *repetition;
            let range_to_replace = start_idx..&end_idx;
            let replacement = [Grapheme::new(
                grapheme.clone(),
                *repetition as u32,
                *repetition as u32,
            )];
            graphemes.splice(range_to_replace, replacement.iter().cloned());
        }
    }
}

fn count_repetitions(
    ranges: Vec<RangeInclusive<usize>>,
    graphemes: &[Grapheme],
) -> HashMap<usize, HashMap<String, usize>> {
    let mut counts = HashMap::<usize, HashMap<String, usize>>::new();

    for range in ranges {
        let start = *range.start();
        let end = *range.end();
        let range_length = end - start + 1;
        let slice_length = graphemes[start..=end]
            .iter()
            .map(|it| it.value())
            .unique()
            .count();
        let repetitions = range_length / slice_length;
        let new_end = start + slice_length;
        let slice = graphemes[start..new_end]
            .iter()
            .map(|it| it.value())
            .join("");

        if counts.contains_key(&start) {
            let indices = counts.get_mut(&start).unwrap();
            if indices.contains_key(&slice) {
                *indices.get_mut(&slice).unwrap() += 1;
            } else {
                indices.insert(slice, repetitions);
            }
        } else {
            counts.insert(start, hashmap![slice => repetitions]);
        }
    }
    counts
}

fn merge_overlapping_ranges(mut ranges: Vec<RangeInclusive<usize>>) -> Vec<RangeInclusive<usize>> {
    ranges.sort_by(|a, b| a.start().cmp(b.start()));

    let mut merged_ranges = vec![];
    let mut it = ranges.iter().peekable();

    while let Some(range) = it.next() {
        let mut current = range.clone();

        while let Some(next) = it.peek() {
            if current.contains(next.start()) {
                if next.end() > current.end() {
                    current = *current.start()..=*next.end();
                }
                it.next();
            } else {
                break;
            }
        }
        merged_ranges.push(current);
    }

    let mut indices = vec![];
    for i in 0..merged_ranges.len() - 1 {
        let first_range = &merged_ranges[i];
        let second_range = &merged_ranges[i + 1];
        if first_range.contains(second_range.start())
            && second_range.start() - first_range.start() > 1
        {
            indices.push(i + 1);
        }
    }

    for i in indices {
        merged_ranges.remove(i);
    }

    merged_ranges
}

fn collect_ranges(ranges: &mut Vec<RangeInclusive<usize>>, graphemes: &[Grapheme], shift: i32) {
    let n = graphemes.len() as i32;
    if n == 1 {
        return;
    }

    let nu = n / 2;
    let nv = n - nu;
    let u = graphemes[..nu as usize].to_vec();
    let v = graphemes[(nu as usize)..].to_vec();
    let ru = u.iter().cloned().rev().collect_vec();
    let rv = v.iter().cloned().rev().collect_vec();

    collect_ranges(ranges, &u, shift);
    collect_ranges(ranges, &v, shift + nu);

    let z1 = z_function(&ru);
    let z2 = z_function(
        &vec![v.clone(), vec![Grapheme::from("#")], u.clone()]
            .iter()
            .cloned()
            .concat(),
    );
    let z3 = z_function(
        &vec![ru.clone(), vec![Grapheme::from("#")], rv.clone()]
            .iter()
            .cloned()
            .concat(),
    );
    let z4 = z_function(&v);

    for cntr in 0..n {
        let (l, k1, k2) = if cntr < nu {
            (
                nu - cntr,
                get_z(&z1, (nu - cntr) as usize),
                get_z(&z2, (nv + 1 + cntr) as usize),
            )
        } else {
            (
                cntr - nu + 1,
                get_z(&z3, (nu + 1 + nv - 1 - (cntr - nu)) as usize),
                get_z(&z4, ((cntr - nu) + 1) as usize),
            )
        };

        if k1 + k2 >= l as usize {
            let left = cntr < nu;
            let start = max(1, l - k2 as i32);
            let end = min(l, k1 as i32);

            for l1 in start..=end {
                if left && l1 == l {
                    break;
                }
                let start = shift + (if left { cntr - l1 } else { cntr - l - l1 + 1 });
                let end = start + 2 * l - 1;

                ranges.push((start as usize)..=(end as usize));
            }
        }
    }
}

fn z_function(graphemes: &[Grapheme]) -> Vec<usize> {
    let n = graphemes.len();
    let mut z = vec![0; n];
    let mut l = 0;
    let mut r = 0;

    for i in 1..n {
        if i <= r {
            z[i] = min(r - i + 1, z[i - l]);
        }
        while i + z[i] < n && graphemes[z[i]] == graphemes[i + z[i]] {
            z[i] += 1;
        }
        if i + z[i] - 1 > r {
            l = i;
            r = i + z[i] - 1;
        }
    }
    z
}

fn get_z(z: &[usize], i: usize) -> usize {
    if (0..z.len()).contains(&i) {
        z[i]
    } else {
        0
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use unicode_segmentation::UnicodeSegmentation;

    #[test]
    fn test_conflate_repetitions() {
        for (input, expected_output) in params() {
            let mut graphemes = input_graphemes(input);
            conflate_repetitions(&mut graphemes);
            assert_eq!(
                graphemes, expected_output,
                "assertion failed for input {}",
                input
            );
        }
    }

    fn input_graphemes(s: &str) -> Vec<Grapheme> {
        UnicodeSegmentation::graphemes(s, true)
            .map(|it| Grapheme::from(it))
            .collect_vec()
    }

    fn params() -> HashMap<&'static str, Vec<&'static str>> {
        hashmap![
            "abc" => vec!["a", "b", "c"],
            "aaaba" => vec!["a{3}", "b", "a"],
            "abcabcbczabab" => vec!["(abc){2}", "b", "c", "z", "(ab){2}"],
            "zabababcdd" => vec!["z", "(ab){3}", "c", "d{2}"]
        ]
    }
}
*/
