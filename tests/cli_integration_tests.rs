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

const TEST_CASE: &str = "I   ♥♥♥ 36 and ٣ and 💩💩.";

#[test]
fn succeeds_with_direct_input() {
    let mut grex = init_command();
    grex.args(&[TEST_CASE]);
    grex.assert()
        .success()
        .stdout(predicate::eq("^I   ♥♥♥ 36 and ٣ and 💩💩\\.$\n"));
}

#[test]
fn succeeds_with_escape_option() {
    let mut grex = init_command();
    grex.args(&["--escape", TEST_CASE]);
    grex.assert().success().stdout(predicate::eq(
        "^I   \\u{2665}\\u{2665}\\u{2665} 36 and \\u{663} and \\u{1f4a9}\\u{1f4a9}\\.$\n",
    ));
}

#[test]
fn succeeds_with_escape_and_surrogate_option() {
    let mut grex = init_command();
    grex.args(&["--escape", "--with-surrogates", TEST_CASE]);
    grex.assert().success().stdout(predicate::eq(
        "^I   \\u{2665}\\u{2665}\\u{2665} 36 and \\u{663} and \\u{d83d}\\u{dca9}\\u{d83d}\\u{dca9}\\.$\n",
    ));
}

#[test]
fn succeeds_with_digit_conversion_option() {
    let mut grex = init_command();
    grex.args(&["--digits", TEST_CASE]);
    grex.assert()
        .success()
        .stdout(predicate::eq("^I   ♥♥♥ \\d\\d and \\d and 💩💩\\.$\n"));
}

#[test]
fn succeeds_with_digit_conversion_and_escape_option() {
    let mut grex = init_command();
    grex.args(&["--digits", "--escape", TEST_CASE]);
    grex.assert().success().stdout(predicate::eq(
        "^I   \\u{2665}\\u{2665}\\u{2665} \\d\\d and \\d and \\u{1f4a9}\\u{1f4a9}\\.$\n",
    ));
}

#[test]
fn succeeds_with_digit_conversion_and_escape_and_surrogate_option() {
    let mut grex = init_command();
    grex.args(&["--digits", "--escape", "--with-surrogates", TEST_CASE]);
    grex.assert().success().stdout(predicate::eq(
        "^I   \\u{2665}\\u{2665}\\u{2665} \\d\\d and \\d and \\u{d83d}\\u{dca9}\\u{d83d}\\u{dca9}\\.$\n"
    ));
}

#[test]
fn succeeds_with_word_conversion_option() {
    let mut grex = init_command();
    grex.args(&["--words", TEST_CASE]);
    grex.assert().success().stdout(predicate::eq(
        "^\\w   ♥♥♥ \\w\\w \\w\\w\\w \\w \\w\\w\\w 💩💩\\.$\n",
    ));
}

#[test]
fn succeeds_with_word_conversion_and_escape_option() {
    let mut grex = init_command();
    grex.args(&["--words", "--escape", TEST_CASE]);
    grex.assert().success().stdout(predicate::eq(
        "^\\w   \\u{2665}\\u{2665}\\u{2665} \\w\\w \\w\\w\\w \\w \\w\\w\\w \\u{1f4a9}\\u{1f4a9}\\.$\n"
    ));
}

#[test]
fn succeeds_with_word_conversion_and_escape_and_surrogate_option() {
    let mut grex = init_command();
    grex.args(&["--words", "--escape", "--with-surrogates", TEST_CASE]);
    grex.assert().success().stdout(predicate::eq(
        "^\\w   \\u{2665}\\u{2665}\\u{2665} \\w\\w \\w\\w\\w \\w \\w\\w\\w \\u{d83d}\\u{dca9}\\u{d83d}\\u{dca9}\\.$\n"
    ));
}

#[test]
fn succeeds_with_space_conversion_option() {
    let mut grex = init_command();
    grex.args(&["--spaces", TEST_CASE]);
    grex.assert().success().stdout(predicate::eq(
        "^I\\s\\s\\s♥♥♥\\s36\\sand\\s٣\\sand\\s💩💩\\.$\n",
    ));
}

