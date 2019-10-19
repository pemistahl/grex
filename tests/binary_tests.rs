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

use std::io::Write;
use std::process::Command;

use assert_cmd::prelude::*;
use predicates::prelude::*;
use tempfile::NamedTempFile;

#[test]
fn assert_that_grex_succeeds_with_direct_input() {
    let mut grex = call_grex();
    grex.args(&["a", "b", "c"]);
    grex.assert().success().stdout(predicate::eq("^[a-c]$\n"));
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
            "error: The file \"/path/to/non-existing/file\" could not be found\n",
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

fn call_grex() -> Command {
    Command::cargo_bin("grex").unwrap()
}
