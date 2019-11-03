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

use std::collections::HashMap;
use std::io::Write;
use std::process::Command;

use assert_cmd::prelude::*;
use maplit::hashmap;
use predicates::prelude::*;
use regex::Regex;
use tempfile::NamedTempFile;

#[test]
fn assert_that_grex_succeeds_with_direct_input() {
    for (input, expected_output) in default_params() {
        let mut grex = call_grex();
        grex.args(input);
        grex.assert()
            .success()
            .stdout(predicate::eq(format!("{}\n", expected_output).as_str()));
    }
}

#[test]
#[allow(unused_must_use)]
fn assert_that_grex_suceeds_with_file_input() {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, "a\nb\\n\nc\näöü\n♥");

    let mut grex = call_grex();
    grex.args(&["-f", file.path().to_str().unwrap()]);
    grex.assert()
        .success()
        .stdout(predicate::eq("^b\\\\n|äöü|[ac♥]$\n"));
}

#[test]
fn assert_that_grex_fails_without_arguments() {
    let mut grex = call_grex();
    grex.assert().failure().stderr(predicate::str::contains(
        "required arguments were not provided",
    ));
}

#[test]
fn assert_that_grex_fails_when_file_name_is_not_provided() {
    let mut grex = call_grex();
    grex.arg("-f");
    grex.assert().failure().stderr(predicate::str::contains(
        "argument '--file <FILE>' requires a value but none was supplied",
    ));
}

#[test]
fn assert_that_grex_fails_when_file_does_not_exist() {
    let mut grex = call_grex();
    grex.args(&["-f", "/path/to/non-existing/file"]);
    grex.assert()
        .success()
        .stdout(predicate::str::is_empty())
        .stderr(predicate::eq(
            "error: the specified file could not be found\n",
        ));
}

#[test]
fn assert_that_grex_fails_with_both_direct_and_file_input() {
    let mut grex = call_grex();
    grex.args(&["a", "b", "c"]);
    grex.args(&["-f", "/path/to/some/file"]);
    grex.assert().failure().stderr(predicate::str::contains(
        "argument '--file <FILE>' cannot be used with 'input'",
    ));
}

#[test]
fn ensure_regular_expressions_match_input() {
    for (input, expected_output) in default_params() {
        let re = Regex::new(expected_output).unwrap();
        for input_str in input {
            assert!(
                re.is_match(input_str),
                "\"{}\" does not match regex",
                input_str
            );
        }
    }
}

fn call_grex() -> Command {
    Command::cargo_bin("grex").unwrap()
}

fn default_params() -> HashMap<Vec<&'static str>, &'static str> {
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

        vec!["a", "ä", "o", "ö", "u", "ü"] => "^[aouäöü]$",
        vec!["y̆", "a", "z"] => "^[az]|y̆$", // goal: "^[az]|y\\u{306}$"

        vec!["a", "b\n", "c"] => "^b\\n|[ac]$",
        vec!["a", "b\\n", "c"] => "^b\\\\n|[ac]$",

        vec!["[a-z]", "(d,e,f)"] => "^\\(d,e,f\\)|\\[a\\-z\\]$",
        vec!["3.5", "4.5", "4,5"] => "^3\\.5|4[,.]5$",

        vec!["I ♥ cake"] => "^I ♥ cake$",
        vec!["I \u{2665} cake"] => "^I ♥ cake$",
        vec!["I \\u{2665} cake"] => "^I \\\\u\\{2665\\} cake$",
        vec!["I \\u2665 cake"] => "^I \\\\u2665 cake$"
    ]
}