#[test]
fn succeeds_with_space_conversion_and_escape_option() {
    let mut grex = init_command();
    grex.args(&["--spaces", "--escape", TEST_CASE]);
    grex.assert().success().stdout(predicate::eq(
        "^I\\s\\s\\s\\u{2665}\\u{2665}\\u{2665}\\s36\\sand\\s\\u{663}\\sand\\s\\u{1f4a9}\\u{1f4a9}\\.$\n"
    ));
}

#[test]
fn succeeds_with_space_conversion_and_escape_and_surrogate_option() {
    let mut grex = init_command();
    grex.args(&["--spaces", "--escape", "--with-surrogates", TEST_CASE]);
    grex.assert().success().stdout(predicate::eq(
        "^I\\s\\s\\s\\u{2665}\\u{2665}\\u{2665}\\s36\\sand\\s\\u{663}\\sand\\s\\u{d83d}\\u{dca9}\\u{d83d}\\u{dca9}\\.$\n"
    ));
}

#[test]
fn succeeds_with_digit_and_word_conversion_option() {
    let mut grex = init_command();
    grex.args(&["--digits", "--words", TEST_CASE]);
    grex.assert().success().stdout(predicate::eq(
        "^\\w   ♥♥♥ \\d\\d \\w\\w\\w \\d \\w\\w\\w 💩💩\\.$\n",
    ));
}

#[test]
fn succeeds_with_digit_and_word_conversion_and_escape_option() {
    let mut grex = init_command();
    grex.args(&["--digits", "--words", "--escape", TEST_CASE]);
    grex.assert().success().stdout(predicate::eq(
        "^\\w   \\u{2665}\\u{2665}\\u{2665} \\d\\d \\w\\w\\w \\d \\w\\w\\w \\u{1f4a9}\\u{1f4a9}\\.$\n"
    ));
}

#[test]
fn succeeds_with_digit_and_word_conversion_and_escape_and_surrogate_option() {
    let mut grex = init_command();
    grex.args(&[
        "--digits",
        "--words",
        "--escape",
        "--with-surrogates",
        TEST_CASE,
    ]);
    grex.assert().success().stdout(predicate::eq(
        "^\\w   \\u{2665}\\u{2665}\\u{2665} \\d\\d \\w\\w\\w \\d \\w\\w\\w \\u{d83d}\\u{dca9}\\u{d83d}\\u{dca9}\\.$\n"
    ));
}

#[test]
fn succeeds_with_digit_and_space_conversion_option() {
    let mut grex = init_command();
    grex.args(&["--digits", "--spaces", TEST_CASE]);
    grex.assert().success().stdout(predicate::eq(
        "^I\\s\\s\\s♥♥♥\\s\\d\\d\\sand\\s\\d\\sand\\s💩💩\\.$\n",
    ));
}

#[test]
fn succeeds_with_digit_and_space_conversion_and_escape_option() {
    let mut grex = init_command();
    grex.args(&["--digits", "--spaces", "--escape", TEST_CASE]);
    grex.assert().success().stdout(predicate::eq(
        "^I\\s\\s\\s\\u{2665}\\u{2665}\\u{2665}\\s\\d\\d\\sand\\s\\d\\sand\\s\\u{1f4a9}\\u{1f4a9}\\.$\n"
    ));
}

#[test]
fn succeeds_with_digit_and_space_conversion_and_escape_and_surrogate_option() {
    let mut grex = init_command();
    grex.args(&[
        "--digits",
        "--spaces",
        "--escape",
        "--with-surrogates",
        TEST_CASE,
    ]);
    grex.assert().success().stdout(predicate::eq(
        "^I\\s\\s\\s\\u{2665}\\u{2665}\\u{2665}\\s\\d\\d\\sand\\s\\d\\sand\\s\\u{d83d}\\u{dca9}\\u{d83d}\\u{dca9}\\.$\n"
    ));
}

