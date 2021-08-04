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

use crate::ast::{Expression, Quantifier};
use crate::char::GraphemeCluster;
use crate::regexp::{Component, RegExpConfig};
use itertools::Itertools;
use std::collections::BTreeSet;
use std::fmt::{Display, Formatter, Result};
use unic_char_range::CharRange;

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Expression::Alternation(options, config) => {
                format_alternation(f, self, options, config)
            }
            Expression::CharacterClass(char_set, config) => {
                format_character_class(f, char_set, config)
            }
            Expression::Concatenation(expr1, expr2, config) => {
                format_concatenation(f, self, expr1, expr2, config)
            }
            Expression::Literal(cluster, config) => format_literal(f, cluster, config),
            Expression::Repetition(expr, quantifier, config) => {
                format_repetition(f, self, expr, quantifier, config)
            }
        }
    }
}

fn get_codepoint_position(c: char) -> usize {
    CharRange::all().iter().position(|it| it == c).unwrap()
}

fn format_alternation(
    f: &mut Formatter<'_>,
    expr: &Expression,
    options: &[Expression],
    config: &RegExpConfig,
) -> Result {
    let alternation_str = options
        .iter()
        .map(|option| {
            if option.precedence() < expr.precedence() && !option.is_single_codepoint() {
                if config.is_capturing_group_enabled() {
                    Component::CapturedParenthesizedExpression(option.to_string())
                        .to_repr(config.is_output_colorized)
                } else {
                    Component::UncapturedParenthesizedExpression(option.to_string())
                        .to_repr(config.is_output_colorized)
                }
            } else {
                format!("{}", option)
            }
        })
        .join(&Component::Pipe.to_repr(config.is_output_colorized));

    write!(f, "{}", alternation_str)
}

fn format_character_class(
    f: &mut Formatter<'_>,
    char_set: &BTreeSet<char>,
    config: &RegExpConfig,
) -> Result {
    let chars_to_escape = ['[', ']', '\\', '-', '^'];
    let escaped_char_set = char_set
        .iter()
        .map(|c| {
            if chars_to_escape.contains(c) {
                format!("{}{}", "\\", c)
            } else if c == &'\n' {
                "\\n".to_string()
            } else if c == &'\r' {
                "\\r".to_string()
            } else if c == &'\t' {
                "\\t".to_string()
            } else {
                c.to_string()
            }
        })
        .collect_vec();
    let char_positions = char_set
        .iter()
        .map(|&it| get_codepoint_position(it))
        .collect_vec();

    let mut subsets = vec![];
    let mut subset = vec![];

    for ((first_c, first_pos), (second_c, second_pos)) in
        escaped_char_set.iter().zip(char_positions).tuple_windows()
    {
        if subset.is_empty() {
            subset.push(first_c);
        }
        if second_pos == first_pos + 1 {
            subset.push(second_c);
        } else {
            subsets.push(subset);
            subset = vec![second_c];
        }
    }

    subsets.push(subset);

    let mut char_class_strs = vec![];

    for subset in subsets.iter() {
        if subset.len() <= 2 {
            for c in subset.iter() {
                char_class_strs.push((*c).to_string());
            }
        } else {
            char_class_strs.push(format!(
                "{}{}{}",
                subset.first().unwrap(),
                Component::Hyphen.to_repr(config.is_output_colorized),
                subset.last().unwrap()
            ));
        }
    }

    write!(
        f,
        "{}{}{}",
        Component::LeftBracket.to_repr(config.is_output_colorized),
        char_class_strs.join(""),
        Component::RightBracket.to_repr(config.is_output_colorized)
    )
}

fn format_concatenation(
    f: &mut Formatter<'_>,
    expr: &Expression,
    expr1: &Expression,
    expr2: &Expression,
    config: &RegExpConfig,
) -> Result {
    let expr_strs = vec![expr1, expr2]
        .iter()
        .map(|&it| {
            if it.precedence() < expr.precedence() && !it.is_single_codepoint() {
                if config.is_capturing_group_enabled() {
                    Component::CapturedParenthesizedExpression(it.to_string())
                        .to_repr(config.is_output_colorized)
                } else {
                    Component::UncapturedParenthesizedExpression(it.to_string())
                        .to_repr(config.is_output_colorized)
                }
            } else {
                format!("{}", it)
            }
        })
        .collect_vec();

    write!(
        f,
        "{}{}",
        expr_strs.first().unwrap(),
        expr_strs.last().unwrap()
    )
}

fn format_literal(
    f: &mut Formatter<'_>,
    cluster: &GraphemeCluster,
    config: &RegExpConfig,
) -> Result {
    let literal_str = cluster
        .graphemes()
        .iter()
        .cloned()
        .map(|mut grapheme| {
            if grapheme.has_repetitions() {
                grapheme
                    .repetitions_mut()
                    .iter_mut()
                    .for_each(|repeated_grapheme| {
                        repeated_grapheme.escape_regexp_symbols(
                            config.is_non_ascii_char_escaped,
                            config.is_astral_code_point_converted_to_surrogate,
                        );
                    });
            } else {
                grapheme.escape_regexp_symbols(
                    config.is_non_ascii_char_escaped,
                    config.is_astral_code_point_converted_to_surrogate,
                );
            }
            grapheme.to_string()
        })
        .join("");

    write!(f, "{}", literal_str)
}

fn format_repetition(
    f: &mut Formatter<'_>,
    expr: &Expression,
    expr1: &Expression,
    quantifier: &Quantifier,
    config: &RegExpConfig,
) -> Result {
    if expr1.precedence() < expr.precedence() && !expr1.is_single_codepoint() {
        if config.is_capturing_group_enabled() {
            write!(
                f,
                "{}{}",
                Component::CapturedParenthesizedExpression(expr1.to_string())
                    .to_repr(config.is_output_colorized),
                Component::Quantifier(quantifier.clone()).to_repr(config.is_output_colorized)
            )
        } else {
            write!(
                f,
                "{}{}",
                Component::UncapturedParenthesizedExpression(expr1.to_string())
                    .to_repr(config.is_output_colorized),
                Component::Quantifier(quantifier.clone()).to_repr(config.is_output_colorized)
            )
        }
    } else {
        write!(
            f,
            "{}{}",
            expr1,
            Component::Quantifier(quantifier.clone()).to_repr(config.is_output_colorized)
        )
    }
}
