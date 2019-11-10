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

use crate::ast::Expression;
use crate::dfa::DFA;
use itertools::Itertools;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter, Result};

pub struct RegExpBuilder {
    test_cases: Vec<String>,
    escape_non_ascii_chars: bool,
    use_surrogate_pairs: bool,
}

impl RegExpBuilder {
    pub fn from(test_cases: Vec<String>) -> Self {
        Self {
            test_cases,
            escape_non_ascii_chars: false,
            use_surrogate_pairs: false,
        }
    }

    pub fn with_escaped_non_ascii_chars(&mut self, use_surrogate_pairs: bool) -> &mut Self {
        self.escape_non_ascii_chars = true;
        self.use_surrogate_pairs = use_surrogate_pairs;
        self
    }

    pub fn build(&mut self) -> RegExp {
        RegExp::from(
            &mut self.test_cases,
            self.escape_non_ascii_chars,
            self.use_surrogate_pairs,
        )
    }
}

pub struct RegExp {
    ast: Expression,
    escape_non_ascii_chars: bool,
    use_surrogate_pairs: bool,
}

impl RegExp {
    fn from(
        test_cases: &mut Vec<String>,
        escape_non_ascii_chars: bool,
        use_surrogate_pairs: bool,
    ) -> Self {
        test_cases.sort();
        test_cases.dedup();
        test_cases.sort_by(|a, b| match a.len().cmp(&b.len()) {
            Ordering::Equal => a.cmp(&b),
            other => other,
        });
        Self {
            ast: Expression::from(DFA::from(&test_cases)),
            escape_non_ascii_chars,
            use_surrogate_pairs,
        }
    }

    fn escape(&self, use_surrogate_pairs: bool) -> String {
        let surrogate_range = '\u{10000}'..'\u{10ffff}';
        self.ast
            .to_string()
            .chars()
            .map(|it| {
                if it.is_ascii() {
                    it.to_string()
                } else if use_surrogate_pairs && surrogate_range.contains(&it) {
                    self.convert_to_surrogate_pair(it)
                } else {
                    it.escape_unicode().to_string()
                }
            })
            .join("")
    }

    fn convert_to_surrogate_pair(&self, c: char) -> String {
        c.encode_utf16(&mut [0; 2])
            .iter()
            .map(|it| format!("\\u{{{:x}}}", it))
            .join("")
    }
}