#[test]
fn succeeds_with_word_and_space_conversion_option() {
    let mut grex = init_command();
    grex.args(&["--words", "--spaces", TEST_CASE]);
    grex.assert().success().stdout(predicate::eq(
        "^\\w\\s\\s\\s♥♥♥\\s\\w\\w\\s\\w\\w\\w\\s\\w\\s\\w\\w\\w\\s💩💩\\.$\n",
    ));
}

#[test]
fn succeeds_with_word_and_space_conversion_and_escape_option() {
    let mut grex = init_command();
    grex.args(&["--words", "--spaces", "--escape", TEST_CASE]);
    grex.assert().success().stdout(predicate::eq(
        "^\\w\\s\\s\\s\\u{2665}\\u{2665}\\u{2665}\\s\\w\\w\\s\\w\\w\\w\\s\\w\\s\\w\\w\\w\\s\\u{1f4a9}\\u{1f4a9}\\.$\n"
    ));
}

#[test]
fn succeeds_with_word_and_space_conversion_and_escape_and_surrogate_option() {
    let mut grex = init_command();
    grex.args(&[
        "--words",
        "--spaces",
        "--escape",
        "--with-surrogates",
        TEST_CASE,
    ]);
    grex.assert().success().stdout(predicate::eq(
        "^\\w\\s\\s\\s\\u{2665}\\u{2665}\\u{2665}\\s\\w\\w\\s\\w\\w\\w\\s\\w\\s\\w\\w\\w\\s\\u{d83d}\\u{dca9}\\u{d83d}\\u{dca9}\\.$\n"
    ));
}

#[test]
fn succeeds_with_digit_and_word_and_space_conversion_option() {
    let mut grex = init_command();
    grex.args(&["--digits", "--words", "--spaces", TEST_CASE]);
    grex.assert().success().stdout(predicate::eq(
        "^\\w\\s\\s\\s♥♥♥\\s\\d\\d\\s\\w\\w\\w\\s\\d\\s\\w\\w\\w\\s💩💩\\.$\n",
    ));
}

#[test]
fn succeeds_with_digit_and_word_and_space_conversion_and_escape_option() {
    let mut grex = init_command();
    grex.args(&["--digits", "--words", "--spaces", "--escape", TEST_CASE]);
    grex.assert().success().stdout(predicate::eq(
        "^\\w\\s\\s\\s\\u{2665}\\u{2665}\\u{2665}\\s\\d\\d\\s\\w\\w\\w\\s\\d\\s\\w\\w\\w\\s\\u{1f4a9}\\u{1f4a9}\\.$\n"
    ));
}

#[test]
fn succeeds_with_digit_and_word_and_space_conversion_and_escape_and_surrogate_option() {
    let mut grex = init_command();
    grex.args(&[
        "--digits",
        "--words",
        "--spaces",
        "--escape",
        "--with-surrogates",
        TEST_CASE,
    ]);
    grex.assert().success().stdout(predicate::eq(
        "^\\w\\s\\s\\s\\u{2665}\\u{2665}\\u{2665}\\s\\d\\d\\s\\w\\w\\w\\s\\d\\s\\w\\w\\w\\s\\u{d83d}\\u{dca9}\\u{d83d}\\u{dca9}\\.$\n"
    ));
}

#[test]
fn succeeds_with_repetition_conversion_option() {
    let mut grex = init_command();
    grex.args(&["--repetitions", TEST_CASE]);
    grex.assert()
        .success()
        .stdout(predicate::eq("^I {3}♥{3} 36 and ٣ and 💩{2}\\.$\n"));
}

#[test]
fn succeeds_with_repetition_conversion_and_escape_option() {
    let mut grex = init_command();
    grex.args(&["--repetitions", "--escape", TEST_CASE]);
    grex.assert().success().stdout(predicate::eq(
        "^I {3}\\u{2665}{3} 36 and \\u{663} and \\u{1f4a9}{2}\\.$\n",
    ));
}

