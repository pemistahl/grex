use grex::RegExpBuilder;
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
