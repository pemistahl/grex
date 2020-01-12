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

use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::io::Write;
use std::process::Command;
use tempfile::NamedTempFile;

#[test]
fn succeeds_with_direct_input() {
    let mut grex = call_grex();
    grex.args(&["a", "b", "c"]);
    grex.assert().success().stdout(predicate::eq("^[a-c]$\n"));
}

#[test]
fn succeeds_with_repetition_conversion_option() {
    let mut grex = call_grex();
    grex.args(&["--convert-repetitions", "xy̆y̆z", "xy̆y̆y̆z"]);
    grex.assert()
        .success()
        .stdout(predicate::eq("^x(y̆){2,3}z$\n"));
}

#[test]
fn succeeds_with_repetition_conversion_and_escape_option() {
    let mut grex = call_grex();
    grex.args(&[
        "--convert-repetitions",
        "--escape",
        "My ♥♥♥ and 💩💩 is yours.",
    ]);
    grex.assert().success().stdout(predicate::eq(
        "^My \\u{2665}{3} and \\u{1f4a9}{2} is yours\\.$\n",
    ));
}

#[test]
fn succeeds_with_repetition_conversion_and_escape_and_surrogate_option() {
    let mut grex = call_grex();
    grex.args(&[
        "--convert-repetitions",
        "--escape",
        "--with-surrogates",
        "My ♥♥♥ and 💩💩 is yours.",
    ]);
    grex.assert().success().stdout(predicate::eq(
        "^My \\u{2665}{3} and (\\u{d83d}\\u{dca9}){2} is yours\\.$\n",
    ));
}

#[test]
fn succeeds_with_escape_option() {
    let mut grex = call_grex();
    grex.args(&["--escape", "My ♥♥ and 💩 is yours."]);
    grex.assert().success().stdout(predicate::eq(
        "^My \\u{2665}\\u{2665} and \\u{1f4a9} is yours\\.$\n",
    ));
}

#[test]
fn succeeds_with_escape_and_surrogate_option() {
    let mut grex = call_grex();
    grex.args(&["--escape", "--with-surrogates", "My ♥♥ and 💩 is yours."]);
    grex.assert().success().stdout(predicate::eq(
        "^My \\u{2665}\\u{2665} and \\u{d83d}\\u{dca9} is yours\\.$\n",
    ));
}

#[test]
fn fails_with_surrogate_but_without_escape_option() {
    let mut grex = call_grex();
    grex.args(&["--with-surrogates", "My ♥ and 💩 is yours."]);
    grex.assert().failure().stderr(predicate::str::contains(
        "required arguments were not provided",
    ));
}

#[test]
#[allow(unused_must_use)]
fn suceeds_with_file_input() {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, "a\nb\\n\n\nc\näöü\n♥");

    let mut grex = call_grex();
    grex.args(&["-f", file.path().to_str().unwrap()]);
    grex.assert()
        .success()
        .stdout(predicate::eq("^(b\\\\n|äöü|[ac♥])$\n"));
}

#[test]
fn fails_without_arguments() {
    let mut grex = call_grex();
    grex.assert().failure().stderr(predicate::str::contains(
        "required arguments were not provided",
    ));
}

#[test]
fn fails_when_file_name_is_not_provided() {
    let mut grex = call_grex();
    grex.arg("-f");
    grex.assert().failure().stderr(predicate::str::contains(
        "argument '--file <FILE>' requires a value but none was supplied",
    ));
}

#[test]
fn fails_when_file_does_not_exist() {
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
fn fails_with_both_direct_and_file_input() {
    let mut grex = call_grex();
    grex.args(&["a", "b", "c"]);
    grex.args(&["-f", "/path/to/some/file"]);
    grex.assert().failure().stderr(predicate::str::contains(
        "argument '--file <FILE>' cannot be used with 'input'",
    ));
}

fn call_grex() -> Command {
    Command::cargo_bin("grex").unwrap()
}