#[test]
fn succeeds_with_repetition_conversion_and_escape_and_surrogate_option() {
    let mut grex = init_command();
    grex.args(&["--repetitions", "--escape", "--with-surrogates", TEST_CASE]);
    grex.assert().success().stdout(predicate::eq(
        "^I {3}\\u{2665}{3} 36 and \\u{663} and (\\u{d83d}\\u{dca9}){2}\\.$\n",
    ));
}

#[test]
fn succeeds_with_repetition_and_digit_conversion_option() {
    let mut grex = init_command();
    grex.args(&["--repetitions", "--digits", TEST_CASE]);
    grex.assert()
        .success()
        .stdout(predicate::eq("^I {3}♥{3} \\d(\\d and ){2}💩{2}\\.$\n"));
}

#[test]
fn succeeds_with_repetition_and_digit_conversion_and_escape_option() {
    let mut grex = init_command();
    grex.args(&["--repetitions", "--digits", "--escape", TEST_CASE]);
    grex.assert().success().stdout(predicate::eq(
        "^I {3}\\u{2665}{3} \\d(\\d and ){2}\\u{1f4a9}{2}\\.$\n",
    ));
}

#[test]
fn succeeds_with_repetition_and_digit_conversion_and_escape_and_surrogate_option() {
    let mut grex = init_command();
    grex.args(&[
        "--repetitions",
        "--digits",
        "--escape",
        "--with-surrogates",
        TEST_CASE,
    ]);
    grex.assert().success().stdout(predicate::eq(
        "^I {3}\\u{2665}{3} \\d(\\d and ){2}(\\u{d83d}\\u{dca9}){2}\\.$\n",
    ));
}

#[test]
fn succeeds_with_repetition_and_word_conversion_option() {
    let mut grex = init_command();
    grex.args(&["--repetitions", "--words", TEST_CASE]);
    grex.assert()
        .success()
        .stdout(predicate::eq("^\\w {3}♥{3} \\w(\\w \\w{3} ){2}💩{2}\\.$\n"));
}

#[test]
fn succeeds_with_repetition_and_word_conversion_and_escape_option() {
    let mut grex = init_command();
    grex.args(&["--repetitions", "--words", "--escape", TEST_CASE]);
    grex.assert().success().stdout(predicate::eq(
        "^\\w {3}\\u{2665}{3} \\w(\\w \\w{3} ){2}\\u{1f4a9}{2}\\.$\n",
    ));
}

#[test]
fn succeeds_with_repetition_and_word_conversion_and_escape_and_surrogate_option() {
    let mut grex = init_command();
    grex.args(&[
        "--repetitions",
        "--words",
        "--escape",
        "--with-surrogates",
        TEST_CASE,
    ]);
    grex.assert().success().stdout(predicate::eq(
        "^\\w {3}\\u{2665}{3} \\w(\\w \\w{3} ){2}(\\u{d83d}\\u{dca9}){2}\\.$\n",
    ));
}

#[test]
fn succeeds_with_repetition_and_space_conversion_option() {
    let mut grex = init_command();
    grex.args(&["--repetitions", "--spaces", TEST_CASE]);
    grex.assert().success().stdout(predicate::eq(
        "^I\\s{3}♥{3}\\s36\\sand\\s٣\\sand\\s💩{2}\\.$\n",
    ));
}

#[test]
fn succeeds_with_repetition_and_space_conversion_and_escape_option() {
    let mut grex = init_command();
    grex.args(&["--repetitions", "--spaces", "--escape", TEST_CASE]);
    grex.assert().success().stdout(predicate::eq(
        "^I\\s{3}\\u{2665}{3}\\s36\\sand\\s\\u{663}\\sand\\s\\u{1f4a9}{2}\\.$\n",
    ));
}

