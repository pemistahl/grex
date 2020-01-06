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

/// This struct builds regular expressions from user-provided test cases.
pub struct RegExpBuilder {
    test_cases: Vec<String>,
    is_non_ascii_char_escaped: bool,
    is_astral_code_point_converted_to_surrogate: bool,
    is_repetition_converted: bool,
}

impl RegExpBuilder {
    /// Specifies the test cases to build the regular expression from.
    /// The test cases may be passed as a shared slice `&[T]` where `T` may represent
    /// anything that can be converted to a `String`.
    ///
    /// **Note:** The test cases do not have to be sorted because `RegExpBuilder` will
    /// sort them for you.
    pub fn from<T: Clone + Into<String>>(test_cases: &[T]) -> Self {
        Self {
            test_cases: test_cases.iter().cloned().map(|it| it.into()).collect_vec(),
            is_non_ascii_char_escaped: false,
            is_astral_code_point_converted_to_surrogate: false,
            is_repetition_converted: false,
        }
    }

    /// Tells `RegExpBuilder` to convert non-ASCII characters to unicode escape sequences.
    /// The parameter `use_surrogate_pairs` specifies whether to convert astral code planes
    /// (range `U+010000` to `U+10FFFF`) to surrogate pairs.
    pub fn with_escaped_non_ascii_chars(&mut self, use_surrogate_pairs: bool) -> &mut Self {
        self.is_non_ascii_char_escaped = true;
        self.is_astral_code_point_converted_to_surrogate = use_surrogate_pairs;
        self
    }

    /// Tells `RegExpBuilder` to detect repeated non-overlapping substrings and to convert
    /// them to `{min,max}` quantifier notation.
    pub fn with_converted_repetitions(&mut self) -> &mut Self {
        self.is_repetition_converted = true;
        self
    }

    /// Builds the actual regular expression using the previously given settings.
    /// Every generated regular expression is surrounded by the anchors `^` and `$`
    /// so that substrings not being part of the test cases are not matched accidentally.
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
            ast: Expression::from(
                DFA::from(Self::grapheme_clusters(
                    &test_cases,
                    is_repetition_converted,
                )),
                is_non_ascii_char_escaped,
                is_astral_code_point_converted_to_surrogate,
            ),
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

        clusters
    }
}

impl Display for RegExp {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self.ast {
            Expression::Alternation(_) => write!(f, "^({})$", self.ast.to_string()),
            _ => write!(f, "^{}$", self.ast.to_string()),
        }
    }
}
