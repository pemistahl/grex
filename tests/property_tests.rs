use grex::RegExpBuilder;
use quickcheck::TestResult;
use quickcheck_macros::quickcheck;
use regex::Regex;
use std::collections::HashSet;

#[quickcheck]
fn produces_valid_regexes(input_strings: HashSet<String>) -> bool {
    let regexp = RegExpBuilder::from(
        input_strings
            .iter()
            .cloned()
            .collect::<Vec<String>>()
            .as_slice(),
    )
    .build();
    Regex::new(&regexp).is_ok()
}

#[quickcheck]
fn matches_each_input_string(input_strings: HashSet<String>) -> TestResult {
    let regexp = RegExpBuilder::from(
        input_strings
            .iter()
            .cloned()
            .collect::<Vec<String>>()
            .as_slice(),
    )
    .build();
    let compiled_regex = Regex::new(&regexp);
    if let Ok(compiled_regex) = compiled_regex {
        TestResult::from_bool(
            input_strings
                .into_iter()
                .all(|input| compiled_regex.is_match(&input)),
        )
    } else {
        TestResult::discard()
    }
}
