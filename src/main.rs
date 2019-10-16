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
use std::cmp::Ordering;

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
        let mut input = cli.input.iter().map(|it| it.as_str()).collect_vec();
        sort_input(&mut input);
        println_regex(input);
    } else if let Some(file_path) = cli.file_path {
        match std::fs::read_to_string(&file_path) {
            Ok(file_content) => {
                let mut input = file_content.lines().collect_vec();
                sort_input(&mut input);
                println_regex(input);
            }
            Err(error) => match error.kind() {
                ErrorKind::NotFound => {
                    eprintln!("error: The file {:?} could not be found", file_path)
                }
                _ => eprintln!(
                    "error: The file {:?} was found but could not be opened",
                    file_path
                ),
            },
        }
    }
}

fn sort_input(strs: &mut Vec<&str>) {
    strs.sort();
    strs.dedup();
    strs.sort_by(|&a, &b| match a.len().cmp(&b.len()) {
        Ordering::Equal => a.cmp(&b),
        other => other,
    });
}

fn println_regex(strs: Vec<&str>) {
    println!("{}", DFA::from(strs).to_regex());
}

#[cfg(test)]
mod tests {
    use crate::ast::{Expression, Quantifier, Substring};
    use crate::dfa::DFA;
    use crate::sort_input;
    use regex::Regex;
    use std::collections::HashMap;

    #[test]
    fn ensure_correctness_of_regular_expressions() {
        for (input, expected_output) in params() {
            assert_regex(input, expected_output);
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

    fn assert_regex(mut input: Vec<&str>, expected_output: &str) {
        let input_str = format!("[{}]", input.join(", "));
        sort_input(&mut input);
        assert_eq!(
            DFA::from(input).to_regex(),
            expected_output,
            "unexpected output for input {}",
            input_str
        );
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

            vec!["a", "ä", "o", "ö", "u", "ü", "♥"] => "^[aou\\u{e4}\\u{f6}\\u{fc}\\u{2665}]$",
            vec!["y̆", "a", "z"] => "^[az]|\\u{79}\\u{306}$", // goal: "^[az]|y\\u{306}$"

            vec!["a", "b\n", "c"] => "^b\\n|[ac]$",
            vec!["a", "b\\n", "c"] => "^b\\\\n|[ac]$"
        ]
    }
}
