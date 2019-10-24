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

use itertools::EitherOrBoth::Both;
use itertools::Itertools;
use maplit::btreeset;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Clone, Eq, PartialEq)]
pub(crate) enum Expression {
    Alternation(Vec<Expression>),
    CharacterClass(BTreeSet<char>),
    Concatenation(Box<Expression>, Box<Expression>),
    Literal(Vec<String>),
    Repetition(Box<Expression>, Quantifier),
}

#[derive(Clone, Eq, PartialEq)]
pub(crate) enum Quantifier {
    KleeneStar,
    QuestionMark,
}

pub(crate) enum Substring {
    Prefix,
    Suffix,
}

impl Expression {
    pub(crate) fn new_alternation(expr1: Expression, expr2: Expression) -> Self {
        let mut options: Vec<Expression> = vec![];
        flatten_alternations(&mut options, vec![expr1, expr2]);
        options.sort_by(|a, b| b.len().cmp(&a.len()));
        Expression::Alternation(options)
    }

    fn new_character_class(
        first_char_set: BTreeSet<char>,
        second_char_set: BTreeSet<char>,
    ) -> Self {
        let union_set = first_char_set.union(&second_char_set).copied().collect();
        Expression::CharacterClass(union_set)
    }

    pub(crate) fn new_concatenation(expr1: Expression, expr2: Expression) -> Self {
        Expression::Concatenation(Box::from(expr1), Box::from(expr2))
    }

    pub(crate) fn new_literal(value: &str) -> Self {
        Expression::Literal(
            UnicodeSegmentation::graphemes(value, true)
                .map(|it| it.to_string())
                .collect_vec(),
        )
    }

    pub(crate) fn new_repetition(expr: Expression, quantifier: Quantifier) -> Self {
        Expression::Repetition(Box::from(expr), quantifier)
    }

    fn is_empty(&self) -> bool {
        match self {
            Expression::Literal(graphemes) => graphemes.is_empty(),
            _ => false,
        }
    }

    pub(crate) fn is_single_codepoint(&self) -> bool {
        match self {
            Expression::CharacterClass(_) => true,
            Expression::Literal(graphemes) => {
                graphemes.len() == 1 && graphemes.first().unwrap().chars().count() == 1
            }
            _ => false,
        }
    }

    fn len(&self) -> usize {
        match self {
            Expression::Alternation(options) => options.first().unwrap().len(),
            Expression::CharacterClass(_) => 1,
            Expression::Concatenation(expr1, expr2) => expr1.len() + expr2.len(),
            Expression::Literal(graphemes) => graphemes.len(),
            Expression::Repetition(expr, _) => expr.len(),
        }
    }

    pub(crate) fn precedence(&self) -> u8 {
        match self {
            Expression::Alternation(_) | Expression::CharacterClass(_) => 1,
            Expression::Concatenation(_, _) | Expression::Literal(_) => 2,
            Expression::Repetition(_, _) => 3,
        }
    }

    pub(crate) fn remove_substring(&mut self, substring: &Substring, length: usize) {
        match self {
            Expression::Concatenation(expr1, expr2) => match substring {
                Substring::Prefix => {
                    if let Expression::Literal(_) = **expr1 {
                        expr1.remove_substring(substring, length)
                    }
                }
                Substring::Suffix => {
                    if let Expression::Literal(_) = **expr2 {
                        expr2.remove_substring(substring, length)
                    }
                }
            },
            Expression::Literal(graphemes) => match substring {
                Substring::Prefix => {
                    graphemes.drain(..length);
                }
                Substring::Suffix => {
                    graphemes.drain(graphemes.len() - length..);
                }
            },
            _ => (),
        }
    }

    pub(crate) fn value(&self, substring: Option<&Substring>) -> Option<Vec<&str>> {
        match self {
            Expression::Concatenation(expr1, expr2) => match substring {
                Some(value) => match value {
                    Substring::Prefix => expr1.value(None),
                    Substring::Suffix => expr2.value(None),
                },
                None => None,
            },
            Expression::Literal(graphemes) => {
                let mut v = vec![];
                for grapheme in graphemes {
                    v.push(grapheme.as_str());
                }
                Some(v)
            }
            _ => None,
        }
    }
}

fn flatten_alternations(flattened_options: &mut Vec<Expression>, current_options: Vec<Expression>) {
    for option in current_options {
        if let Expression::Alternation(expr_options) = option {
            flatten_alternations(flattened_options, expr_options);
        } else {
            flattened_options.push(option);
        }
    }
}

