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

use crate::cluster::GraphemeCluster;
use crate::component::Component;
use crate::config::RegExpConfig;
use crate::dfa::Dfa;
use crate::expression::Expression;
use itertools::Itertools;
use regex::Regex;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter, Result};

pub struct RegExp<'a> {
    ast: Expression<'a>,
    config: &'a RegExpConfig,
}

impl<'a> RegExp<'a> {
    pub(crate) fn from(test_cases: &'a mut Vec<String>, config: &'a RegExpConfig) -> Self {
        if config.is_case_insensitive_matching {
            Self::convert_for_case_insensitive_matching(test_cases);
        }
        Self::sort(test_cases);
        let grapheme_clusters = Self::grapheme_clusters(test_cases, config);
        let mut dfa = Dfa::from(&grapheme_clusters, true, config);
        let mut ast = Expression::from(dfa, config);

        if config.is_start_anchor_disabled && config.is_end_anchor_disabled {
            let mut regex = Self::convert_expr_to_regex(&ast, config);

            if config.is_verbose_mode_enabled {
                // Remove line breaks before checking matches, otherwise check will be incorrect.
                regex = Regex::new(&regex.to_string().replace('\n', "")).unwrap();
            }

            if !Self::is_each_test_case_matched_after_rotating_alternations(
                &regex, &mut ast, test_cases,
            ) {
                dfa = Dfa::from(&grapheme_clusters, false, config);
                ast = Expression::from(dfa, config);
                regex = Self::convert_expr_to_regex(&ast, config);

                if !Self::regex_matches_all_test_cases(&regex, test_cases) {
                    let mut exprs = vec![];
                    for cluster in grapheme_clusters {
                        let literal = Expression::new_literal(cluster, config);
                        exprs.push(literal);
                    }
                    ast = Expression::new_alternation(exprs, config);
                }
            }
        }

        Self { ast, config }
    }

    fn convert_for_case_insensitive_matching(test_cases: &mut Vec<String>) {
        // Convert only those test cases to lowercase if
        // they keep their original number of characters.
        // Otherwise, "İ" -> "i\u{307}" would not match "İ".
        *test_cases = test_cases
            .iter()
            .map(|it| {
                let lower_test_case = it.to_lowercase();
                if lower_test_case.chars().count() == it.chars().count() {
                    lower_test_case
                } else {
                    it.to_string()
                }
            })
            .collect_vec();
    }

    fn convert_expr_to_regex(expr: &Expression, config: &RegExpConfig) -> Regex {
        if config.is_output_colorized {
            let color_replace_regex = Regex::new("\u{1b}\\[(?:\\d+;\\d+|0)m").unwrap();
            Regex::new(&color_replace_regex.replace_all(&expr.to_string(), "")).unwrap()
        } else {
            Regex::new(&expr.to_string()).unwrap()
        }
    }

    fn regex_matches_all_test_cases(regex: &Regex, test_cases: &[String]) -> bool {
        test_cases
            .iter()
            .all(|test_case| regex.find_iter(test_case).count() == 1)
    }

    fn sort(test_cases: &mut Vec<String>) {
        test_cases.sort();
        test_cases.dedup();
        test_cases.sort_by(|a, b| match a.len().cmp(&b.len()) {
            Ordering::Equal => a.cmp(b),
            other => other,
        });
    }

    fn grapheme_clusters(
        test_cases: &'a [String],
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

        if config.is_repetition_converted {
            for cluster in clusters.iter_mut() {
                cluster.convert_repetitions();
            }
        }

        clusters
    }

    fn is_each_test_case_matched_after_rotating_alternations(
        regex: &Regex,
        expr: &mut Expression,
        test_cases: &[String],
    ) -> bool {
        for _ in 1..test_cases.len() {
            if Self::regex_matches_all_test_cases(regex, test_cases) {
                return true;
            } else if let Expression::Alternation(options, _, _, _) = expr {
                options.rotate_right(1);
            } else if let Expression::Concatenation(first, second, _, _, _) = expr {
                let a: &mut Expression = first;
                let b: &mut Expression = second;

                if let Expression::Alternation(options, _, _, _) = a {
                    options.rotate_right(1);
                } else if let Expression::Alternation(options, _, _, _) = b {
                    options.rotate_right(1);
                }
            }
        }
        false
    }
}

