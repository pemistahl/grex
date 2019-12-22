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
use crate::grapheme::GraphemeCluster;
use itertools::Itertools;
use std::clone::Clone;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter, Result};

pub struct RegExpBuilder {
    test_cases: Vec<String>,
    is_non_ascii_char_escaped: bool,
    is_astral_code_point_converted_to_surrogate: bool,
    is_repetition_converted: bool,
}

impl RegExpBuilder {
    pub fn from<T: Clone + Into<String>>(test_cases: &[T]) -> Self {
        Self {
            test_cases: test_cases.iter().cloned().map(|it| it.into()).collect_vec(),
            is_non_ascii_char_escaped: false,
            is_astral_code_point_converted_to_surrogate: false,
            is_repetition_converted: false,
        }
    }

    pub fn with_escaped_non_ascii_chars(&mut self, use_surrogate_pairs: bool) -> &mut Self {
        self.is_non_ascii_char_escaped = true;
        self.is_astral_code_point_converted_to_surrogate = use_surrogate_pairs;
        self
    }

    pub fn with_converted_repetitions(&mut self) -> &mut Self {
        self.is_repetition_converted = true;
        self
    }

    pub fn build(&mut self) -> String {
        RegExp::from(
            &mut self.test_cases,
            self.is_non_ascii_char_escaped,
            self.is_astral_code_point_converted_to_surrogate,
            self.is_repetition_converted,
        )
        .to_string()
    }
}

pub(crate) struct RegExp {
    ast: Expression,
}

impl RegExp {
    fn from(
        test_cases: &mut Vec<String>,
        is_non_ascii_char_escaped: bool,
        is_astral_code_point_converted_to_surrogate: bool,
        is_repetition_converted: bool,
    ) -> Self {
        Self::sort(test_cases);
        Self {
            ast: Expression::from(DFA::from(Self::grapheme_clusters(
                &test_cases,
                is_non_ascii_char_escaped,
                is_astral_code_point_converted_to_surrogate,
                is_repetition_converted,
            ))),
        }
    }

    fn sort(test_cases: &mut Vec<String>) {
        test_cases.sort();
        test_cases.dedup();
        test_cases.sort_by(|a, b| match a.len().cmp(&b.len()) {
            Ordering::Equal => a.cmp(&b),
            other => other,
        });
    }

    fn grapheme_clusters(
        test_cases: &[String],
        is_non_ascii_char_escaped: bool,
        is_astral_code_point_converted_to_surrogate: bool,
        is_repetition_converted: bool,
    ) -> Vec<GraphemeCluster> {
        let mut clusters = test_cases
            .iter()
            .map(|it| GraphemeCluster::from(it))
            .collect_vec();

        if is_repetition_converted {
            for cluster in clusters.iter_mut() {
                cluster.convert_repetitions();
            }
        }

        if is_non_ascii_char_escaped {
            for cluster in clusters.iter_mut() {
                cluster.escape_non_ascii_chars(is_astral_code_point_converted_to_surrogate);
            }
        }

        clusters
    }
}

impl Display for RegExp {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "^{}$", self.ast.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use maplit::hashmap;
    use regex::Regex;
    use std::collections::HashMap;

    #[test]
    fn test_regexp_builder_with_default_options() {
        for (test_cases, expected_output) in default_params() {
            let regexp = RegExpBuilder::from(&test_cases).build();
            assert_eq!(regexp, expected_output);
        }
    }

    #[test]
    fn test_regexp_builder_with_converted_repetitions() {
        for (test_cases, expected_output) in repetition_conversion_params() {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_converted_repetitions()
                .build();
            assert_eq!(regexp, expected_output);
        }
    }

    #[test]
    fn test_regexp_builder_with_escaping() {
        for (test_cases, expected_output) in escaping_params() {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_escaped_non_ascii_chars(false)
                .build();
            assert_eq!(regexp, expected_output);
        }
    }

    #[test]
    fn test_regexp_builder_with_escaping_and_surrogates() {
        for (test_cases, expected_output) in escaping_params_with_surrogates() {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_escaped_non_ascii_chars(true)
                .build();
            assert_eq!(regexp, expected_output);
        }
    }

    #[test]
    fn test_regexp_builder_with_converted_repetitions_and_escaping() {
        for (test_cases, expected_output) in repetition_conversion_and_escaping_params() {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_converted_repetitions()
                .with_escaped_non_ascii_chars(false)
                .build();
            assert_eq!(regexp, expected_output);
        }
    }

    #[test]
    fn test_regexp_builder_with_converted_repetitions_and_escaping_and_surrogates() {
        for (test_cases, expected_output) in repetition_conversion_and_escaping_params_with_surrogates()
        {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_converted_repetitions()
                .with_escaped_non_ascii_chars(true)
                .build();
            assert_eq!(regexp, expected_output);
        }
    }

