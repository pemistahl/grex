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

#![cfg(not(target_family = "wasm"))]

use grex::RegExpBuilder;
use proptest::prelude::*;
use regex::{Error, Regex, RegexBuilder};

proptest! {
    #![proptest_config(ProptestConfig::with_cases(500))]

    #[test]
    fn valid_regexes_with_default_settings(
        test_cases in prop::collection::hash_set(".{1,10}", 1..=5)
    ) {
        let test_cases_vec = test_cases.iter().cloned().collect::<Vec<_>>();
        let regexp = RegExpBuilder::from(&test_cases_vec).build();
        prop_assert!(compile_regexp(&regexp).is_ok());
    }

    #[test]
    fn valid_regexes_with_case_insensitive_matching(
        test_cases in prop::collection::hash_set(".{1,10}", 1..=5)
    ) {
        let test_cases_vec = test_cases.iter().cloned().collect::<Vec<_>>();
        let regexp = RegExpBuilder::from(&test_cases_vec)
            .with_case_insensitive_matching()
            .build();
        prop_assert!(compile_regexp(&regexp).is_ok());
    }

    #[test]
    fn valid_regexes_with_case_insensitive_matching_and_verbose_mode(
        test_cases in prop::collection::hash_set(".{1,10}", 1..=5)
    ) {
        let test_cases_vec = test_cases.iter().cloned().collect::<Vec<_>>();
        let regexp = RegExpBuilder::from(&test_cases_vec)
            .with_case_insensitive_matching()
            .with_verbose_mode()
            .build();
        prop_assert!(compile_regexp(&regexp).is_ok());
    }

    #[test]
    fn valid_regexes_with_escape_sequences(
        test_cases in prop::collection::hash_set(".{1,10}", 1..=5)
    ) {
        let test_cases_vec = test_cases.iter().cloned().collect::<Vec<_>>();
        let regexp = RegExpBuilder::from(&test_cases_vec)
            .with_escaping_of_non_ascii_chars(false)
            .build();
        prop_assert!(compile_regexp(&regexp).is_ok());
    }

    #[test]
    fn valid_regexes_with_verbose_mode(
        test_cases in prop::collection::hash_set(".{1,10}", 1..=5)
    ) {
        let test_cases_vec = test_cases.iter().cloned().collect::<Vec<_>>();
        let regexp = RegExpBuilder::from(&test_cases_vec)
            .with_verbose_mode()
            .build();
        prop_assert!(compile_regexp(&regexp).is_ok());
    }

    #[test]
    fn valid_regexes_with_escape_sequences_and_verbose_mode(
        test_cases in prop::collection::hash_set(".{1,10}", 1..=5)
    ) {
        let test_cases_vec = test_cases.iter().cloned().collect::<Vec<_>>();
        let regexp = RegExpBuilder::from(&test_cases_vec)
            .with_escaping_of_non_ascii_chars(false)
            .with_verbose_mode()
            .build();
        prop_assert!(compile_regexp(&regexp).is_ok());
    }

    #[test]
    fn valid_regexes_with_conversion_of_digits(
        test_cases in prop::collection::hash_set(".{1,10}", 1..=5)
    ) {
        let test_cases_vec = test_cases.iter().cloned().collect::<Vec<_>>();
        let regexp = RegExpBuilder::from(&test_cases_vec)
            .with_conversion_of_digits()
            .build();
        prop_assert!(compile_regexp(&regexp).is_ok());
    }

    #[test]
    fn valid_regexes_with_conversion_of_digits_and_verbose_mode(
        test_cases in prop::collection::hash_set(".{1,10}", 1..=5)
    ) {
        let test_cases_vec = test_cases.iter().cloned().collect::<Vec<_>>();
        let regexp = RegExpBuilder::from(&test_cases_vec)
            .with_conversion_of_digits()
            .with_verbose_mode()
            .build();
        prop_assert!(compile_regexp(&regexp).is_ok());
    }

    #[test]
    fn valid_regexes_with_conversion_of_non_digits(
        test_cases in prop::collection::hash_set(".{1,10}", 1..=5)
    ) {
        let test_cases_vec = test_cases.iter().cloned().collect::<Vec<_>>();
        let regexp = RegExpBuilder::from(&test_cases_vec)
            .with_conversion_of_non_digits()
            .build();
        prop_assert!(compile_regexp(&regexp).is_ok());
    }

    #[test]
    fn valid_regexes_with_conversion_of_non_digits_and_verbose_mode(
        test_cases in prop::collection::hash_set(".{1,10}", 1..=5)
    ) {
        let test_cases_vec = test_cases.iter().cloned().collect::<Vec<_>>();
        let regexp = RegExpBuilder::from(&test_cases_vec)
            .with_conversion_of_non_digits()
            .with_verbose_mode()
            .build();
        prop_assert!(compile_regexp(&regexp).is_ok());
    }

    #[test]
    fn valid_regexes_with_conversion_of_whitespace(
        test_cases in prop::collection::hash_set(".{1,10}", 1..=5)
    ) {
        let test_cases_vec = test_cases.iter().cloned().collect::<Vec<_>>();
        let regexp = RegExpBuilder::from(&test_cases_vec)
            .with_conversion_of_whitespace()
            .build();
        prop_assert!(compile_regexp(&regexp).is_ok());
    }

    #[test]
    fn valid_regexes_with_conversion_of_whitespace_and_verbose_mode(
        test_cases in prop::collection::hash_set(".{1,10}", 1..=5)
    ) {
        let test_cases_vec = test_cases.iter().cloned().collect::<Vec<_>>();
        let regexp = RegExpBuilder::from(&test_cases_vec)
            .with_conversion_of_whitespace()
            .with_verbose_mode()
            .build();
        prop_assert!(compile_regexp(&regexp).is_ok());
    }

    #[test]
    fn valid_regexes_with_conversion_of_non_whitespace(
        test_cases in prop::collection::hash_set(".{1,10}", 1..=5)
    ) {
        let test_cases_vec = test_cases.iter().cloned().collect::<Vec<_>>();
        let regexp = RegExpBuilder::from(&test_cases_vec)
            .with_conversion_of_non_whitespace()
            .build();
        prop_assert!(compile_regexp(&regexp).is_ok());
    }

    #[test]
    fn valid_regexes_with_conversion_of_non_whitespace_and_verbose_mode(
        test_cases in prop::collection::hash_set(".{1,10}", 1..=5)
    ) {
        let test_cases_vec = test_cases.iter().cloned().collect::<Vec<_>>();
        let regexp = RegExpBuilder::from(&test_cases_vec)
            .with_conversion_of_non_whitespace()
            .with_verbose_mode()
            .build();
        prop_assert!(compile_regexp(&regexp).is_ok());
    }

    #[test]
    fn valid_regexes_with_conversion_of_words(
        test_cases in prop::collection::hash_set(".{1,10}", 1..=5)
    ) {
        let test_cases_vec = test_cases.iter().cloned().collect::<Vec<_>>();
        let regexp = RegExpBuilder::from(&test_cases_vec)
            .with_conversion_of_words()
            .build();
        prop_assert!(compile_regexp(&regexp).is_ok());
    }

    #[test]
    fn valid_regexes_with_conversion_of_words_and_verbose_mode(
        test_cases in prop::collection::hash_set(".{1,10}", 1..=5)
    ) {
        let test_cases_vec = test_cases.iter().cloned().collect::<Vec<_>>();
        let regexp = RegExpBuilder::from(&test_cases_vec)
            .with_conversion_of_words()
            .with_verbose_mode()
            .build();
        prop_assert!(compile_regexp(&regexp).is_ok());
    }

    #[test]
    fn valid_regexes_with_conversion_of_non_words(
        test_cases in prop::collection::hash_set(".{1,10}", 1..=5)
    ) {
        let test_cases_vec = test_cases.iter().cloned().collect::<Vec<_>>();
        let regexp = RegExpBuilder::from(&test_cases_vec)
            .with_conversion_of_non_words()
            .build();
        prop_assert!(compile_regexp(&regexp).is_ok());
    }

    #[test]
    fn valid_regexes_with_conversion_of_non_words_and_verbose_mode(
        test_cases in prop::collection::hash_set(".{1,10}", 1..=5)
    ) {
        let test_cases_vec = test_cases.iter().cloned().collect::<Vec<_>>();
        let regexp = RegExpBuilder::from(&test_cases_vec)
            .with_conversion_of_non_words()
            .with_verbose_mode()
            .build();
        prop_assert!(compile_regexp(&regexp).is_ok());
    }

    #[test]
    fn valid_regexes_with_conversion_of_repetitions(
        test_cases in prop::collection::hash_set(".{1,10}", 1..=5),
        minimum_repetitions in 1..100u32,
        minimum_substring_length in 1..100u32
    ) {
        let test_cases_vec = test_cases.iter().cloned().collect::<Vec<_>>();
        let regexp = RegExpBuilder::from(&test_cases_vec)
            .with_conversion_of_repetitions()
            .with_minimum_repetitions(minimum_repetitions)
            .with_minimum_substring_length(minimum_substring_length)
            .build();
        prop_assert!(compile_regexp(&regexp).is_ok());
    }

    #[test]
    fn valid_regexes_with_conversion_of_repetitions_and_verbose_mode(
        test_cases in prop::collection::hash_set(".{1,10}", 1..=5),
        minimum_repetitions in 1..100u32,
        minimum_substring_length in 1..100u32
    ) {
        let test_cases_vec = test_cases.iter().cloned().collect::<Vec<_>>();
        let regexp = RegExpBuilder::from(&test_cases_vec)
            .with_conversion_of_repetitions()
            .with_minimum_repetitions(minimum_repetitions)
            .with_minimum_substring_length(minimum_substring_length)
            .with_verbose_mode()
            .build();
        prop_assert!(compile_regexp(&regexp).is_ok());
    }

    #[test]
    fn matching_regexes_with_default_settings(
        test_cases in prop::collection::hash_set(".{1,10}", 1..=5)
    ) {
        let test_cases_vec = test_cases.iter().cloned().collect::<Vec<_>>();
        let regexp = RegExpBuilder::from(&test_cases_vec).build();
        if let Ok(compiled_regexp) = compile_regexp(&regexp) {
            prop_assert!(test_cases.iter().all(|test_case| compiled_regexp.is_match(test_case)));
        }
    }

    #[test]
    fn matching_regexes_with_case_insensitive_matching(
        test_cases in prop::collection::hash_set(".{1,10}", 1..=5)
    ) {
        let test_cases_vec = test_cases.iter().cloned().collect::<Vec<_>>();
        let regexp = RegExpBuilder::from(&test_cases_vec)
            .with_case_insensitive_matching()
            .build();
        if let Ok(compiled_regexp) = compile_regexp(&regexp) {
            prop_assert!(test_cases.iter().all(|test_case| compiled_regexp.is_match(test_case)));
        }
    }

    #[test]
    fn matching_regexes_with_case_insensitive_matching_and_verbose_mode(
        test_cases in prop::collection::hash_set(".{1,10}", 1..=5)
    ) {
        let test_cases_vec = test_cases.iter().cloned().collect::<Vec<_>>();
        let regexp = RegExpBuilder::from(&test_cases_vec)
            .with_case_insensitive_matching()
            .with_verbose_mode()
            .build();
        if let Ok(compiled_regexp) = compile_regexp(&regexp) {
            prop_assert!(test_cases.iter().all(|test_case| compiled_regexp.is_match(test_case)));
        }
    }

    #[test]
    fn matching_regexes_with_escape_sequences(
        test_cases in prop::collection::hash_set(".{1,10}", 1..=5)
    ) {
        let test_cases_vec = test_cases.iter().cloned().collect::<Vec<_>>();
        let regexp = RegExpBuilder::from(&test_cases_vec)
            .with_escaping_of_non_ascii_chars(false)
            .build();
        if let Ok(compiled_regexp) = compile_regexp(&regexp) {
            prop_assert!(test_cases.iter().all(|test_case| compiled_regexp.is_match(test_case)));
        }
    }

    #[test]
    fn matching_regexes_with_verbose_mode(
        test_cases in prop::collection::hash_set(".{1,10}", 1..=5)
    ) {
        let test_cases_vec = test_cases.iter().cloned().collect::<Vec<_>>();
        let regexp = RegExpBuilder::from(&test_cases_vec)
            .with_verbose_mode()
            .build();
        if let Ok(compiled_regexp) = compile_regexp(&regexp) {
            prop_assert!(test_cases.iter().all(|test_case| compiled_regexp.is_match(test_case)));
        }
    }

    #[test]
    fn matching_regexes_with_escape_sequences_and_verbose_mode(
        test_cases in prop::collection::hash_set(".{1,10}", 1..=5)
    ) {
        let test_cases_vec = test_cases.iter().cloned().collect::<Vec<_>>();
        let regexp = RegExpBuilder::from(&test_cases_vec)
            .with_escaping_of_non_ascii_chars(false)
            .with_verbose_mode()
            .build();
        if let Ok(compiled_regexp) = compile_regexp(&regexp) {
            prop_assert!(test_cases.iter().all(|test_case| compiled_regexp.is_match(test_case)));
        }
    }

    #[test]
    fn matching_regexes_with_conversion_of_digits(
        test_cases in prop::collection::hash_set(".{1,10}", 1..=5)
    ) {
        let test_cases_vec = test_cases.iter().cloned().collect::<Vec<_>>();
        let regexp = RegExpBuilder::from(&test_cases_vec)
            .with_conversion_of_digits()
            .build();
        if let Ok(compiled_regexp) = compile_regexp(&regexp) {
            prop_assert!(test_cases.iter().all(|test_case| compiled_regexp.is_match(test_case)));
        }
    }

    #[test]
    fn matching_regexes_with_conversion_of_digits_and_verbose_mode(
        test_cases in prop::collection::hash_set(".{1,10}", 1..=5)
    ) {
        let test_cases_vec = test_cases.iter().cloned().collect::<Vec<_>>();
        let regexp = RegExpBuilder::from(&test_cases_vec)
            .with_conversion_of_digits()
            .with_verbose_mode()
            .build();
        if let Ok(compiled_regexp) = compile_regexp(&regexp) {
            prop_assert!(test_cases.iter().all(|test_case| compiled_regexp.is_match(test_case)));
        }
    }

    #[test]
    fn matching_regexes_with_conversion_of_non_digits(
        test_cases in prop::collection::hash_set(".{1,10}", 1..=5)
    ) {
        let test_cases_vec = test_cases.iter().cloned().collect::<Vec<_>>();
        let regexp = RegExpBuilder::from(&test_cases_vec)
            .with_conversion_of_digits()
            .build();
        if let Ok(compiled_regexp) = compile_regexp(&regexp) {
            prop_assert!(test_cases.iter().all(|test_case| compiled_regexp.is_match(test_case)));
        }
    }

    #[test]
    fn matching_regexes_with_conversion_of_non_digits_and_verbose_mode(
        test_cases in prop::collection::hash_set(".{1,10}", 1..=5)
    ) {
        let test_cases_vec = test_cases.iter().cloned().collect::<Vec<_>>();
        let regexp = RegExpBuilder::from(&test_cases_vec)
            .with_conversion_of_digits()
            .with_verbose_mode()
            .build();
        if let Ok(compiled_regexp) = compile_regexp(&regexp) {
            prop_assert!(test_cases.iter().all(|test_case| compiled_regexp.is_match(test_case)));
        }
    }

    #[test]
    fn matching_regexes_with_conversion_of_whitespace(
        test_cases in prop::collection::hash_set(".{1,10}", 1..=5)
    ) {
        let test_cases_vec = test_cases.iter().cloned().collect::<Vec<_>>();
        let regexp = RegExpBuilder::from(&test_cases_vec)
            .with_conversion_of_whitespace()
            .build();
        if let Ok(compiled_regexp) = compile_regexp(&regexp) {
            prop_assert!(test_cases.iter().all(|test_case| compiled_regexp.is_match(test_case)));
        }
    }

    #[test]
    fn matching_regexes_with_conversion_of_whitespace_and_verbose_mode(
        test_cases in prop::collection::hash_set(".{1,10}", 1..=5)
    ) {
        let test_cases_vec = test_cases.iter().cloned().collect::<Vec<_>>();
        let regexp = RegExpBuilder::from(&test_cases_vec)
            .with_conversion_of_whitespace()
            .with_verbose_mode()
            .build();
        if let Ok(compiled_regexp) = compile_regexp(&regexp) {
            prop_assert!(test_cases.iter().all(|test_case| compiled_regexp.is_match(test_case)));
        }
    }

    #[test]
    fn matching_regexes_with_conversion_of_non_whitespace(
        test_cases in prop::collection::hash_set(".{1,10}", 1..=5)
    ) {
        let test_cases_vec = test_cases.iter().cloned().collect::<Vec<_>>();
        let regexp = RegExpBuilder::from(&test_cases_vec)
            .with_conversion_of_non_whitespace()
            .build();
        if let Ok(compiled_regexp) = compile_regexp(&regexp) {
            prop_assert!(test_cases.iter().all(|test_case| compiled_regexp.is_match(test_case)));
        }
    }

    #[test]
    fn matching_regexes_with_conversion_of_non_whitespace_and_verbose_mode(
        test_cases in prop::collection::hash_set(".{1,10}", 1..=5)
    ) {
        let test_cases_vec = test_cases.iter().cloned().collect::<Vec<_>>();
        let regexp = RegExpBuilder::from(&test_cases_vec)
            .with_conversion_of_non_whitespace()
            .with_verbose_mode()
            .build();
        if let Ok(compiled_regexp) = compile_regexp(&regexp) {
            prop_assert!(test_cases.iter().all(|test_case| compiled_regexp.is_match(test_case)));
        }
    }

    #[test]
    fn matching_regexes_with_conversion_of_words(
        test_cases in prop::collection::hash_set(".{1,10}", 1..=5)
    ) {
        let test_cases_vec = test_cases.iter().cloned().collect::<Vec<_>>();
        let regexp = RegExpBuilder::from(&test_cases_vec)
            .with_conversion_of_words()
            .build();
        if let Ok(compiled_regexp) = compile_regexp(&regexp) {
            prop_assert!(test_cases.iter().all(|test_case| compiled_regexp.is_match(test_case)));
        }
    }

    #[test]
    fn matching_regexes_with_conversion_of_words_and_verbose_mode(
        test_cases in prop::collection::hash_set(".{1,10}", 1..=5)
    ) {
        let test_cases_vec = test_cases.iter().cloned().collect::<Vec<_>>();
        let regexp = RegExpBuilder::from(&test_cases_vec)
            .with_conversion_of_words()
            .with_verbose_mode()
            .build();
        if let Ok(compiled_regexp) = compile_regexp(&regexp) {
            prop_assert!(test_cases.iter().all(|test_case| compiled_regexp.is_match(test_case)));
        }
    }

    #[test]
    fn matching_regexes_with_conversion_of_non_words(
        test_cases in prop::collection::hash_set(".{1,10}", 1..=5)
    ) {
        let test_cases_vec = test_cases.iter().cloned().collect::<Vec<_>>();
        let regexp = RegExpBuilder::from(&test_cases_vec)
            .with_conversion_of_non_words()
            .build();
        if let Ok(compiled_regexp) = compile_regexp(&regexp) {
            prop_assert!(test_cases.iter().all(|test_case| compiled_regexp.is_match(test_case)));
        }
    }

    #[test]
    fn matching_regexes_with_conversion_of_non_words_and_verbose_mode(
        test_cases in prop::collection::hash_set(".{1,10}", 1..=5)
    ) {
        let test_cases_vec = test_cases.iter().cloned().collect::<Vec<_>>();
        let regexp = RegExpBuilder::from(&test_cases_vec)
            .with_conversion_of_non_words()
            .with_verbose_mode()
            .build();
        if let Ok(compiled_regexp) = compile_regexp(&regexp) {
            prop_assert!(test_cases.iter().all(|test_case| compiled_regexp.is_match(test_case)));
        }
    }

    #[test]
    fn matching_regexes_with_conversion_of_repetitions(
        test_cases in prop::collection::hash_set(".{1,10}", 1..=5),
        minimum_repetitions in 1..100u32,
        minimum_substring_length in 1..100u32
    ) {
        let test_cases_vec = test_cases.iter().cloned().collect::<Vec<_>>();
        let regexp = RegExpBuilder::from(&test_cases_vec)
            .with_conversion_of_repetitions()
            .with_minimum_repetitions(minimum_repetitions)
            .with_minimum_substring_length(minimum_substring_length)
            .build();
        if let Ok(compiled_regexp) = compile_regexp(&regexp) {
            prop_assert!(test_cases.iter().all(|test_case| compiled_regexp.is_match(test_case)));
        }
    }

    #[test]
    fn matching_regexes_with_conversion_of_repetitions_and_verbose_mode(
        test_cases in prop::collection::hash_set(".{1,10}", 1..=5),
        minimum_repetitions in 1..100u32,
        minimum_substring_length in 1..100u32
    ) {
        let test_cases_vec = test_cases.iter().cloned().collect::<Vec<_>>();
        let regexp = RegExpBuilder::from(&test_cases_vec)
            .with_conversion_of_repetitions()
            .with_minimum_repetitions(minimum_repetitions)
            .with_minimum_substring_length(minimum_substring_length)
            .with_verbose_mode()
            .build();
        if let Ok(compiled_regexp) = compile_regexp(&regexp) {
            prop_assert!(test_cases.iter().all(|test_case| compiled_regexp.is_match(test_case)));
        }
    }

    #[test]
    fn matching_regexes_without_start_anchor(
        test_cases in prop::collection::hash_set("[A-C]{1,10}", 1..=5)
    ) {
        let test_cases_vec = test_cases.iter().cloned().collect::<Vec<_>>();
        let regexp = RegExpBuilder::from(&test_cases_vec).without_start_anchor().build();
        if let Ok(compiled_regexp) = compile_regexp(&regexp) {
            for test_case in test_cases_vec {
                let substrings = compiled_regexp.find_iter(&test_case).map(|m| m.as_str()).collect::<Vec<_>>();
                prop_assert_eq!(
                    substrings.len(),
                    1,
                    "expression '{}' does not match test case '{}' entirely but {} of its substrings: {:?}",
                    regexp,
                    test_case,
                    substrings.len(),
                    substrings
                );
            }
        }
    }

    #[test]
    fn matching_regexes_without_end_anchor(
        test_cases in prop::collection::hash_set("[A-C]{1,10}", 1..=5)
    ) {
        let test_cases_vec = test_cases.iter().cloned().collect::<Vec<_>>();
        let regexp = RegExpBuilder::from(&test_cases_vec).without_end_anchor().build();
        if let Ok(compiled_regexp) = compile_regexp(&regexp) {
            for test_case in test_cases_vec {
                let substrings = compiled_regexp.find_iter(&test_case).map(|m| m.as_str()).collect::<Vec<_>>();
                prop_assert_eq!(
                    substrings.len(),
                    1,
                    "expression '{}' does not match test case '{}' entirely but {} of its substrings: {:?}",
                    regexp,
                    test_case,
                    substrings.len(),
                    substrings
                );
            }
        }
    }

    #[test]
    fn matching_regexes_without_anchors(
        test_cases in prop::collection::hash_set("[A-C]{1,10}", 1..=5)
    ) {
        let test_cases_vec = test_cases.iter().cloned().collect::<Vec<_>>();
        let regexp = RegExpBuilder::from(&test_cases_vec).without_anchors().build();
        if let Ok(compiled_regexp) = compile_regexp(&regexp) {
            for test_case in test_cases_vec {
                let substrings = compiled_regexp.find_iter(&test_case).map(|m| m.as_str()).collect::<Vec<_>>();
                prop_assert_eq!(
                    substrings.len(),
                    1,
                    "expression '{}' does not match test case '{}' entirely but {} of its substrings: {:?}",
                    regexp,
                    test_case,
                    substrings.len(),
                    substrings
                );
            }
        }
    }

    #[test]
    fn regexes_not_matching_other_strings_with_default_settings(
        test_cases in prop::collection::hash_set(".{1,10}", 1..=5),
        other_strings in prop::collection::hash_set(".{1,10}", 1..=5)
    ) {
        if test_cases.is_disjoint(&other_strings) {
            let test_cases_vec = test_cases.iter().cloned().collect::<Vec<_>>();
            let regexp = RegExpBuilder::from(&test_cases_vec).build();
            if let Ok(compiled_regexp) = compile_regexp(&regexp) {
                prop_assert!(other_strings.iter().all(|other_string| !compiled_regexp.is_match(other_string)));
            }
        }
    }

    #[test]
    fn regexes_not_matching_other_strings_with_escape_sequences(
        test_cases in prop::collection::hash_set(".{1,10}", 1..=5),
        other_strings in prop::collection::hash_set(".{1,10}", 1..=5)
    ) {
        if test_cases.is_disjoint(&other_strings) {
            let test_cases_vec = test_cases.iter().cloned().collect::<Vec<_>>();
            let regexp = RegExpBuilder::from(&test_cases_vec)
                .with_escaping_of_non_ascii_chars(false)
                .build();
            if let Ok(compiled_regexp) = compile_regexp(&regexp) {
                prop_assert!(other_strings.iter().all(|other_string| !compiled_regexp.is_match(other_string)));
            }
        }
    }
}

fn compile_regexp(regexp: &str) -> Result<Regex, Error> {
    RegexBuilder::new(regexp).size_limit(20000000).build()
}