pub(crate) fn repeat_zero_or_more_times(expr: &Option<Expression>) -> Option<Expression> {
    if let Some(value) = expr {
        Some(Expression::new_repetition(
            value.clone(),
            Quantifier::KleeneStar,
        ))
    } else {
        None
    }
}

pub(crate) fn concatenate(a: &Option<Expression>, b: &Option<Expression>) -> Option<Expression> {
    if a.is_none() || b.is_none() {
        return None;
    }

    let expr1 = a.as_ref().unwrap();
    let expr2 = b.as_ref().unwrap();

    if expr1.is_empty() {
        return b.clone();
    }
    if expr2.is_empty() {
        return a.clone();
    }

    if let (Expression::Literal(graphemes_a), Expression::Literal(graphemes_b)) = (&expr1, &expr2) {
        return Some(Expression::new_literal(
            format!("{}{}", graphemes_a.join(""), graphemes_b.join("")).as_str(),
        ));
    }

    if let (Expression::Literal(graphemes_a), Expression::Concatenation(first, second)) =
        (&expr1, &expr2)
    {
        if let Expression::Literal(graphemes_first) = &**first {
            let literal = Expression::new_literal(
                format!("{}{}", graphemes_a.join(""), graphemes_first.join("")).as_str(),
            );
            return Some(Expression::new_concatenation(literal, *second.clone()));
        }
    }

    if let (Expression::Literal(graphemes_b), Expression::Concatenation(first, second)) =
        (&expr2, &expr1)
    {
        if let Expression::Literal(graphemes_second) = &**second {
            let literal = Expression::new_literal(
                format!("{}{}", graphemes_second.join(""), graphemes_b.join("")).as_str(),
            );
            return Some(Expression::new_concatenation(*first.clone(), literal));
        }
    }

    Some(Expression::new_concatenation(expr1.clone(), expr2.clone()))
}

pub(crate) fn union(a: &Option<Expression>, b: &Option<Expression>) -> Option<Expression> {
    if let (Some(mut expr1), Some(mut expr2)) = (a.clone(), b.clone()) {
        if expr1 != expr2 {
            let common_prefix = remove_common_substring(&mut expr1, &mut expr2, Substring::Prefix);
            let common_suffix = remove_common_substring(&mut expr1, &mut expr2, Substring::Suffix);

            let mut result = if expr1.is_empty() {
                Some(Expression::new_repetition(
                    expr2.clone(),
                    Quantifier::QuestionMark,
                ))
            } else if expr2.is_empty() {
                Some(Expression::new_repetition(
                    expr1.clone(),
                    Quantifier::QuestionMark,
                ))
            } else {
                None
            };

            if result.is_none() {
                if let Expression::Repetition(expr, quantifier) = expr1.clone() {
                    if quantifier == Quantifier::QuestionMark {
                        let alternation = Expression::new_alternation(*expr, expr2.clone());
                        result = Some(Expression::new_repetition(
                            alternation,
                            Quantifier::QuestionMark,
                        ));
                    }
                }
            }

            if result.is_none() {
                if let Expression::Repetition(expr, quantifier) = expr2.clone() {
                    if quantifier == Quantifier::QuestionMark {
                        let alternation = Expression::new_alternation(expr1.clone(), *expr);
                        result = Some(Expression::new_repetition(
                            alternation,
                            Quantifier::QuestionMark,
                        ));
                    }
                }
            }

            if result.is_none() && expr1.is_single_codepoint() && expr2.is_single_codepoint() {
                let first_char_set = extract_character_set(expr1.clone());
                let second_char_set = extract_character_set(expr2.clone());
                result = Some(Expression::new_character_class(
                    first_char_set,
                    second_char_set,
                ));
            }

            if result.is_none() {
                result = Some(Expression::new_alternation(expr1.clone(), expr2.clone()));
            }

            if let Some(prefix) = common_prefix {
                result = Some(Expression::new_concatenation(
                    Expression::new_literal(&prefix),
                    result.unwrap(),
                ));
            }

            if let Some(suffix) = common_suffix {
                result = Some(Expression::new_concatenation(
                    result.unwrap(),
                    Expression::new_literal(&suffix),
                ));
            }

            result
        } else if a.is_some() {
            a.clone()
        } else if b.is_some() {
            b.clone()
        } else {
            None
        }
    } else if a.is_some() {
        a.clone()
    } else if b.is_some() {
        b.clone()
    } else {
        None
    }
}