#[test]
fn succeeds_with_repetition_and_space_conversion_and_escape_and_surrogate_option() {
    let mut grex = init_command();
    grex.args(&[
        "--repetitions",
        "--spaces",
        "--escape",
        "--with-surrogates",
        TEST_CASE,
    ]);
    grex.assert().success().stdout(predicate::eq(
        "^I\\s{3}\\u{2665}{3}\\s36\\sand\\s\\u{663}\\sand\\s(\\u{d83d}\\u{dca9}){2}\\.$\n",
    ));
}

#[test]
fn succeeds_with_repetition_and_digit_and_word_conversion_option() {
    let mut grex = init_command();
    grex.args(&["--repetitions", "--digits", "--words", TEST_CASE]);
    grex.assert()
        .success()
        .stdout(predicate::eq("^\\w {3}♥{3} \\d(\\d \\w{3} ){2}💩{2}\\.$\n"));
}

#[test]
fn succeeds_with_repetition_and_digit_and_word_conversion_and_escape_option() {
    let mut grex = init_command();
    grex.args(&[
        "--repetitions",
        "--digits",
        "--words",
        "--escape",
        TEST_CASE,
    ]);
    grex.assert().success().stdout(predicate::eq(
        "^\\w {3}\\u{2665}{3} \\d(\\d \\w{3} ){2}\\u{1f4a9}{2}\\.$\n",
    ));
}

#[test]
fn succeeds_with_repetition_and_digit_and_word_conversion_and_escape_and_surrogate_option() {
    let mut grex = init_command();
    grex.args(&[
        "--repetitions",
        "--digits",
        "--words",
        "--escape",
        "--with-surrogates",
        TEST_CASE,
    ]);
    grex.assert().success().stdout(predicate::eq(
        "^\\w {3}\\u{2665}{3} \\d(\\d \\w{3} ){2}(\\u{d83d}\\u{dca9}){2}\\.$\n",
    ));
}

#[test]
fn succeeds_with_repetition_and_digit_and_space_conversion_option() {
    let mut grex = init_command();
    grex.args(&["--repetitions", "--digits", "--spaces", TEST_CASE]);
    grex.assert().success().stdout(predicate::eq(
        "^I\\s{3}♥{3}\\s\\d(\\d\\sand\\s){2}💩{2}\\.$\n",
    ));
}

#[test]
fn succeeds_with_repetition_and_digit_and_space_conversion_and_escape_option() {
    let mut grex = init_command();
    grex.args(&[
        "--repetitions",
        "--digits",
        "--spaces",
        "--escape",
        TEST_CASE,
    ]);
    grex.assert().success().stdout(predicate::eq(
        "^I\\s{3}\\u{2665}{3}\\s\\d(\\d\\sand\\s){2}\\u{1f4a9}{2}\\.$\n",
    ));
}

#[test]
fn succeeds_with_repetition_and_digit_and_space_conversion_and_escape_and_surrogate_option() {
    let mut grex = init_command();
    grex.args(&[
        "--repetitions",
        "--digits",
        "--spaces",
        "--escape",
        "--with-surrogates",
        TEST_CASE,
    ]);
    grex.assert().success().stdout(predicate::eq(
        "^I\\s{3}\\u{2665}{3}\\s\\d(\\d\\sand\\s){2}(\\u{d83d}\\u{dca9}){2}\\.$\n",
    ));
}

#[test]
fn succeeds_with_repetition_and_word_and_space_conversion_option() {
    let mut grex = init_command();
    grex.args(&["--repetitions", "--words", "--spaces", TEST_CASE]);
    grex.assert().success().stdout(predicate::eq(
        "^\\w\\s{3}♥{3}\\s\\w(\\w\\s\\w{3}\\s){2}💩{2}\\.$\n",
    ));
}

#[test]
fn succeeds_with_repetition_and_word_and_space_conversion_and_escape_option() {
    let mut grex = init_command();
    grex.args(&[
        "--repetitions",
        "--words",
        "--spaces",
        "--escape",
        TEST_CASE,
    ]);
    grex.assert().success().stdout(predicate::eq(
        "^\\w\\s{3}\\u{2665}{3}\\s\\w(\\w\\s\\w{3}\\s){2}\\u{1f4a9}{2}\\.$\n",
    ));
}