impl Display for RegExp {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let regex_str = if self.escape_non_ascii_chars {
            self.escape(self.use_surrogate_pairs)
        } else {
            self.ast.to_string()
        };
        write!(f, "^{}$", regex_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;
    use maplit::hashmap;
    use regex::Regex;
    use std::collections::HashMap;

    #[test]
    fn test_regexp_builder_with_default_options() {
        for (input, expected_output) in default_params() {
            let test_cases = convert_input(input);
            let regexp = RegExpBuilder::from(test_cases).build();
            assert_eq!(regexp.to_string(), expected_output);
        }
    }

    #[test]
    fn test_regexp_builder_with_escaping() {
        for (input, expected_output) in escaped_params() {
            let test_cases = convert_input(input);
            let regexp = RegExpBuilder::from(test_cases)
                .with_escaped_non_ascii_chars(false)
                .build();
            assert_eq!(regexp.to_string(), expected_output);
        }
    }

    #[test]
    fn test_regexp_builder_with_escaping_and_surrogates() {
        for (input, expected_output) in escaped_params_with_surrogates() {
            let test_cases = convert_input(input);
            let regexp = RegExpBuilder::from(test_cases)
                .with_escaped_non_ascii_chars(true)
                .build();
            assert_eq!(regexp.to_string(), expected_output);
        }
    }

    #[test]
    fn ensure_regular_expressions_match_input() {
        for (input, expected_output) in default_params() {
            let re = Regex::new(expected_output).unwrap();
            for input_str in input {
                assert!(
                    re.is_match(input_str),
                    "\"{}\" does not match regex",
                    input_str
                );
            }
        }
    }

    fn convert_input(strs: Vec<&str>) -> Vec<String> {
        strs.iter().map(|&it| it.to_string()).collect_vec()
    }

    fn default_params() -> HashMap<Vec<&'static str>, &'static str> {
        hashmap![
            vec![""] => "^$",
            vec![" "] => "^ $",
            vec!["   "] => "^   $",

            vec!["a", "b"] => "^[ab]$",
            vec!["a", "b", "c"] => "^[a-c]$",
            vec!["a", "c", "d", "e", "f"] => "^[ac-f]$",
            vec!["a", "b", "x", "d", "e"] => "^[abdex]$",
            vec!["a", "b", "x", "de"] => "^de|[abx]$",
            vec!["a", "b", "c", "x", "d", "e"] => "^[a-ex]$",
            vec!["a", "b", "c", "x", "de"] => "^de|[a-cx]$",
            vec!["a", "b", "c", "d", "e", "f", "o", "x", "y", "z"] => "^[a-fox-z]$",
            vec!["a", "b", "d", "e", "f", "o", "x", "y", "z"] => "^[abd-fox-z]$",

            vec!["1", "2"] => "^[12]$",
            vec!["1", "2", "3"] => "^[1-3]$",
            vec!["1", "3", "4", "5", "6"] => "^[13-6]$",
            vec!["1", "2", "8", "4", "5"] => "^[12458]$",
            vec!["1", "2", "8", "45"] => "^45|[128]$",
            vec!["1", "2", "3", "8", "4", "5"] => "^[1-58]$",
            vec!["1", "2", "3", "8", "45"] => "^45|[1-38]$",
            vec!["1", "2", "3", "5", "7", "8", "9"] => "^[1-357-9]$",

            vec!["a", "b", "bc"] => "^bc?|a$",
            vec!["a", "b", "bcd"] => "^b(cd)?|a$",
            vec!["a", "ab", "abc"] => "^a(bc?)?$",
            vec!["ac", "bc"] => "^[ab]c$",
            vec!["ab", "ac"] => "^a[bc]$",
            vec!["abx", "cdx"] => "^(ab|cd)x$",
            vec!["abd", "acd"] => "^a[bc]d$",
            vec!["abc", "abcd"] => "^abcd?$",
            vec!["abc", "abcde"] => "^abc(de)?$",
            vec!["ade", "abcde"] => "^a(bc)?de$",
            vec!["abcxy", "adexy"] => "^a(bc|de)xy$",
            vec!["axy", "abcxy", "adexy"] => "^a((bc)?|de)xy$", // goal: "^a(bc|de)?xy$",

            vec!["abcxy", "abcw", "efgh"] => "^abc(xy|w)|efgh$",
            vec!["abcxy", "efgh", "abcw"] => "^abc(xy|w)|efgh$",
            vec!["efgh", "abcxy", "abcw"] => "^abc(xy|w)|efgh$",

            vec!["abxy", "cxy", "efgh"] => "^(ab|c)xy|efgh$",
            vec!["abxy", "efgh", "cxy"] => "^(ab|c)xy|efgh$",
            vec!["efgh", "abxy", "cxy"] => "^(ab|c)xy|efgh$",

            vec!["a", "ä", "o", "ö", "u", "ü"] => "^[aouäöü]$",
            vec!["y̆", "a", "z"] => "^[az]|y̆$", // goal: "^[az]|y\\u{306}$"

            vec!["a", "b\n", "c"] => "^b\\n|[ac]$",
            vec!["a", "b\\n", "c"] => "^b\\\\n|[ac]$",

            vec!["[a-z]", "(d,e,f)"] => "^\\(d,e,f\\)|\\[a\\-z\\]$",
            vec!["3.5", "4.5", "4,5"] => "^3\\.5|4[,.]5$",

            vec!["I ♥ cake"] => "^I ♥ cake$",
            vec!["I \u{2665} cake"] => "^I ♥ cake$",
            vec!["I \\u{2665} cake"] => "^I \\\\u\\{2665\\} cake$",
            vec!["I \\u2665 cake"] => "^I \\\\u2665 cake$"
        ]
    }

    fn escaped_params() -> HashMap<Vec<&'static str>, &'static str> {
        hashmap![
            vec!["My ♥ and 💩 is yours."] => "^My \\u{2665} and \\u{1f4a9} is yours\\.$"
        ]
    }

    fn escaped_params_with_surrogates() -> HashMap<Vec<&'static str>, &'static str> {
        hashmap![
            vec!["My ♥ and 💩 is yours."] => "^My \\u{2665} and \\u{d83d}\\u{dca9} is yours\\.$"
        ]
    }
}
