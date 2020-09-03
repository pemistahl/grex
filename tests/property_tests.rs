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

use grex::{Feature, RegExpBuilder};
use proptest::prelude::*;
use regex::{Error, Regex, RegexBuilder};

proptest! {
    #![proptest_config(ProptestConfig::with_cases(500))]

    #[test]
    #[ignore]
    fn valid_regexes_with_default_settings(
        test_cases in prop::collection::hash_set(".{1,20}", 1..=10)
    ) {
        let test_cases_vec = test_cases.iter().cloned().collect::<Vec<_>>();
        let regexp = RegExpBuilder::from(&test_cases_vec).build();
        prop_assert!(compile_regexp(&regexp).is_ok());
    }

    #[test]
    #[ignore]
    fn valid_regexes_with_escape_sequences(
        test_cases in prop::collection::hash_set(".{1,20}", 1..=10)
    ) {
        let test_cases_vec = test_cases.iter().cloned().collect::<Vec<_>>();
        let regexp = RegExpBuilder::from(&test_cases_vec)
            .with_escaping_of_non_ascii_chars(false)
            .build();
        prop_assert!(compile_regexp(&regexp).is_ok());
    }

    #[test]
    #[ignore]
    fn valid_regexes_with_conversion_features(
        test_cases in prop::collection::hash_set(".{1,20}", 1..=10),
        conversion_features in prop::collection::hash_set(conversion_feature_strategy(), 1..=9),
        minimum_repetitions in 1..100u32,
        minimum_substring_length in 1..100u32
    ) {
        let test_cases_vec = test_cases.iter().cloned().collect::<Vec<_>>();
        let regexp = RegExpBuilder::from(&test_cases_vec)
            .with_conversion_of(&conversion_features.into_iter().collect::<Vec<_>>())
            .with_minimum_repetitions(minimum_repetitions)
            .with_minimum_substring_length(minimum_substring_length)
            .build();
        prop_assert!(compile_regexp(&regexp).is_ok());
    }

    #[test]
    #[ignore]
    fn valid_regexes_with_conversion_features_and_escape_sequences(
        test_cases in prop::collection::hash_set(".{1,20}", 1..=10),
        conversion_features in prop::collection::hash_set(conversion_feature_strategy(), 1..=9),
        minimum_repetitions in 1..100u32,
        minimum_substring_length in 1..100u32
    ) {
        let test_cases_vec = test_cases.iter().cloned().collect::<Vec<_>>();
        let regexp = RegExpBuilder::from(&test_cases_vec)
            .with_conversion_of(&conversion_features.into_iter().collect::<Vec<_>>())
            .with_minimum_repetitions(minimum_repetitions)
            .with_minimum_substring_length(minimum_substring_length)
            .with_escaping_of_non_ascii_chars(false)
            .build();
        prop_assert!(compile_regexp(&regexp).is_ok());
    }

    #[test]
    #[ignore]
    fn matching_regexes_with_default_settings(
        test_cases in prop::collection::hash_set(".{1,20}", 1..=10)
    ) {
        let test_cases_vec = test_cases.iter().cloned().collect::<Vec<_>>();
        let regexp = RegExpBuilder::from(&test_cases_vec).build();
        if let Ok(compiled_regexp) = compile_regexp(&regexp) {
            prop_assert!(test_cases.iter().all(|test_case| compiled_regexp.is_match(&test_case)));
        }
    }

    #[test]
    #[ignore]
    fn matching_regexes_with_escape_sequences(
        test_cases in prop::collection::hash_set(".{1,20}", 1..=10)
    ) {
        let test_cases_vec = test_cases.iter().cloned().collect::<Vec<_>>();
        let regexp = RegExpBuilder::from(&test_cases_vec)
            .with_escaping_of_non_ascii_chars(false)
            .build();
        if let Ok(compiled_regexp) = compile_regexp(&regexp) {
            prop_assert!(test_cases.iter().all(|test_case| compiled_regexp.is_match(&test_case)));
        }
    }

    #[test]
    #[ignore]
    fn matching_regexes_with_conversion_features(
        test_cases in prop::collection::hash_set(".{1,20}", 1..=10),
        conversion_features in prop::collection::hash_set(conversion_feature_strategy(), 1..=9),
        minimum_repetitions in 1..100u32,
        minimum_substring_length in 1..100u32
    ) {
        let test_cases_vec = test_cases.iter().cloned().collect::<Vec<_>>();
        let regexp = RegExpBuilder::from(&test_cases_vec)
            .with_conversion_of(&conversion_features.into_iter().collect::<Vec<_>>())
            .with_minimum_repetitions(minimum_repetitions)
            .with_minimum_substring_length(minimum_substring_length)
            .build();
        if let Ok(compiled_regexp) = compile_regexp(&regexp) {
            prop_assert!(test_cases.iter().all(|test_case| compiled_regexp.is_match(&test_case)));
        }
    }

    #[test]
    #[ignore]
    fn matching_regexes_with_conversion_features_and_escape_sequences(
        test_cases in prop::collection::hash_set(".{1,20}", 1..=10),
        conversion_features in prop::collection::hash_set(conversion_feature_strategy(), 1..=9),
        minimum_repetitions in 1..100u32,
        minimum_substring_length in 1..100u32
    ) {
        let test_cases_vec = test_cases.iter().cloned().collect::<Vec<_>>();
        let regexp = RegExpBuilder::from(&test_cases_vec)
            .with_conversion_of(&conversion_features.into_iter().collect::<Vec<_>>())
            .with_minimum_repetitions(minimum_repetitions)
            .with_minimum_substring_length(minimum_substring_length)
            .with_escaping_of_non_ascii_chars(false)
            .build();
        if let Ok(compiled_regexp) = compile_regexp(&regexp) {
            prop_assert!(test_cases.iter().all(|test_case| compiled_regexp.is_match(&test_case)));
        }
    }

    #[test]
    #[ignore]
    fn regexes_not_matching_other_strings_with_default_settings(
        test_cases in prop::collection::hash_set(".{1,20}", 1..=10),
        other_strings in prop::collection::hash_set(".{1,20}", 1..=10)
    ) {
        if test_cases.is_disjoint(&other_strings) {
            let test_cases_vec = test_cases.iter().cloned().collect::<Vec<_>>();
            let regexp = RegExpBuilder::from(&test_cases_vec).build();
            if let Ok(compiled_regexp) = compile_regexp(&regexp) {
                prop_assert!(other_strings.iter().all(|other_string| !compiled_regexp.is_match(&other_string)));
            }
        }
    }

    #[test]
    #[ignore]
    fn regexes_not_matching_other_strings_with_default_settings_restricted_alphabet(
        test_cases in prop::collection::hash_set("[a-f]{1,10}", 1..=10),
        other_strings in prop::collection::hash_set("[a-f]{1,10}", 1..=100)
    ) {
        let test_cases_vec = test_cases.iter().cloned().collect::<Vec<_>>();
        let regexp = RegExpBuilder::from(&test_cases_vec).build();
        let compiled_regexp = compile_regexp(&regexp).unwrap();
        prop_assert!(other_strings.iter().filter(|other_string| !test_cases_vec.contains(other_string)).all(|other_string| !compiled_regexp.is_match(&other_string)));
    }


    #[test]
    #[ignore]
    fn regexes_not_matching_other_strings_with_repetition_restricted_alphabet(
        test_cases in prop::collection::hash_set("[a-f]{1,10}", 1..=10),
        other_strings in prop::collection::hash_set("[a-f]{1,10}", 1..=100)
    ) {
        let test_cases_vec = test_cases.iter().cloned().collect::<Vec<_>>();
        let regexp = RegExpBuilder::from(&test_cases_vec).with_conversion_of(&[Feature::Repetition]).build();
        let compiled_regexp = compile_regexp(&regexp).unwrap();
        prop_assert!(other_strings.iter().filter(|other_string| !test_cases_vec.contains(other_string)).all(|other_string| !compiled_regexp.is_match(&other_string)));
    }

    #[test]
    #[ignore]
    fn regexes_not_matching_other_strings_with_escape_sequences(
        test_cases in prop::collection::hash_set(".{1,20}", 1..=10),
        other_strings in prop::collection::hash_set(".{1,20}", 1..=10)
    ) {
        if test_cases.is_disjoint(&other_strings) {
            let test_cases_vec = test_cases.iter().cloned().collect::<Vec<_>>();
            let regexp = RegExpBuilder::from(&test_cases_vec)
                .with_escaping_of_non_ascii_chars(false)
                .build();
            if let Ok(compiled_regexp) = compile_regexp(&regexp) {
                prop_assert!(other_strings.iter().all(|other_string| !compiled_regexp.is_match(&other_string)));
            }
        }
    }

    #[test]
    #[ignore]
    fn regexes_not_matching_negative_test_cases(
        test_cases in prop::collection::hash_set("[a-f]{1,20}", 1..=10),
        negative_test_cases in prop::collection::hash_set("[a-f]{1,20}", 1..=10)
    ) {
        let test_cases_vec = test_cases.iter().cloned().collect::<Vec<_>>();
        let negative_test_cases_vec = negative_test_cases.iter().cloned().collect::<Vec<_>>();
        let regexp = RegExpBuilder::from(&test_cases_vec)
            .with_negative_matches(&negative_test_cases_vec)
            .build();
        let compiled_regexp = compile_regexp(&regexp).unwrap();
        for test_case in negative_test_cases {
            prop_assert_eq!(compiled_regexp.is_match(&test_case), false, "Negative test case \"{}\" matched regex \"{}\"", test_case, regexp);
        }
        for test_case in test_cases.iter().filter(|test_case| !negative_test_cases_vec.contains(&test_case)) {
            prop_assert_eq!(compiled_regexp.is_match(&test_case), true, "Positive test case \"{}\" didn't match regex \"{}\"", test_case, regexp);
        }
    }

    #[test]
    #[ignore]
    fn regexes_not_matching_negative_test_cases_with_repetition(
        test_cases in prop::collection::hash_set("[a-f]{1,20}", 1..=10),
        negative_test_cases in prop::collection::hash_set("[a-f]{1,20}", 1..=10)
    ) {
        let test_cases_vec = test_cases.iter().cloned().collect::<Vec<_>>();
        let negative_test_cases_vec = negative_test_cases.iter().cloned().collect::<Vec<_>>();
        let regexp = RegExpBuilder::from(&test_cases_vec)
            .with_negative_matches(&negative_test_cases_vec)
            .with_conversion_of(&[Feature::Repetition])
            .build();
        if let Ok(compiled_regexp) = compile_regexp(&regexp) {
            for test_case in negative_test_cases {
                prop_assert_eq!(compiled_regexp.is_match(&test_case), false, "Negative test case \"{}\" matched regex \"{}\"", test_case, regexp);
            }
            for test_case in test_cases.iter().filter(|test_case| !negative_test_cases_vec.contains(&test_case)) {
                prop_assert_eq!(compiled_regexp.is_match(&test_case), true, "Positive test case \"{}\" didn't matched regex \"{}\"", test_case, regexp);
            }
        }
    }


    #[test]
    #[ignore]
    fn regexes_not_matching_negative_test_cases_with_conversion_features_ascii_printable(
        test_cases in prop::collection::hash_set("[ -~]{1,20}", 1..=10),
        negative_test_cases in prop::collection::hash_set("[ -~]{1,20}", 1..=10),
        conversion_features in prop::collection::hash_set(conversion_feature_strategy(), 1..=9),
        minimum_repetitions in 1..100u32,
        minimum_substring_length in 1..100u32
    ) {
        let test_cases_vec = test_cases.iter().cloned().collect::<Vec<_>>();
        let negative_test_cases_vec = negative_test_cases.iter().cloned().collect::<Vec<_>>();
        let regexp = RegExpBuilder::from(&test_cases_vec)
            .with_negative_matches(&negative_test_cases_vec)
            .with_conversion_of(&conversion_features.into_iter().collect::<Vec<_>>())
            .with_minimum_repetitions(minimum_repetitions)
            .with_minimum_substring_length(minimum_substring_length)
            .build();
        if let Ok(compiled_regexp) = compile_regexp(&regexp) {
            for test_case in negative_test_cases {
                prop_assert_eq!(compiled_regexp.is_match(&test_case), false, "Negative test case \"{}\" matched regex \"{}\"", test_case, regexp);
            }
        }
    }
}

fn conversion_feature_strategy() -> impl Strategy<Value = Feature> {
    prop_oneof![
        Just(Feature::Digit),
        Just(Feature::NonDigit),
        Just(Feature::Space),
        Just(Feature::NonSpace),
        Just(Feature::Word),
        Just(Feature::NonWord),
        Just(Feature::Repetition),
        Just(Feature::CaseInsensitivity),
        Just(Feature::CapturingGroup)
    ]
}

fn compile_regexp(regexp: &str) -> Result<Regex, Error> {
    RegexBuilder::new(regexp).size_limit(20000000).build()
}
