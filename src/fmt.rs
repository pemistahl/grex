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

use std::collections::BTreeSet;
use std::fmt::{Display, Formatter, Result};

use itertools::Itertools;
use unic_char_range::CharRange;

use crate::ast::{Expression, Quantifier};
use crate::grapheme::GraphemeCluster;

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Expression::Alternation(options) => format_alternation(f, &self, options),
            Expression::CharacterClass(char_set) => format_character_class(f, char_set),
            Expression::Concatenation(expr1, expr2) => format_concatenation(f, &self, expr1, expr2),
            Expression::Literal(cluster) => format_literal(f, cluster),
            Expression::Repetition(expr, quantifier) => {
                format_repetition(f, &self, expr, quantifier)
            }
        }
    }
}

impl Display for Quantifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}",
            match self {
                Quantifier::KleeneStar => '*',
                Quantifier::QuestionMark => '?',
            }
        )
    }
}

fn get_codepoint_position(c: char) -> usize {
    CharRange::all().iter().position(|it| it == c).unwrap()
}

fn format_alternation(f: &mut Formatter<'_>, expr: &Expression, options: &[Expression]) -> Result {
    let alternation_str = options
        .iter()
        .map(|option| {
            if option.precedence() < expr.precedence() && !option.is_single_codepoint() {
                format!("({})", option)
            } else {
                format!("{}", option)
            }
        })
        .join("|");

    write!(f, "{}", alternation_str)
}

fn format_character_class(f: &mut Formatter<'_>, char_set: &BTreeSet<char>) -> Result {
    let char_positions = char_set
        .iter()
        .map(|&it| get_codepoint_position(it))
        .collect_vec();

    let mut subsets = vec![];
    let mut subset = vec![];

    for ((first_c, first_pos), (second_c, second_pos)) in
        char_set.iter().zip(char_positions).tuple_windows()
    {
        if subset.is_empty() {
            subset.push(first_c);
        }
        if second_pos == first_pos + 1 {
            subset.push(second_c);
        } else {
            subsets.push(subset);
            subset = vec![];
            subset.push(second_c);
        }
    }

    subsets.push(subset);

    let mut char_class_strs = vec![];

    for subset in subsets.iter() {
        if subset.len() <= 2 {
            for c in subset.iter() {
                char_class_strs.push(format!("{}", c));
            }
        } else {
            char_class_strs.push(format!(
                "{}-{}",
                subset.first().unwrap(),
                subset.last().unwrap()
            ));
        }
    }

    write!(f, "[{}]", char_class_strs.join(""))
}

fn format_concatenation(
    f: &mut Formatter<'_>,
    expr: &Expression,
    expr1: &Expression,
    expr2: &Expression,
) -> Result {
    let expr_strs = vec![expr1, expr2]
        .iter()
        .map(|&it| {
            if it.precedence() < expr.precedence() && !it.is_single_codepoint() {
                format!("({})", it)
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

fn format_literal(f: &mut Formatter<'_>, cluster: &GraphemeCluster) -> Result {
    let chars_to_escape = [
        "(", ")", "[", "]", "{", "}", "\\", "+", "*", "-", ".", "?", "|", "^", "$",
    ];
    let literal_str = cluster
        .graphemes()
        .iter()
        .map(|it| {
            let s = it.to_string();
            if chars_to_escape.contains(&&s[..]) {
                format!("{}{}", "\\", s)
            } else if s == "\t" {
                "\\t".to_string()
            } else if s == "\n" {
                "\\n".to_string()
            } else if s == "\r" {
                "\\r".to_string()
            } else {
                s
            }
        })
        .join("");

    write!(f, "{}", literal_str)
}

fn format_repetition(
    f: &mut Formatter<'_>,
    expr: &Expression,
    expr1: &Expression,
    quantifier: &Quantifier,
) -> Result {
    if expr1.precedence() < expr.precedence() && !expr1.is_single_codepoint() {
        write!(f, "({}){}", expr1, quantifier)
    } else {
        write!(f, "{}{}", expr1, quantifier)
    }
}