fn extract_character_set(expr: Expression) -> BTreeSet<char> {
    match expr {
        Expression::Literal(graphemes) => {
            let single_char = graphemes.first().unwrap().chars().next().unwrap();
            btreeset![single_char]
        }
        Expression::CharacterClass(char_set) => char_set,
        _ => BTreeSet::new(),
    }
}

fn remove_common_substring(
    a: &mut Expression,
    b: &mut Expression,
    substring: Substring,
) -> Option<String> {
    let common_substring = find_common_substring(a, b, &substring);
    if let Some(value) = &common_substring {
        a.remove_substring(&substring, value.len());
        b.remove_substring(&substring, value.len());
    }
    common_substring
}

fn find_common_substring(a: &Expression, b: &Expression, substring: &Substring) -> Option<String> {
    let mut graphemes_a = a.value(Some(substring)).unwrap_or_else(|| vec![]);
    let mut graphemes_b = b.value(Some(substring)).unwrap_or_else(|| vec![]);
    let mut common_graphemes = vec![];

    if let Substring::Suffix = substring {
        graphemes_a.reverse();
        graphemes_b.reverse();
    }

    for pair in graphemes_a.iter().zip_longest(graphemes_b.iter()) {
        match pair {
            Both(grapheme_a, grapheme_b) => {
                if grapheme_a == grapheme_b {
                    common_graphemes.push(*grapheme_a);
                } else {
                    break;
                }
            }
            _ => break,
        }
    }

    if let Substring::Suffix = substring {
        common_graphemes.reverse();
    }

    if common_graphemes.is_empty() {
        None
    } else {
        Some(common_graphemes.join(""))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ensure_correct_string_representation_of_alternation_1() {
        let literal1 = Expression::new_literal("abc");
        let literal2 = Expression::new_literal("def");
        let alternation = Expression::new_alternation(literal1, literal2);
        assert_eq!(alternation.to_string(), "abc|def");
    }

    #[test]
    fn ensure_correct_string_representation_of_alternation_2() {
        let literal1 = Expression::new_literal("a");
        let literal2 = Expression::new_literal("ab");
        let literal3 = Expression::new_literal("abc");
        let alternation1 = Expression::new_alternation(literal1, literal2);
        let alternation2 = Expression::new_alternation(alternation1, literal3);
        assert_eq!(alternation2.to_string(), "abc|ab|a");
    }

    #[test]
    fn ensure_correct_string_representation_of_concatenation_1() {
        let literal1 = Expression::new_literal("abc");
        let literal2 = Expression::new_literal("def");
        let concatenation = Expression::new_concatenation(literal1, literal2);
        assert_eq!(concatenation.to_string(), "abcdef");
    }

    #[test]
    fn ensure_correct_string_representation_of_concatenation_2() {
        let literal1 = Expression::new_literal("abc");
        let literal2 = Expression::new_literal("def");
        let repetition = Expression::new_repetition(literal1, Quantifier::KleeneStar);
        let concatenation = Expression::new_concatenation(repetition, literal2);
        assert_eq!(concatenation.to_string(), "(abc)*def");
    }

    #[test]
    fn ensure_correct_removal_of_prefix_in_literal() {
        let mut literal = Expression::new_literal("abcdef");
        assert_eq!(
            literal.value(None),
            Some(vec!["a", "b", "c", "d", "e", "f"])
        );

        literal.remove_substring(&Substring::Prefix, 2);
        assert_eq!(literal.value(None), Some(vec!["c", "d", "e", "f"]));
    }

    #[test]
    fn ensure_correct_removal_of_suffix_in_literal() {
        let mut literal = Expression::new_literal("abcdef");
        assert_eq!(
            literal.value(None),
            Some(vec!["a", "b", "c", "d", "e", "f"])
        );

        literal.remove_substring(&Substring::Suffix, 2);
        assert_eq!(literal.value(None), Some(vec!["a", "b", "c", "d"]));
    }

    #[test]
    fn ensure_correct_string_representation_of_repetition_1() {
        let literal = Expression::new_literal("abc");
        let repetition = Expression::new_repetition(literal, Quantifier::KleeneStar);
        assert_eq!(repetition.to_string(), "(abc)*");
    }

    #[test]
    fn ensure_correct_string_representation_of_repetition_2() {
        let literal = Expression::new_literal("a");
        let repetition = Expression::new_repetition(literal, Quantifier::QuestionMark);
        assert_eq!(repetition.to_string(), "a?");
    }
}
