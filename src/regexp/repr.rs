/*
 * Copyright © 2019-2020 Peter M. Stahl pemistahl@gmail.com
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
use crate::char::GraphemeCluster;
use crate::color::colorize;
use crate::fsm::DFA;
use crate::regexp::config::RegExpConfig;
use colored::Colorize;
use itertools::Itertools;
use std::cmp::Ordering;
use std::fmt::{Display, Error, Formatter, Result};

pub struct RegExp<'a> {
    ast: Expression<'a>,
    config: &'a RegExpConfig,
}

impl<'a> RegExp<'a> {
    pub(crate) fn from(test_cases: &mut Vec<String>, config: &'a RegExpConfig) -> Self {
        if config.is_case_insensitive_matching() {
            Self::convert_to_lowercase(test_cases);
        }
        Self::sort(test_cases);
        let grapheme_clusters = Self::grapheme_clusters(&test_cases, config);
        let dfa = DFA::from(grapheme_clusters, config);
        let ast = Expression::from(dfa, config);
        Self { ast, config }
    }

    fn convert_to_lowercase(test_cases: &mut Vec<String>) {
        std::mem::replace(
            test_cases,
            test_cases.iter().map(|it| it.to_lowercase()).collect_vec(),
        );
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
        config: &'a RegExpConfig,
    ) -> Vec<GraphemeCluster<'a>> {
        let mut clusters = test_cases
            .iter()
            .map(|it| GraphemeCluster::from(it, config))
            .collect_vec();

        if config.is_char_class_feature_enabled() {
            for cluster in clusters.iter_mut() {
                cluster.convert_to_char_classes();
            }
        }

        if config.is_repetition_converted() {
            for cluster in clusters.iter_mut() {
                cluster.convert_repetitions();
            }
        }

        clusters
    }
}

impl Display for RegExp<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if let [case_insensitive_flag, left_anchor, right_anchor, left_non_capturing_parenthesis, left_capturing_parenthesis, right_parenthesis] =
            &colorize(
                vec!["(?i)", "^", "$", "(?:", "(", ")"],
                self.config.is_output_colorized,
            )[..]
        {
            let flag = if self.config.is_case_insensitive_matching() {
                case_insensitive_flag.clone()
            } else {
                "".clear()
            };

            match self.ast {
                Expression::Alternation(_, _) => {
                    let left_parenthesis = if self.config.is_capturing_group_enabled() {
                        left_capturing_parenthesis
                    } else {
                        left_non_capturing_parenthesis
                    };

                    write!(
                        f,
                        "{}{}{}{}{}{}",
                        flag,
                        left_anchor,
                        left_parenthesis,
                        self.ast.to_string(),
                        right_parenthesis,
                        right_anchor
                    )
                }
                _ => write!(
                    f,
                    "{}{}{}{}",
                    flag,
                    left_anchor,
                    self.ast.to_string(),
                    right_anchor
                ),
            }
        } else {
            Err(Error::default())
        }
    }
}