    #[test]
    fn ensure_default_regular_expressions_match_input() {
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

    #[test]
    fn ensure_regular_expressions_with_converted_repetitions_match_input() {
        for (input, expected_output) in repetition_conversion_params() {
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

    fn default_params() -> HashMap<Vec<&'static str>, &'static str> {
        hashmap![
            vec![]      => "^$",
            vec![""]    => "^$",
            vec![" "]   => "^ $",
            vec!["   "] => "^   $",

            vec!["a", "b"]                                         => "^[ab]$",
            vec!["a", "b", "c"]                                    => "^[a-c]$",
            vec!["a", "c", "d", "e", "f"]                          => "^[ac-f]$",
            vec!["a", "b", "x", "d", "e"]                          => "^[abdex]$",
            vec!["a", "b", "x", "de"]                              => "^de|[abx]$",
            vec!["a", "b", "c", "x", "d", "e"]                     => "^[a-ex]$",
            vec!["a", "b", "c", "x", "de"]                         => "^de|[a-cx]$",
            vec!["a", "b", "c", "d", "e", "f", "o", "x", "y", "z"] => "^[a-fox-z]$",
            vec!["a", "b", "d", "e", "f", "o", "x", "y", "z"]      => "^[abd-fox-z]$",

            vec!["1", "2"]                          => "^[12]$",
            vec!["1", "2", "3"]                     => "^[1-3]$",
            vec!["1", "3", "4", "5", "6"]           => "^[13-6]$",
            vec!["1", "2", "8", "4", "5"]           => "^[12458]$",
            vec!["1", "2", "8", "45"]               => "^45|[128]$",
            vec!["1", "2", "3", "8", "4", "5"]      => "^[1-58]$",
            vec!["1", "2", "3", "8", "45"]          => "^45|[1-38]$",
            vec!["1", "2", "3", "5", "7", "8", "9"] => "^[1-357-9]$",

            vec!["a", "b", "bc"]          => "^bc?|a$",
            vec!["a", "b", "bcd"]         => "^b(cd)?|a$",
            vec!["a", "ab", "abc"]        => "^a(bc?)?$",
            vec!["ac", "bc"]              => "^[ab]c$",
            vec!["ab", "ac"]              => "^a[bc]$",
            vec!["abx", "cdx"]            => "^(ab|cd)x$",
            vec!["abd", "acd"]            => "^a[bc]d$",
            vec!["abc", "abcd"]           => "^abcd?$",
            vec!["abc", "abcde"]          => "^abc(de)?$",
            vec!["ade", "abcde"]          => "^a(bc)?de$",
            vec!["abcxy", "adexy"]        => "^a(bc|de)xy$",
            vec!["axy", "abcxy", "adexy"] => "^a((bc)?|de)xy$", // goal: "^a(bc|de)?xy$",

            vec!["abcxy", "abcw", "efgh"] => "^abc(xy|w)|efgh$",
            vec!["abcxy", "efgh", "abcw"] => "^abc(xy|w)|efgh$",
            vec!["efgh", "abcxy", "abcw"] => "^abc(xy|w)|efgh$",

            vec!["abxy", "cxy", "efgh"] => "^(ab|c)xy|efgh$",
            vec!["abxy", "efgh", "cxy"] => "^(ab|c)xy|efgh$",
            vec!["efgh", "abxy", "cxy"] => "^(ab|c)xy|efgh$",

            vec!["a", "ä", "o", "ö", "u", "ü"] => "^[aouäöü]$",
            vec!["y̆", "a", "z"]                => "^[az]|y̆$", // goal: "^[az]|y\\u{306}$"

            vec!["a", "b\n", "c"]  => "^b\\n|[ac]$",
            vec!["a", "b\\n", "c"] => "^b\\\\n|[ac]$",

            vec!["[a-z]", "(d,e,f)"]  => "^\\(d,e,f\\)|\\[a\\-z\\]$",
            vec!["3.5", "4.5", "4,5"] => "^3\\.5|4[,.]5$",

            vec!["I ♥ cake"]         => "^I ♥ cake$",
            vec!["I \u{2665} cake"]  => "^I ♥ cake$",
            vec!["I \\u{2665} cake"] => "^I \\\\u\\{2665\\} cake$",
            vec!["I \\u2665 cake"]   => "^I \\\\u2665 cake$",

            vec!["My ♥ is yours.", "My 💩 is yours."] => "^My [♥💩] is yours\\.$",
        ]
    }

    fn repetition_conversion_params() -> HashMap<Vec<&'static str>, &'static str> {
        hashmap![
            vec![]      => "^$",
            vec![""]    => "^$",
            vec![" "]   => "^ $",
            vec!["   "] => "^ {3}$",

            vec!["a"]               => "^a$",
            vec!["aa"]              => "^a{2}$",
            vec!["aaa"]             => "^a{3}$",
            vec!["a", "aa"]         => "^a{1,2}$",
            vec!["aaa", "a", "aa"]  => "^a{1,3}$",
            vec!["aaaa", "a", "aa"] => "^a{1,2}|a{4}$",

            vec!["ababab"] => "^(ab){3}$",
            vec!["abababa"] => "^(ab){3}a$",
            vec!["aababab"] => "^a(ab){3}$",
            vec!["abababaa"] => "^(ab){3}a{2}$",

            vec!["b", "ba"]                 => "^ba?$",
            vec!["b", "ba", "baa"]          => "^b(a{1,2})?$",
            vec!["b", "ba", "baaa", "baa"]  => "^b(a{1,3})?$",
            vec!["b", "ba", "baaaa", "baa"] => "^b(a{1,2}|a{4})?$",

            vec!["axy", "abcxyxy", "adexy"] => "^a((de)?xy|bc(xy){2})$",

            vec!["xy̆y̆y̆y̆z"]         => "^x(y̆){4}z$",
            vec!["xy̆y̆z", "xy̆y̆y̆z"]  => "^x(y̆){2,3}z$",
            vec!["xy̆y̆z", "xy̆y̆y̆y̆z"] => "^x((y̆){2}|(y̆){4})z$",

            vec!["zyxx", "yxx"] => "^z?yx{2}$",
            vec!["zyxx", "yxx", "yxxx"] => "^zyx{2}|yx{2,3}$",
            vec!["zyxxx", "yxx", "yxxx"] => "^zyx{3}|yx{2,3}$",

            vec!["a", "b\n\n", "c"]                   => "^b\\n{2}|[ac]$",
            vec!["a", "b\nb\nb", "c"]                 => "^(b\\n){2}b|[ac]$",
            vec!["a", "b\nx\nx", "c"]                 => "^b(\\nx){2}|[ac]$",
            vec!["a", "b\n\t\n\t", "c"]               => "^b(\\n\\t){2}|[ac]$",
            vec!["a", "b\n", "b\n\n", "b\n\n\n", "c"] => "^b\\n{1,3}|[ac]$",

            vec!["4.5", "3.55"]                  => "^4\\.5|3\\.5{2}$",
            vec!["4.5", "4.55"]                  => "^4\\.5{1,2}$",
            vec!["4.5", "4.55", "3.5"]           => "^3\\.5|4\\.5{1,2}$",
            vec!["4.5", "44.5", "44.55", "4.55"] => "^4{1,2}\\.5{1,2}$",

            vec!["I ♥♥ cake"]                 => "^I ♥{2} cake$",
            vec!["I ♥ cake", "I ♥♥ cake"]     => "^I ♥{1,2} cake$",
            vec!["I \u{2665}\u{2665} cake"]   => "^I ♥{2} cake$",
            vec!["I \\u{2665} cake"]          => "^I \\\\u\\{26{2}5\\} cake$",
            vec!["I \\u{2665}\\u{2665} cake"] => "^I (\\\\u\\{2665\\}){2} cake$",
            vec!["I \\u2665\\u2665 cake"]     => "^I (\\\\u2665){2} cake$",

            vec!["My ♥♥♥ is yours.", "My 💩💩 is yours."] => "^My (💩{2}|♥{3}) is yours\\.$",
        ]
    }

    fn escaping_params() -> HashMap<Vec<&'static str>, &'static str> {
        hashmap![
            vec!["My ♥ and 💩 is yours."] => "^My \\u{2665} and \\u{1f4a9} is yours\\.$",
            vec!["My ♥ is yours.", "My 💩 is yours."] => "^My (\\u{2665}|\\u{1f4a9}) is yours\\.$", // goal: "^My [\\u{2665}\\u{1f4a9}] is yours\\.$"
        ]
    }

    fn escaping_params_with_surrogates() -> HashMap<Vec<&'static str>, &'static str> {
        hashmap![
            vec!["My ♥ and 💩 is yours."] => "^My \\u{2665} and \\u{d83d}\\u{dca9} is yours\\.$"
        ]
    }

    fn repetition_conversion_and_escaping_params() -> HashMap<Vec<&'static str>, &'static str> {
        hashmap![
            vec!["My ♥♥♥ and 💩💩 is yours."] => "^My \\u{2665}{3} and \\u{1f4a9}{2} is yours\\.$",
            vec!["My ♥♥♥ is yours.", "My 💩💩 is yours."] => "^My (\\u{1f4a9}{2}|\\u{2665}{3}) is yours\\.$",
        ]
    }

    fn repetition_conversion_and_escaping_params_with_surrogates(
    ) -> HashMap<Vec<&'static str>, &'static str> {
        hashmap![
            vec!["My ♥♥♥ and 💩💩 is yours."] => "^My \\u{2665}{3} and (\\u{d83d}\\u{dca9}){2} is yours\\.$",
            vec!["My ♥♥♥ is yours.", "My 💩💩 is yours."] => "^My ((\\u{d83d}\\u{dca9}){2}|\\u{2665}{3}) is yours\\.$",
        ]
    }
}