#[test]
fn succeeds_with_repetition_and_word_and_space_conversion_and_escape_and_surrogate_option() {
    let mut grex = init_command();
    grex.args(&[
        "--repetitions",
        "--words",
        "--spaces",
        "--escape",
        "--with-surrogates",
        TEST_CASE,
    ]);
    grex.assert().success().stdout(predicate::eq(
        "^\\w\\s{3}\\u{2665}{3}\\s\\w(\\w\\s\\w{3}\\s){2}(\\u{d83d}\\u{dca9}){2}\\.$\n",
    ));
}

#[test]
fn succeeds_with_repetition_and_digit_and_word_and_space_conversion_option() {
    let mut grex = init_command();
    grex.args(&[
        "--repetitions",
        "--digits",
        "--words",
        "--spaces",
        TEST_CASE,
    ]);
    grex.assert().success().stdout(predicate::eq(
        "^\\w\\s{3}♥{3}\\s\\d(\\d\\s\\w{3}\\s){2}💩{2}\\.$\n",
    ));
}

#[test]
fn succeeds_with_repetition_and_digit_and_word_and_space_conversion_and_escape_option() {
    let mut grex = init_command();
    grex.args(&[
        "--repetitions",
        "--digits",
        "--words",
        "--spaces",
        "--escape",
        TEST_CASE,
    ]);
    grex.assert().success().stdout(predicate::eq(
        "^\\w\\s{3}\\u{2665}{3}\\s\\d(\\d\\s\\w{3}\\s){2}\\u{1f4a9}{2}\\.$\n",
    ));
}

#[test]
fn succeeds_with_repetition_and_digit_and_word_and_space_conversion_and_escape_and_surrogate_option(
) {
    let mut grex = init_command();
    grex.args(&[
        "--repetitions",
        "--digits",
        "--words",
        "--spaces",
        "--escape",
        "--with-surrogates",
        TEST_CASE,
    ]);
    grex.assert().success().stdout(predicate::eq(
        "^\\w\\s{3}\\u{2665}{3}\\s\\d(\\d\\s\\w{3}\\s){2}(\\u{d83d}\\u{dca9}){2}\\.$\n",
    ));
}

#[test]
fn fails_with_surrogate_but_without_escape_option() {
    let mut grex = init_command();
    grex.args(&["--with-surrogates", TEST_CASE]);
    grex.assert().failure().stderr(predicate::str::contains(
        "required arguments were not provided",
    ));
}

#[test]
#[allow(unused_must_use)]
fn succeeds_with_file_input() {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, "a\nb\\n\n\nc\näöü\n♥");

    let mut grex = init_command();
    grex.args(&["-f", file.path().to_str().unwrap()]);
    grex.assert()
        .success()
        .stdout(predicate::eq("^(b\\\\n|äöü|[ac♥])$\n"));
}

#[test]
fn fails_without_arguments() {
    let mut grex = init_command();
    grex.assert().failure().stderr(predicate::str::contains(
        "required arguments were not provided",
    ));
}

#[test]
fn fails_when_file_name_is_not_provided() {
    let mut grex = init_command();
    grex.arg("-f");
    grex.assert().failure().stderr(predicate::str::contains(
        "argument '--file <FILE>' requires a value but none was supplied",
    ));
}

#[test]
fn fails_when_file_does_not_exist() {
    let mut grex = init_command();
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
    let mut grex = init_command();
    grex.args(&[TEST_CASE]);
    grex.args(&["-f", "/path/to/some/file"]);
    grex.assert().failure().stderr(predicate::str::contains(
        "argument '--file <FILE>' cannot be used with 'input'",
    ));
}

fn init_command() -> Command {
    Command::cargo_bin("grex").unwrap()
}
