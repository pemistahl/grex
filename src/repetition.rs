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
use std::cmp::{max, min};
use std::ops::Range;

pub(crate) fn conflate_repetitions(graphemes: &[Grapheme]) -> Vec<Grapheme> {
    let mut ranges = vec![];
    collect_ranges(&mut ranges, &graphemes, 0);
    if ranges.is_empty() {
        return graphemes.to_owned();
    }
    ranges = convert_ranges(&ranges, &graphemes);
    convert_graphemes(graphemes, ranges)
}

fn convert_graphemes(graphemes: &[Grapheme], ranges: Vec<Range<usize>>) -> Vec<Grapheme> {
    ranges
        .iter()
        .map(|range| {
            graphemes[range.clone()]
                .iter()
                .map(|grapheme| grapheme.value().clone())
                .join("")
        })
        .coalesce(|x, y| {
            if *x.split("`__`").collect_vec().last().unwrap() == y {
                Ok(format!("{}`__`{}", x, y))
            } else {
                Err((x, y))
            }
        })
        .map(|it| {
            let parts = it.split("`__`").collect_vec();
            let repetition = parts.len() as u32;
            Grapheme::new(parts.first().unwrap().to_string(), repetition, repetition)
        })
        .collect_vec()
}

fn convert_ranges(ranges: &[Range<usize>], graphemes: &[Grapheme]) -> Vec<Range<usize>> {
    let mut optional_ranges = ranges.iter().cloned().map(|it| Some(it)).collect_vec();
    let mut indices = vec![];

    for i in 0..optional_ranges.len() - 1 {
        let first_range = optional_ranges[i].as_ref().unwrap();
        let second_range = optional_ranges[i + 1].as_ref().unwrap();

        let first_start = first_range.start;
        let first_end = first_range.end;

        let second_start = second_range.start;
        let second_end = second_range.end;

        if first_start == second_start {
            if first_end > second_end {
                indices.push(i);
            } else if first_end < second_end {
                indices.push(i + 1);
            }
        }
    }

    for i in indices.iter() {
        optional_ranges[*i].take();
    }

    indices.clear();

    optional_ranges = optional_ranges
        .iter()
        .cloned()
        .filter(|it| it.is_some())
        .collect_vec();

    let mut split_ranges = vec![];

    for range in optional_ranges.iter() {
        let old_start = range.as_ref().unwrap().start;
        let old_end = range.as_ref().unwrap().end;
        let new_end = old_start + (old_end - old_start) / 2;

        split_ranges.push(Some(old_start..new_end));
        split_ranges.push(Some(new_end..old_end));
    }

    split_ranges.sort_by(|a, b| a.as_ref().unwrap().start.cmp(&b.as_ref().unwrap().start));
    split_ranges.dedup();

    for i in 0..split_ranges.len() - 1 {
        let first_range = split_ranges[i].as_ref().unwrap();
        let second_range = split_ranges[i + 1].as_ref().unwrap();

        let first_start = first_range.start;
        let second_start = second_range.start;

        if first_start == second_start {
            let first_graphemes = graphemes[first_range.clone()]
                .iter()
                .map(|it| it.value())
                .collect_vec();
            let second_graphemes = graphemes[second_range.clone()]
                .iter()
                .map(|it| it.value())
                .collect_vec();

            let second_contains_first = first_graphemes
                .iter()
                .all(|it| second_graphemes.contains(it));

            if second_contains_first {
                indices.push(i);
            } else {
                indices.push(i + 1);
            }
        }
    }

    for i in indices.iter() {
        split_ranges[*i].take();
    }

    indices.clear();

    split_ranges = split_ranges
        .iter()
        .cloned()
        .filter(|it| it.is_some())
        .collect_vec();

    let mut current_end = split_ranges.first().unwrap().as_ref().unwrap().end;
    for i in 1..split_ranges.len() {
        let current_range = &split_ranges[i];
        if current_range.as_ref().unwrap().start == current_end {
            current_end = current_range.as_ref().unwrap().end;
        } else {
            indices.push(i);
        }
    }

    for i in indices.iter() {
        split_ranges[*i].take();
    }

    let mut new_ranges = split_ranges
        .iter()
        .cloned()
        .filter(|it| it.is_some())
        .map(|it| it.unwrap())
        .collect_vec();

    let first_start = new_ranges.first().unwrap().start;
    let last_end = new_ranges.last().unwrap().end;

    if first_start > 0 {
        new_ranges.insert(0, 0..first_start);
    }
    if last_end < graphemes.len() {
        new_ranges.push(last_end..graphemes.len());
    }

    new_ranges
}

fn collect_ranges(ranges: &mut Vec<Range<usize>>, graphemes: &[Grapheme], shift: i32) {
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

                ranges.push((start as usize)..((end + 1) as usize));
            }
        }
    }

    ranges.sort_by(|a, b| a.start.cmp(&b.start));
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
