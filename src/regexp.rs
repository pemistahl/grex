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

        /*
        if is_non_ascii_char_escaped {
            for cluster in clusters.iter_mut() {
                cluster.escape_non_ascii_chars(is_astral_code_point_converted_to_surrogate);
            }
        }
        */

        clusters
    }
}

impl Display for RegExp {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "^{}$", self.ast.to_string())
    }
}
