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

use grex::RegExpBuilder;
use proptest::prelude::*;
use regex::Regex;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(300))]

    #[test]
    fn ensure_syntactically_valid_regexes_with_default_settings(
        test_cases in prop::collection::hash_set(".{1,20}", 1..=10)
    ) {
        let test_cases_vec = test_cases.iter().cloned().collect::<Vec<_>>();
        let regexp = RegExpBuilder::from(&test_cases_vec).build();
        prop_assert!(Regex::new(&regexp).is_ok());
    }

    #[test]
    fn ensure_syntactically_valid_regexes_with_converted_repetitions(
        test_cases in prop::collection::hash_set(".{1,20}", 1..=10)
    ) {
        let test_cases_vec = test_cases.iter().cloned().collect::<Vec<_>>();
        let regexp = RegExpBuilder::from(&test_cases_vec)
            .with_converted_repetitions()
            .build();
        prop_assert!(Regex::new(&regexp).is_ok());
    }

    #[test]
    fn ensure_syntactically_valid_regexes_with_escaped_non_ascii_chars(
        test_cases in prop::collection::hash_set(".{1,20}", 1..=10)
    ) {
        let test_cases_vec = test_cases.iter().cloned().collect::<Vec<_>>();
        let regexp = RegExpBuilder::from(&test_cases_vec)
            .with_escaped_non_ascii_chars(false)
            .build();
        prop_assert!(Regex::new(&regexp).is_ok());
    }

    #[test]
    fn ensure_syntactically_valid_regexes_with_converted_repetitions_and_escaped_non_ascii_chars(
        test_cases in prop::collection::hash_set(".{1,20}", 1..=10)
    ) {
        let test_cases_vec = test_cases.iter().cloned().collect::<Vec<_>>();
        let regexp = RegExpBuilder::from(&test_cases_vec)
            .with_converted_repetitions()
            .with_escaped_non_ascii_chars(false)
            .build();
        prop_assert!(Regex::new(&regexp).is_ok());
    }

    #[test]
    fn ensure_regexes_match_test_cases_with_default_settings(
        test_cases in prop::collection::hash_set(".{1,20}", 1..=10)
    ) {
        let test_cases_vec = test_cases.iter().cloned().collect::<Vec<_>>();
        let regexp = RegExpBuilder::from(&test_cases_vec).build();
        let compiled_regex = Regex::new(&regexp);

        if let Ok(compiled_regex) = compiled_regex {
            prop_assert!(test_cases.iter().all(|test_case| compiled_regex.is_match(&test_case)));
        }
    }

    #[test]
    fn ensure_regexes_match_test_cases_with_converted_repetitions(
        test_cases in prop::collection::hash_set(".{1,20}", 1..=10)
    ) {
        let test_cases_vec = test_cases.iter().cloned().collect::<Vec<_>>();
        let regexp = RegExpBuilder::from(&test_cases_vec)
            .with_converted_repetitions()
            .build();
        let compiled_regex = Regex::new(&regexp);

        if let Ok(compiled_regex) = compiled_regex {
            prop_assert!(test_cases.iter().all(|test_case| compiled_regex.is_match(&test_case)));
        }
    }

    #[test]
    fn ensure_regexes_match_test_cases_with_escaped_non_ascii_chars(
        test_cases in prop::collection::hash_set(".{1,20}", 1..=10)
    ) {
        let test_cases_vec = test_cases.iter().cloned().collect::<Vec<_>>();
        let regexp = RegExpBuilder::from(&test_cases_vec)
            .with_escaped_non_ascii_chars(false)
            .build();
        let compiled_regex = Regex::new(&regexp);

        if let Ok(compiled_regex) = compiled_regex {
            prop_assert!(test_cases.iter().all(|test_case| compiled_regex.is_match(&test_case)));
        }
    }

    #[test]
    fn ensure_regexes_match_test_cases_with_converted_repetitions_and_escaped_non_ascii_chars(
        test_cases in prop::collection::hash_set(".{1,20}", 1..=10)
    ) {
        let test_cases_vec = test_cases.iter().cloned().collect::<Vec<_>>();
        let regexp = RegExpBuilder::from(&test_cases_vec)
            .with_converted_repetitions()
            .with_escaped_non_ascii_chars(false)
            .build();
        let compiled_regex = Regex::new(&regexp);

        if let Ok(compiled_regex) = compiled_regex {
            prop_assert!(test_cases.iter().all(|test_case| compiled_regex.is_match(&test_case)));
        }
    }

    #[test]
    fn ensure_regexes_do_not_match_other_strings_with_default_settings(
        test_cases in prop::collection::hash_set(".{1,20}", 1..=10),
        other_strings in prop::collection::hash_set(".{1,20}", 1..=10)
    ) {
        if test_cases.is_disjoint(&other_strings) {
            let test_cases_vec = test_cases.iter().cloned().collect::<Vec<_>>();
            let regexp = RegExpBuilder::from(&test_cases_vec).build();
            let compiled_regex = Regex::new(&regexp);

            if let Ok(compiled_regex) = compiled_regex {
                prop_assert!(other_strings.iter().all(|other_string| !compiled_regex.is_match(&other_string)));
            }
        }
    }

    #[test]
    fn ensure_regexes_do_not_match_other_strings_with_converted_repetitions(
        test_cases in prop::collection::hash_set(".{1,20}", 1..=10),
        other_strings in prop::collection::hash_set(".{1,20}", 1..=10)
    ) {
        if test_cases.is_disjoint(&other_strings) {
            let test_cases_vec = test_cases.iter().cloned().collect::<Vec<_>>();
            let regexp = RegExpBuilder::from(&test_cases_vec)
                .with_converted_repetitions()
                .build();
            let compiled_regex = Regex::new(&regexp);

            if let Ok(compiled_regex) = compiled_regex {
                prop_assert!(other_strings.iter().all(|other_string| !compiled_regex.is_match(&other_string)));
            }
        }
    }

    #[test]
    fn ensure_regexes_do_not_match_other_strings_with_escaped_non_ascii_chars(
        test_cases in prop::collection::hash_set(".{1,20}", 1..=10),
        other_strings in prop::collection::hash_set(".{1,20}", 1..=10)
    ) {
        if test_cases.is_disjoint(&other_strings) {
            let test_cases_vec = test_cases.iter().cloned().collect::<Vec<_>>();
            let regexp = RegExpBuilder::from(&test_cases_vec)
                .with_escaped_non_ascii_chars(false)
                .build();
            let compiled_regex = Regex::new(&regexp);

            if let Ok(compiled_regex) = compiled_regex {
                prop_assert!(other_strings.iter().all(|other_string| !compiled_regex.is_match(&other_string)));
            }
        }
    }

    #[test]
    fn ensure_regexes_do_not_match_other_strings_with_converted_repetitions_and_escaped_non_ascii_chars(
        test_cases in prop::collection::hash_set(".{1,20}", 1..=10),
        other_strings in prop::collection::hash_set(".{1,20}", 1..=10)
    ) {
        if test_cases.is_disjoint(&other_strings) {
            let test_cases_vec = test_cases.iter().cloned().collect::<Vec<_>>();
            let regexp = RegExpBuilder::from(&test_cases_vec)
                .with_converted_repetitions()
                .with_escaped_non_ascii_chars(false)
                .build();
            let compiled_regex = Regex::new(&regexp);

            if let Ok(compiled_regex) = compiled_regex {
                prop_assert!(other_strings.iter().all(|other_string| !compiled_regex.is_match(&other_string)));
            }
        }
    }
}
