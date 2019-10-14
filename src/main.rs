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

use std::io::ErrorKind;
use std::path::PathBuf;

use itertools::Itertools;
use structopt::StructOpt;

use crate::dfa::DFA;

#[macro_use]
mod macros;
mod ast;
mod dfa;
mod fmt;

#[derive(StructOpt)]
#[structopt(author, about)]
struct CLI {
    #[structopt(required_unless = "file", conflicts_with = "file")]
    input: Vec<String>,

    #[structopt(
        name = "file",
        short,
        long,
        parse(from_os_str),
        required_unless = "input"
    )]
    file_path: Option<PathBuf>,
}

fn main() {
    let cli = CLI::from_args();

    if !cli.input.is_empty() {
        let input = cli.input.iter().map(|it| it.as_str()).collect_vec();
        println!("{}", DFA::from(input).to_regex());
    } else if let Some(file_path) = cli.file_path {
        match std::fs::read_to_string(file_path) {
            Ok(file_content) => {
                let input = file_content.lines().collect_vec();
                println!("{}", DFA::from(input).to_regex());
            }
            Err(error) => match error.kind() {
                ErrorKind::NotFound => eprintln!("Error: The provided file could not be found"),
                _ => eprintln!("Error: The provided file was found but could not be opened"),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{Expression, Quantifier, Substring};
    use crate::dfa::DFA;
    use regex::Regex;
    use std::collections::HashMap;

    #[test]
    fn ensure_correctness_of_regular_expressions() {
        for (input, expected_output) in params() {
            assert_eq!(DFA::from(input).to_regex(), expected_output);
        }
    }

    #[test]
    fn ensure_regular_expressions_match_input() {
        for (input, expected_output) in params() {
            let re = Regex::new(expected_output).unwrap();
            for input_str in input {
                assert_match(&re, input_str);
            }
        }
    }

    #[test]
    fn ensure_correct_string_representations_of_literals() {
        let params = hashmap![
            "I ♥ cake"         => "I \\u{2665} cake",
            "I \u{2665} cake"  => "I \\u{2665} cake",
            "I \\u{2665} cake" => "I \\\\u\\{2665\\} cake",
            "I \\u2665 cake"   => "I \\\\u2665 cake"
        ];

        for (input, expected_output) in params {
            let literal = Expression::new_literal(input);
            assert_eq!(literal.to_string(), expected_output);
        }
    }

    #[test]
    fn ensure_correct_matches_of_literal_regular_expressions_1() {
        let re = Regex::new("I \\u{2665} cake").unwrap();

        assert_match(&re, "I ♥ cake");
        assert_match(&re, "I \u{2665} cake");

        assert_no_match(&re, "I \\u{2665} cake");
        assert_no_match(&re, "I \\u\\{2665\\} cake");
    }

    #[test]
    fn ensure_correct_matches_of_literal_regular_expressions_2() {
        let re = Regex::new("I \\\\u\\{2665\\} cake").unwrap();

        assert_match(&re, "I \\u{2665} cake");

        assert_no_match(&re, "I \u{2665} cake");
        assert_no_match(&re, "I ♥ cake");
        assert_no_match(&re, "I \\u\\{2665\\} cake");
    }

    #[test]
    fn ensure_correct_matches_of_literal_regular_expressions_3() {
        let re = Regex::new("I \\\\u2665 cake").unwrap();

        assert_match(&re, "I \\u2665 cake");

        assert_no_match(&re, "I \u{2665} cake");
        assert_no_match(&re, "I ♥ cake");
        assert_no_match(&re, "I \\u{2665} cake");
        assert_no_match(&re, "I \\u\\{2665\\} cake");
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

    fn assert_match(re: &Regex, text: &str) {
        assert!(re.is_match(text), "\"{}\" does not match regex", text);
    }

    fn assert_no_match(re: &Regex, text: &str) {
        assert!(
            !re.is_match(text),
            "\"{}\" does match regex unexpectedly",
            text
        );
    }

    fn params() -> HashMap<Vec<&'static str>, &'static str> {
        hashmap![
            vec!["a", "b"] => "[ab]",
            vec!["a", "b", "c"] => "[a-c]",
            vec!["a", "b", "c", "d", "e", "f"] => "[a-f]",
            vec!["a", "c", "d", "e", "f"] => "[ac-f]",
            vec!["a", "b", "c", "x", "d", "e"] => "[a-ex]",
            vec!["a", "b", "c", "d", "e", "f", "o", "x", "y", "z"] => "[a-fox-z]",
            vec!["a", "b", "d", "e", "f", "o", "x", "y", "z"] => "[abd-fox-z]",
            vec!["a", "b", "bc"] => "bc?|a",
            vec!["a", "b", "bcd"] => "b(cd)?|a",
            vec!["a", "ab", "abc"] => "a(bc?)?",
            vec!["ac", "bc"] => "[ab]c",
            vec!["ab", "ac"] => "a[bc]",
            vec!["abx", "cdx"] => "(ab|cd)x",
            vec!["abd", "acd"] => "a[bc]d",
            vec!["abc", "abcd"] => "abcd?",
            vec!["abc", "abcde"] => "abc(de)?"
        ]
    }
}