impl Display for RegExp<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let flag =
            if self.config.is_case_insensitive_matching && self.config.is_verbose_mode_enabled {
                Component::IgnoreCaseAndVerboseModeFlag.to_repr(self.config.is_output_colorized)
            } else if self.config.is_case_insensitive_matching {
                Component::IgnoreCaseFlag.to_repr(self.config.is_output_colorized)
            } else if self.config.is_verbose_mode_enabled {
                Component::VerboseModeFlag.to_repr(self.config.is_output_colorized)
            } else {
                String::new()
            };

        let caret = if self.config.is_start_anchor_disabled {
            String::new()
        } else {
            Component::Caret(self.config.is_verbose_mode_enabled)
                .to_repr(self.config.is_output_colorized)
        };

        let dollar_sign = if self.config.is_end_anchor_disabled {
            String::new()
        } else {
            Component::DollarSign(self.config.is_verbose_mode_enabled)
                .to_repr(self.config.is_output_colorized)
        };

        let mut regexp = match self.ast {
            Expression::Alternation(_, _, _, _) => {
                format!(
                    "{}{}{}{}",
                    flag,
                    caret,
                    if self.config.is_capturing_group_enabled {
                        Component::CapturedParenthesizedExpression(
                            self.ast.to_string(),
                            self.config.is_verbose_mode_enabled,
                            false,
                        )
                        .to_repr(self.config.is_output_colorized)
                    } else {
                        Component::UncapturedParenthesizedExpression(
                            self.ast.to_string(),
                            self.config.is_verbose_mode_enabled,
                            false,
                        )
                        .to_repr(self.config.is_output_colorized)
                    },
                    dollar_sign
                )
            }
            _ => {
                format!("{}{}{}{}", flag, caret, self.ast, dollar_sign)
            }
        };

        regexp = regexp
            .replace('\u{b}', "\\v") // U+000B Line Tabulation
            .replace('\u{c}', "\\f"); // U+000C Form Feed

        if self.config.is_verbose_mode_enabled {
            regexp = regexp
                .replace('#', "\\#")
                .replace(
                    [
                        ' ', ' ', ' ', ' ', ' ', ' ', ' ', '\u{85}', '\u{a0}', '\u{1680}',
                        '\u{2000}', '\u{2001}', '\u{2002}', '\u{2003}', '\u{2004}', '\u{2005}',
                        '\u{2006}', '\u{2007}', '\u{2008}', '\u{2009}', '\u{200a}', '\u{2028}',
                        '\u{2029}', '\u{202f}', '\u{205f}', '\u{3000}',
                    ],
                    "\\s",
                )
                .replace(' ', "\\ ");
        }

        write!(
            f,
            "{}",
            if self.config.is_verbose_mode_enabled {
                indent_regexp(regexp, self.config)
            } else {
                regexp
            }
        )
    }
}

fn indent_regexp(regexp: String, config: &RegExpConfig) -> String {
    let mut indented_regexp = vec![];
    let mut nesting_level = 0;

    for (i, line) in regexp.lines().enumerate() {
        if i == 1 && config.is_start_anchor_disabled {
            nesting_level += 1;
        }
        if line.is_empty() {
            continue;
        }

        let is_colored_line = line.starts_with("\u{1b}[");

        if nesting_level > 0
            && ((is_colored_line && (line.contains('$') || line.contains(')')))
                || (line == "$" || line.starts_with(')')))
        {
            nesting_level -= 1;
        }

        let indentation = "  ".repeat(nesting_level);
        indented_regexp.push(format!("{indentation}{line}"));

        if (is_colored_line && (line.contains('^') || (i > 0 && line.contains('('))))
            || (line == "^" || (i > 0 && line.starts_with('(')))
        {
            nesting_level += 1;
        }
    }

    indented_regexp.join("\n")
}
