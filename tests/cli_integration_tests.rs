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

use assert_cmd::Command;
use indoc::indoc;
use predicates::prelude::*;
use std::io::Write;
use tempfile::NamedTempFile;

const TEST_CASE: &str = "I   ♥♥♥ 36 and ٣ and y̆y̆ and 💩💩.";

mod no_conversion {
    use super::*;

    mod no_repetition {
        use super::*;

        #[test]
        fn succeeds() {
            let mut grex = init_command();
            grex.args(&[TEST_CASE]);
            grex.assert()
                .success()
                .stdout(predicate::eq("^I   ♥♥♥ 36 and ٣ and y̆y̆ and 💩💩\\.$\n"));
        }

        #[test]
        fn succeeds_with_ignore_case_option() {
            let mut grex = init_command();
            grex.args(&["--ignore-case", "Ä@Ö€Ü", "ä@ö€ü", "Ä@ö€Ü", "ä@Ö€ü"]);
            grex.assert()
                .success()
                .stdout(predicate::eq("(?i)^ä@ö€ü$\n"));
        }

        #[test]
        fn succeeds_with_leading_hyphen() {
            let mut grex = init_command();
            grex.args(&["-a", "b", "c"]);
            grex.assert()
                .success()
                .stdout(predicate::eq("^(?:\\-a|[bc])$\n"));
        }

        #[test]
        fn succeeds_with_escape_option() {
            let mut grex = init_command();
            grex.args(&["--escape", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(
                "^I   \\u{2665}\\u{2665}\\u{2665} 36 and \\u{663} and y\\u{306}y\\u{306} and \\u{1f4a9}\\u{1f4a9}\\.$\n",
            ));
        }

        #[test]
        fn succeeds_with_escape_and_surrogate_option() {
            let mut grex = init_command();
            grex.args(&["--escape", "--with-surrogates", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(
                "^I   \\u{2665}\\u{2665}\\u{2665} 36 and \\u{663} and y\\u{306}y\\u{306} and \\u{d83d}\\u{dca9}\\u{d83d}\\u{dca9}\\.$\n",
            ));
        }

        #[test]
        fn succeeds_with_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&["--verbose", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  I\ \ \ ♥♥♥\ 36\ and\ ٣\ and\ y̆y̆\ and\ 💩💩\.
                $
                "#,
            )));
        }

        #[test]
        fn succeeds_with_escape_and_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&["--escape", "--verbose", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  I\ \ \ \u{2665}\u{2665}\u{2665}\ 36\ and\ \u{663}\ and\ y\u{306}y\u{306}\ and\ \u{1f4a9}\u{1f4a9}\.
                $
                "#
            )));
        }

        #[test]
        fn succeeds_with_escape_and_surrogate_and_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&["--escape", "--with-surrogates", "--verbose", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  I\ \ \ \u{2665}\u{2665}\u{2665}\ 36\ and\ \u{663}\ and\ y\u{306}y\u{306}\ and\ \u{d83d}\u{dca9}\u{d83d}\u{dca9}\.
                $
                "#
            )));
        }

        #[test]
        fn succeeds_with_file_input() {
            let mut file = NamedTempFile::new().unwrap();
            writeln!(file, "a\nb\\n\n\nc\näöü\n♥").unwrap();

            let mut grex = init_command();
            grex.args(&["-f", file.path().to_str().unwrap()]);
            grex.assert()
                .success()
                .stdout(predicate::eq("^(?:b\\\\n|äöü|[ac♥])$\n"));
        }

        #[test]
        fn succeeds_with_test_cases_from_stdin() {
            let mut grex = init_command();
            grex.write_stdin("a\nb\\n\n\nc\näöü\n♥")
                .arg("-")
                .assert()
                .stdout(predicate::eq("^(?:b\\\\n|äöü|[ac♥])$\n"));
        }

        #[test]
        fn succeeds_with_file_from_stdin() {
            let mut file = NamedTempFile::new().unwrap();
            writeln!(file, "a\nb\\n\n\nc\näöü\n♥").unwrap();

            let mut grex = init_command();
            grex.write_stdin(file.path().to_str().unwrap())
                .args(&["-f", "-"])
                .assert()
                .stdout(predicate::eq("^(?:b\\\\n|äöü|[ac♥])$\n"));
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
                "a value is required for '--file <FILE>' but none was supplied",
            ));
        }

        #[test]
        fn fails_when_file_does_not_exist() {
            let mut grex = init_command();
            grex.args(&["-f", "/path/to/non-existing/file"]);
            grex.assert()
                .failure()
                .stdout(predicate::str::is_empty())
                .stderr(predicate::eq(
                    "error: the specified file could not be found\n",
                ));
        }

        #[test]
        fn fails_with_first_file_input_and_then_direct_input() {
            let mut grex = init_command();
            grex.args(&["-f", "/path/to/some/file", TEST_CASE]);
            grex.assert().failure().stderr(predicate::str::contains(
                "the argument '--file <FILE>' cannot be used with '[INPUT]...'",
            ));
        }
    }

    mod repetition {
        use super::*;

        #[test]
        fn succeeds() {
            let mut grex = init_command();
            grex.args(&["--repetitions", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(
                "^I {3}♥{3} 36 and ٣ and (?:y̆){2} and 💩{2}\\.$\n",
            ));
        }

        #[test]
        fn succeeds_with_ignore_case_option() {
            let mut grex = init_command();
            grex.args(&["--repetitions", "--ignore-case", "ÄÖÜäöü@Ö€", "äöüÄöÜ@ö€"]);
            grex.assert()
                .success()
                .stdout(predicate::eq("(?i)^(?:äöü){2}@ö€$\n"));
        }

        #[test]
        fn succeeds_with_escape_option() {
            let mut grex = init_command();
            grex.args(&["--repetitions", "--escape", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(
                "^I {3}\\u{2665}{3} 36 and \\u{663} and (?:y\\u{306}){2} and \\u{1f4a9}{2}\\.$\n",
            ));
        }

        #[test]
        fn succeeds_with_escape_and_surrogate_option() {
            let mut grex = init_command();
            grex.args(&["--repetitions", "--escape", "--with-surrogates", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(
                "^I {3}\\u{2665}{3} 36 and \\u{663} and (?:y\\u{306}){2} and (?:\\u{d83d}\\u{dca9}){2}\\.$\n",
            ));
        }

        #[test]
        fn succeeds_with_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&["--repetitions", "--verbose", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  I\ {3}♥{3}\ 36\ and\ ٣\ and\ 
                  (?:
                    y̆
                  ){2}
                  \ and\ 💩{2}\.
                $
                "#
            )));
        }

        #[test]
        fn succeeds_with_escape_and_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&["--repetitions", "--escape", "--verbose", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  I\ {3}\u{2665}{3}\ 36\ and\ \u{663}\ and\ 
                  (?:
                    y\u{306}
                  ){2}
                  \ and\ \u{1f4a9}{2}\.
                $
                "#
            )));
        }

        #[test]
        fn succeeds_with_escape_and_surrogate_and_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&[
                "--repetitions",
                "--escape",
                "--with-surrogates",
                "--verbose",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  I\ {3}\u{2665}{3}\ 36\ and\ \u{663}\ and\ 
                  (?:
                    y\u{306}
                  ){2}
                  \ and\ 
                  (?:
                    \u{d83d}\u{dca9}
                  ){2}
                  \.
                $
                "#
            )));
        }

        #[test]
        fn succeeds_with_increased_minimum_repetitions() {
            let mut grex = init_command();
            grex.args(&["--repetitions", "--min-repetitions", "2", TEST_CASE]);
            grex.assert()
                .success()
                .stdout(predicate::eq("^I {3}♥{3} 36 and ٣ and y̆y̆ and 💩💩\\.$\n"));
        }

        #[test]
        fn succeeds_with_increased_minimum_substring_length() {
            let mut grex = init_command();
            grex.args(&["--repetitions", "--min-substring-length", "2", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(
                "^I   ♥♥♥ 36 and ٣ and (?:y̆){2} and 💩💩\\.$\n",
            ));
        }

        #[test]
        fn fails_with_minimum_repetitions_equal_to_zero() {
            let mut grex = init_command();
            grex.args(&["--min-repetitions", "0", TEST_CASE]);
            grex.assert()
                .failure()
                .stderr(predicate::str::contains("Value must not be zero"));
        }

        #[test]
        fn fails_with_minimum_repetitions_equal_to_invalid_value() {
            let mut grex = init_command();
            grex.args(&["--min-repetitions", "§!$", TEST_CASE]);
            grex.assert().failure().stderr(predicate::str::contains(
                "Value is not a valid unsigned integer",
            ));
        }

        #[test]
        fn fails_with_minimum_substring_length_equal_to_zero() {
            let mut grex = init_command();
            grex.args(&["--min-substring-length", "0", TEST_CASE]);
            grex.assert()
                .failure()
                .stderr(predicate::str::contains("Value must not be zero"));
        }

        #[test]
        fn fails_with_minimum_substring_length_equal_to_invalid_value() {
            let mut grex = init_command();
            grex.args(&["--min-substring-length", "§!$", TEST_CASE]);
            grex.assert().failure().stderr(predicate::str::contains(
                "Value is not a valid unsigned integer",
            ));
        }
    }
}

mod digit_conversion {
    use super::*;

    mod no_repetition {
        use super::*;

        #[test]
        fn succeeds() {
            let mut grex = init_command();
            grex.args(&["--digits", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(
                "^I   ♥♥♥ \\d\\d and \\d and y̆y̆ and 💩💩\\.$\n",
            ));
        }

        #[test]
        fn succeeds_with_escape_option() {
            let mut grex = init_command();
            grex.args(&["--digits", "--escape", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(
                "^I   \\u{2665}\\u{2665}\\u{2665} \\d\\d and \\d and y\\u{306}y\\u{306} and \\u{1f4a9}\\u{1f4a9}\\.$\n",
            ));
        }

        #[test]
        fn succeeds_with_escape_and_surrogate_option() {
            let mut grex = init_command();
            grex.args(&["--digits", "--escape", "--with-surrogates", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(
                "^I   \\u{2665}\\u{2665}\\u{2665} \\d\\d and \\d and y\\u{306}y\\u{306} and \\u{d83d}\\u{dca9}\\u{d83d}\\u{dca9}\\.$\n"
            ));
        }

        #[test]
        fn succeeds_with_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&["--digits", "--verbose", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  I\ \ \ ♥♥♥\ \d\d\ and\ \d\ and\ y̆y̆\ and\ 💩💩\.
                $
                "#
            )));
        }

        #[test]
        fn succeeds_with_escape_and_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&["--digits", "--escape", "--verbose", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  I\ \ \ \u{2665}\u{2665}\u{2665}\ \d\d\ and\ \d\ and\ y\u{306}y\u{306}\ and\ \u{1f4a9}\u{1f4a9}\.
                $
                "#
            )));
        }

        #[test]
        fn succeeds_with_escape_and_surrogate_and_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&[
                "--digits",
                "--escape",
                "--with-surrogates",
                "--verbose",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  I\ \ \ \u{2665}\u{2665}\u{2665}\ \d\d\ and\ \d\ and\ y\u{306}y\u{306}\ and\ \u{d83d}\u{dca9}\u{d83d}\u{dca9}\.
                $
                "#
            )));
        }

        #[test]
        fn succeeds_with_capturing_groups_option() {
            let mut grex = init_command();
            grex.args(&["--capture-groups", "abc", "def"]);
            grex.assert()
                .success()
                .stdout(predicate::eq("^(abc|def)$\n"));
        }

        #[test]
        fn succeeds_with_syntax_highlighting() {
            let mut grex = init_command();
            grex.args(&["--colorize", "abc", "def"]);
            grex.assert()
                .success()
                .stdout(predicate::eq("\u{1b}[1;33m^\u{1b}[0m\u{1b}[1;32m(?:\u{1b}[0mabc\u{1b}[1;31m|\u{1b}[0mdef\u{1b}[1;32m)\u{1b}[0m\u{1b}[1;33m$\u{1b}[0m\n"));
        }
    }

    mod repetition {
        use super::*;

        #[test]
        fn succeeds() {
            let mut grex = init_command();
            grex.args(&["--repetitions", "--digits", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(
                "^I {3}♥{3} \\d(?:\\d and ){2}(?:y̆){2} and 💩{2}\\.$\n",
            ));
        }

        #[test]
        fn succeeds_with_escape_option() {
            let mut grex = init_command();
            grex.args(&["--repetitions", "--digits", "--escape", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(
                "^I {3}\\u{2665}{3} \\d(?:\\d and ){2}(?:y\\u{306}){2} and \\u{1f4a9}{2}\\.$\n",
            ));
        }

        #[test]
        fn succeeds_with_escape_and_surrogate_option() {
            let mut grex = init_command();
            grex.args(&[
                "--repetitions",
                "--digits",
                "--escape",
                "--with-surrogates",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(
                "^I {3}\\u{2665}{3} \\d(?:\\d and ){2}(?:y\\u{306}){2} and (?:\\u{d83d}\\u{dca9}){2}\\.$\n",
            ));
        }

        #[test]
        fn succeeds_with_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&["--repetitions", "--digits", "--verbose", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  I\ {3}♥{3}\ \d
                  (?:
                    \d\ and\ 
                  ){2}
                  (?:
                    y̆
                  ){2}
                  \ and\ 💩{2}\.
                $
                "#
            )));
        }

        #[test]
        fn succeeds_with_escape_and_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&[
                "--repetitions",
                "--digits",
                "--escape",
                "--verbose",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  I\ {3}\u{2665}{3}\ \d
                  (?:
                    \d\ and\ 
                  ){2}
                  (?:
                    y\u{306}
                  ){2}
                  \ and\ \u{1f4a9}{2}\.
                $
                "#
            )));
        }

        #[test]
        fn succeeds_with_escape_and_surrogate_and_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&[
                "--repetitions",
                "--digits",
                "--escape",
                "--with-surrogates",
                "--verbose",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  I\ {3}\u{2665}{3}\ \d
                  (?:
                    \d\ and\ 
                  ){2}
                  (?:
                    y\u{306}
                  ){2}
                  \ and\ 
                  (?:
                    \u{d83d}\u{dca9}
                  ){2}
                  \.
                $
                "#
            )));
        }

        #[test]
        fn succeeds_with_increased_minimum_repetitions() {
            let mut grex = init_command();
            grex.args(&[
                "--repetitions",
                "--min-repetitions",
                "2",
                "--digits",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(
                "^I {3}♥{3} \\d\\d and \\d and y̆y̆ and 💩💩\\.$\n",
            ));
        }

        #[test]
        fn succeeds_with_increased_minimum_substring_length() {
            let mut grex = init_command();
            grex.args(&[
                "--repetitions",
                "--min-substring-length",
                "2",
                "--digits",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(
                "^I   ♥♥♥ \\d(?:\\d and ){2}(?:y̆){2} and 💩💩\\.$\n",
            ));
        }
    }
}

mod space_conversion {
    use super::*;

    mod no_repetition {
        use super::*;

        #[test]
        fn succeeds() {
            let mut grex = init_command();
            grex.args(&["--spaces", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(
                "^I\\s\\s\\s♥♥♥\\s36\\sand\\s٣\\sand\\sy̆y̆\\sand\\s💩💩\\.$\n",
            ));
        }

        #[test]
        fn succeeds_with_escape_option() {
            let mut grex = init_command();
            grex.args(&["--spaces", "--escape", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(
                "^I\\s\\s\\s\\u{2665}\\u{2665}\\u{2665}\\s36\\sand\\s\\u{663}\\sand\\sy\\u{306}y\\u{306}\\sand\\s\\u{1f4a9}\\u{1f4a9}\\.$\n"
            ));
        }

        #[test]
        fn succeeds_with_escape_and_surrogate_option() {
            let mut grex = init_command();
            grex.args(&["--spaces", "--escape", "--with-surrogates", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(
                "^I\\s\\s\\s\\u{2665}\\u{2665}\\u{2665}\\s36\\sand\\s\\u{663}\\sand\\sy\\u{306}y\\u{306}\\sand\\s\\u{d83d}\\u{dca9}\\u{d83d}\\u{dca9}\\.$\n"
            ));
        }

        #[test]
        fn succeeds_with_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&["--spaces", "--verbose", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  I\s\s\s♥♥♥\s36\sand\s٣\sand\sy̆y̆\sand\s💩💩\.
                $
                "#
            )));
        }

        #[test]
        fn succeeds_with_escape_and_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&["--spaces", "--escape", "--verbose", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  I\s\s\s\u{2665}\u{2665}\u{2665}\s36\sand\s\u{663}\sand\sy\u{306}y\u{306}\sand\s\u{1f4a9}\u{1f4a9}\.
                $
                "#
            )));
        }

        #[test]
        fn succeeds_with_escape_and_surrogate_and_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&[
                "--spaces",
                "--escape",
                "--with-surrogates",
                "--verbose",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  I\s\s\s\u{2665}\u{2665}\u{2665}\s36\sand\s\u{663}\sand\sy\u{306}y\u{306}\sand\s\u{d83d}\u{dca9}\u{d83d}\u{dca9}\.
                $
                "#
            )));
        }
    }

    mod repetition {
        use super::*;

        #[test]
        fn succeeds() {
            let mut grex = init_command();
            grex.args(&["--repetitions", "--spaces", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(
                "^I\\s{3}♥{3}\\s36\\sand\\s٣\\sand\\s(?:y̆){2}\\sand\\s💩{2}\\.$\n",
            ));
        }

        #[test]
        fn succeeds_with_escape_option() {
            let mut grex = init_command();
            grex.args(&["--repetitions", "--spaces", "--escape", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(
                "^I\\s{3}\\u{2665}{3}\\s36\\sand\\s\\u{663}\\sand\\s(?:y\\u{306}){2}\\sand\\s\\u{1f4a9}{2}\\.$\n",
            ));
        }

        #[test]
        fn succeeds_with_escape_and_surrogate_option() {
            let mut grex = init_command();
            grex.args(&[
                "--repetitions",
                "--spaces",
                "--escape",
                "--with-surrogates",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(
                "^I\\s{3}\\u{2665}{3}\\s36\\sand\\s\\u{663}\\sand\\s(?:y\\u{306}){2}\\sand\\s(?:\\u{d83d}\\u{dca9}){2}\\.$\n",
            ));
        }

        #[test]
        fn succeeds_with_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&["--repetitions", "--spaces", "--verbose", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  I\s{3}♥{3}\s36\sand\s٣\sand\s
                  (?:
                    y̆
                  ){2}
                  \sand\s💩{2}\.
                $
                "#
            )));
        }

        #[test]
        fn succeeds_with_escape_and_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&[
                "--repetitions",
                "--spaces",
                "--escape",
                "--verbose",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  I\s{3}\u{2665}{3}\s36\sand\s\u{663}\sand\s
                  (?:
                    y\u{306}
                  ){2}
                  \sand\s\u{1f4a9}{2}\.
                $
                "#
            )));
        }

        #[test]
        fn succeeds_with_escape_and_surrogate_and_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&[
                "--repetitions",
                "--spaces",
                "--escape",
                "--with-surrogates",
                "--verbose",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  I\s{3}\u{2665}{3}\s36\sand\s\u{663}\sand\s
                  (?:
                    y\u{306}
                  ){2}
                  \sand\s
                  (?:
                    \u{d83d}\u{dca9}
                  ){2}
                  \.
                $
                "#
            )));
        }
    }
}

mod word_conversion {
    use super::*;

    mod no_repetition {
        use super::*;

        #[test]
        fn succeeds() {
            let mut grex = init_command();
            grex.args(&["--words", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(
                "^\\w   ♥♥♥ \\w\\w \\w\\w\\w \\w \\w\\w\\w \\w\\w\\w\\w \\w\\w\\w 💩💩\\.$\n",
            ));
        }

        #[test]
        fn succeeds_with_escape_option() {
            let mut grex = init_command();
            grex.args(&["--words", "--escape", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(
                "^\\w   \\u{2665}\\u{2665}\\u{2665} \\w\\w \\w\\w\\w \\w \\w\\w\\w \\w\\w\\w\\w \\w\\w\\w \\u{1f4a9}\\u{1f4a9}\\.$\n"
            ));
        }

        #[test]
        fn succeeds_with_escape_and_surrogate_option() {
            let mut grex = init_command();
            grex.args(&["--words", "--escape", "--with-surrogates", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(
                "^\\w   \\u{2665}\\u{2665}\\u{2665} \\w\\w \\w\\w\\w \\w \\w\\w\\w \\w\\w\\w\\w \\w\\w\\w \\u{d83d}\\u{dca9}\\u{d83d}\\u{dca9}\\.$\n"
            ));
        }

        #[test]
        fn succeeds_with_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&["--words", "--verbose", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  \w\ \ \ ♥♥♥\ \w\w\ \w\w\w\ \w\ \w\w\w\ \w\w\w\w\ \w\w\w\ 💩💩\.
                $
                "#
            )));
        }

        #[test]
        fn succeeds_with_escape_and_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&["--words", "--escape", "--verbose", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  \w\ \ \ \u{2665}\u{2665}\u{2665}\ \w\w\ \w\w\w\ \w\ \w\w\w\ \w\w\w\w\ \w\w\w\ \u{1f4a9}\u{1f4a9}\.
                $
                "#
            )));
        }

        #[test]
        fn succeeds_with_escape_and_surrogate_and_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&[
                "--words",
                "--escape",
                "--with-surrogates",
                "--verbose",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  \w\ \ \ \u{2665}\u{2665}\u{2665}\ \w\w\ \w\w\w\ \w\ \w\w\w\ \w\w\w\w\ \w\w\w\ \u{d83d}\u{dca9}\u{d83d}\u{dca9}\.
                $
                "#
            )));
        }
    }

    mod repetition {
        use super::*;

        #[test]
        fn succeeds() {
            let mut grex = init_command();
            grex.args(&["--repetitions", "--words", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(
                "^\\w {3}♥{3} \\w{2}(?: \\w{3} \\w){2}(?:\\w{3} ){2}💩{2}\\.$\n",
            ));
        }

        #[test]
        fn succeeds_with_escape_option() {
            let mut grex = init_command();
            grex.args(&["--repetitions", "--words", "--escape", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(
                "^\\w {3}\\u{2665}{3} \\w{2}(?: \\w{3} \\w){2}(?:\\w{3} ){2}\\u{1f4a9}{2}\\.$\n",
            ));
        }

        #[test]
        fn succeeds_with_escape_and_surrogate_option() {
            let mut grex = init_command();
            grex.args(&[
                "--repetitions",
                "--words",
                "--escape",
                "--with-surrogates",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(
                "^\\w {3}\\u{2665}{3} \\w{2}(?: \\w{3} \\w){2}(?:\\w{3} ){2}(?:\\u{d83d}\\u{dca9}){2}\\.$\n",
            ));
        }

        #[test]
        fn succeeds_with_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&["--repetitions", "--words", "--verbose", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  \w\ {3}♥{3}\ \w{2}
                  (?:
                    \ \w{3}\ \w
                  ){2}
                  (?:
                    \w{3}\ 
                  ){2}
                  💩{2}\.
                $
                "#
            )));
        }

        #[test]
        fn succeeds_with_escape_and_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&[
                "--repetitions",
                "--words",
                "--escape",
                "--verbose",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  \w\ {3}\u{2665}{3}\ \w{2}
                  (?:
                    \ \w{3}\ \w
                  ){2}
                  (?:
                    \w{3}\ 
                  ){2}
                  \u{1f4a9}{2}\.
                $
                "#
            )));
        }

        #[test]
        fn succeeds_with_escape_and_surrogate_and_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&[
                "--repetitions",
                "--words",
                "--escape",
                "--with-surrogates",
                "--verbose",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  \w\ {3}\u{2665}{3}\ \w{2}
                  (?:
                    \ \w{3}\ \w
                  ){2}
                  (?:
                    \w{3}\ 
                  ){2}
                  (?:
                    \u{d83d}\u{dca9}
                  ){2}
                  \.
                $
                "#
            )));
        }
    }
}

mod digit_space_conversion {
    use super::*;

    mod no_repetition {
        use super::*;

        #[test]
        fn succeeds() {
            let mut grex = init_command();
            grex.args(&["--digits", "--spaces", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(
                "^I\\s\\s\\s♥♥♥\\s\\d\\d\\sand\\s\\d\\sand\\sy̆y̆\\sand\\s💩💩\\.$\n",
            ));
        }

        #[test]
        fn succeeds_with_escape_option() {
            let mut grex = init_command();
            grex.args(&["--digits", "--spaces", "--escape", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(
                "^I\\s\\s\\s\\u{2665}\\u{2665}\\u{2665}\\s\\d\\d\\sand\\s\\d\\sand\\sy\\u{306}y\\u{306}\\sand\\s\\u{1f4a9}\\u{1f4a9}\\.$\n"
            ));
        }

        #[test]
        fn succeeds_with_escape_and_surrogate_option() {
            let mut grex = init_command();
            grex.args(&[
                "--digits",
                "--spaces",
                "--escape",
                "--with-surrogates",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(
                "^I\\s\\s\\s\\u{2665}\\u{2665}\\u{2665}\\s\\d\\d\\sand\\s\\d\\sand\\sy\\u{306}y\\u{306}\\sand\\s\\u{d83d}\\u{dca9}\\u{d83d}\\u{dca9}\\.$\n"
            ));
        }

        #[test]
        fn succeeds_with_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&["--digits", "--spaces", "--verbose", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  I\s\s\s♥♥♥\s\d\d\sand\s\d\sand\sy̆y̆\sand\s💩💩\.
                $
                "#
            )));
        }

        #[test]
        fn succeeds_with_escape_and_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&["--digits", "--spaces", "--escape", "--verbose", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  I\s\s\s\u{2665}\u{2665}\u{2665}\s\d\d\sand\s\d\sand\sy\u{306}y\u{306}\sand\s\u{1f4a9}\u{1f4a9}\.
                $
                "#
            )));
        }

        #[test]
        fn succeeds_with_escape_and_surrogate_and_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&[
                "--digits",
                "--spaces",
                "--escape",
                "--with-surrogates",
                "--verbose",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  I\s\s\s\u{2665}\u{2665}\u{2665}\s\d\d\sand\s\d\sand\sy\u{306}y\u{306}\sand\s\u{d83d}\u{dca9}\u{d83d}\u{dca9}\.
                $
                "#
            )));
        }
    }

    mod repetition {
        use super::*;

        #[test]
        fn succeeds() {
            let mut grex = init_command();
            grex.args(&["--repetitions", "--digits", "--spaces", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(
                "^I\\s{3}♥{3}\\s\\d(?:\\d\\sand\\s){2}(?:y̆){2}\\sand\\s💩{2}\\.$\n",
            ));
        }

        #[test]
        fn succeeds_with_escape_option() {
            let mut grex = init_command();
            grex.args(&[
                "--repetitions",
                "--digits",
                "--spaces",
                "--escape",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(
                "^I\\s{3}\\u{2665}{3}\\s\\d(?:\\d\\sand\\s){2}(?:y\\u{306}){2}\\sand\\s\\u{1f4a9}{2}\\.$\n",
            ));
        }

        #[test]
        fn succeeds_with_escape_and_surrogate_option() {
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
                "^I\\s{3}\\u{2665}{3}\\s\\d(?:\\d\\sand\\s){2}(?:y\\u{306}){2}\\sand\\s(?:\\u{d83d}\\u{dca9}){2}\\.$\n",
            ));
        }

        #[test]
        fn succeeds_with_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&[
                "--repetitions",
                "--digits",
                "--spaces",
                "--verbose",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  I\s{3}♥{3}\s\d
                  (?:
                    \d\sand\s
                  ){2}
                  (?:
                    y̆
                  ){2}
                  \sand\s💩{2}\.
                $
                "#
            )));
        }

        #[test]
        fn succeeds_with_escape_and_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&[
                "--repetitions",
                "--digits",
                "--spaces",
                "--escape",
                "--verbose",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  I\s{3}\u{2665}{3}\s\d
                  (?:
                    \d\sand\s
                  ){2}
                  (?:
                    y\u{306}
                  ){2}
                  \sand\s\u{1f4a9}{2}\.
                $
                "#
            )));
        }

        #[test]
        fn succeeds_with_escape_and_surrogate_and_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&[
                "--repetitions",
                "--digits",
                "--spaces",
                "--escape",
                "--with-surrogates",
                "--verbose",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  I\s{3}\u{2665}{3}\s\d
                  (?:
                    \d\sand\s
                  ){2}
                  (?:
                    y\u{306}
                  ){2}
                  \sand\s
                  (?:
                    \u{d83d}\u{dca9}
                  ){2}
                  \.
                $
                "#
            )));
        }
    }
}

mod digit_word_conversion {
    use super::*;

    mod no_repetition {
        use super::*;

        #[test]
        fn succeeds() {
            let mut grex = init_command();
            grex.args(&["--digits", "--words", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(
                "^\\w   ♥♥♥ \\d\\d \\w\\w\\w \\d \\w\\w\\w \\w\\w\\w\\w \\w\\w\\w 💩💩\\.$\n",
            ));
        }

        #[test]
        fn succeeds_with_escape_option() {
            let mut grex = init_command();
            grex.args(&["--digits", "--words", "--escape", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(
                "^\\w   \\u{2665}\\u{2665}\\u{2665} \\d\\d \\w\\w\\w \\d \\w\\w\\w \\w\\w\\w\\w \\w\\w\\w \\u{1f4a9}\\u{1f4a9}\\.$\n"
            ));
        }

        #[test]
        fn succeeds_with_escape_and_surrogate_option() {
            let mut grex = init_command();
            grex.args(&[
                "--digits",
                "--words",
                "--escape",
                "--with-surrogates",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(
                "^\\w   \\u{2665}\\u{2665}\\u{2665} \\d\\d \\w\\w\\w \\d \\w\\w\\w \\w\\w\\w\\w \\w\\w\\w \\u{d83d}\\u{dca9}\\u{d83d}\\u{dca9}\\.$\n"
            ));
        }

        #[test]
        fn succeeds_with_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&["--digits", "--words", "--verbose", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  \w\ \ \ ♥♥♥\ \d\d\ \w\w\w\ \d\ \w\w\w\ \w\w\w\w\ \w\w\w\ 💩💩\.
                $
                "#
            )));
        }

        #[test]
        fn succeeds_with_escape_and_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&["--digits", "--words", "--escape", "--verbose", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  \w\ \ \ \u{2665}\u{2665}\u{2665}\ \d\d\ \w\w\w\ \d\ \w\w\w\ \w\w\w\w\ \w\w\w\ \u{1f4a9}\u{1f4a9}\.
                $
                "#
            )));
        }

        #[test]
        fn succeeds_with_escape_and_surrogate_and_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&[
                "--digits",
                "--words",
                "--escape",
                "--with-surrogates",
                "--verbose",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  \w\ \ \ \u{2665}\u{2665}\u{2665}\ \d\d\ \w\w\w\ \d\ \w\w\w\ \w\w\w\w\ \w\w\w\ \u{d83d}\u{dca9}\u{d83d}\u{dca9}\.
                $
                "#
            )));
        }
    }

    mod repetition {
        use super::*;

        #[test]
        fn succeeds() {
            let mut grex = init_command();
            grex.args(&["--repetitions", "--digits", "--words", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(
                "^\\w {3}♥{3} \\d(?:\\d \\w{3} ){2}\\w(?:\\w{3} ){2}💩{2}\\.$\n",
            ));
        }

        #[test]
        fn succeeds_with_escape_option() {
            let mut grex = init_command();
            grex.args(&[
                "--repetitions",
                "--digits",
                "--words",
                "--escape",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(
                "^\\w {3}\\u{2665}{3} \\d(?:\\d \\w{3} ){2}\\w(?:\\w{3} ){2}\\u{1f4a9}{2}\\.$\n",
            ));
        }

        #[test]
        fn succeeds_with_escape_and_surrogate_option() {
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
                "^\\w {3}\\u{2665}{3} \\d(?:\\d \\w{3} ){2}\\w(?:\\w{3} ){2}(?:\\u{d83d}\\u{dca9}){2}\\.$\n",
            ));
        }

        #[test]
        fn succeeds_with_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&[
                "--repetitions",
                "--digits",
                "--words",
                "--verbose",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  \w\ {3}♥{3}\ \d
                  (?:
                    \d\ \w{3}\ 
                  ){2}
                  \w
                  (?:
                    \w{3}\ 
                  ){2}
                  💩{2}\.
                $
                "#
            )));
        }

        #[test]
        fn succeeds_with_escape_and_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&[
                "--repetitions",
                "--digits",
                "--words",
                "--escape",
                "--verbose",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  \w\ {3}\u{2665}{3}\ \d
                  (?:
                    \d\ \w{3}\ 
                  ){2}
                  \w
                  (?:
                    \w{3}\ 
                  ){2}
                  \u{1f4a9}{2}\.
                $
                "#
            )));
        }

        #[test]
        fn succeeds_with_escape_and_surrogate_and_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&[
                "--repetitions",
                "--digits",
                "--words",
                "--escape",
                "--with-surrogates",
                "--verbose",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  \w\ {3}\u{2665}{3}\ \d
                  (?:
                    \d\ \w{3}\ 
                  ){2}
                  \w
                  (?:
                    \w{3}\ 
                  ){2}
                  (?:
                    \u{d83d}\u{dca9}
                  ){2}
                  \.
                $
                "#
            )));
        }
    }
}

mod space_word_conversion {
    use super::*;

    mod no_repetition {
        use super::*;

        #[test]
        fn succeeds() {
            let mut grex = init_command();
            grex.args(&["--words", "--spaces", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(
                "^\\w\\s\\s\\s♥♥♥\\s\\w\\w\\s\\w\\w\\w\\s\\w\\s\\w\\w\\w\\s\\w\\w\\w\\w\\s\\w\\w\\w\\s💩💩\\.$\n",
            ));
        }

        #[test]
        fn succeeds_with_escape_option() {
            let mut grex = init_command();
            grex.args(&["--words", "--spaces", "--escape", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(
                "^\\w\\s\\s\\s\\u{2665}\\u{2665}\\u{2665}\\s\\w\\w\\s\\w\\w\\w\\s\\w\\s\\w\\w\\w\\s\\w\\w\\w\\w\\s\\w\\w\\w\\s\\u{1f4a9}\\u{1f4a9}\\.$\n"
            ));
        }

        #[test]
        fn succeeds_with_escape_and_surrogate_option() {
            let mut grex = init_command();
            grex.args(&[
                "--words",
                "--spaces",
                "--escape",
                "--with-surrogates",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(
                "^\\w\\s\\s\\s\\u{2665}\\u{2665}\\u{2665}\\s\\w\\w\\s\\w\\w\\w\\s\\w\\s\\w\\w\\w\\s\\w\\w\\w\\w\\s\\w\\w\\w\\s\\u{d83d}\\u{dca9}\\u{d83d}\\u{dca9}\\.$\n"
            ));
        }

        #[test]
        fn succeeds_with_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&["--words", "--spaces", "--verbose", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  \w\s\s\s♥♥♥\s\w\w\s\w\w\w\s\w\s\w\w\w\s\w\w\w\w\s\w\w\w\s💩💩\.
                $
                "#
            )));
        }

        #[test]
        fn succeeds_with_escape_and_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&["--words", "--spaces", "--escape", "--verbose", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  \w\s\s\s\u{2665}\u{2665}\u{2665}\s\w\w\s\w\w\w\s\w\s\w\w\w\s\w\w\w\w\s\w\w\w\s\u{1f4a9}\u{1f4a9}\.
                $
                "#
            )));
        }

        #[test]
        fn succeeds_with_escape_and_surrogate_and_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&[
                "--words",
                "--spaces",
                "--escape",
                "--with-surrogates",
                "--verbose",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  \w\s\s\s\u{2665}\u{2665}\u{2665}\s\w\w\s\w\w\w\s\w\s\w\w\w\s\w\w\w\w\s\w\w\w\s\u{d83d}\u{dca9}\u{d83d}\u{dca9}\.
                $
                "#
            )));
        }
    }

    mod repetition {
        use super::*;

        #[test]
        fn succeeds() {
            let mut grex = init_command();
            grex.args(&["--repetitions", "--words", "--spaces", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(
                "^\\w\\s{3}♥{3}\\s\\w{2}(?:\\s\\w{3}\\s\\w){2}(?:\\w{3}\\s){2}💩{2}\\.$\n",
            ));
        }

        #[test]
        fn succeeds_with_escape_option() {
            let mut grex = init_command();
            grex.args(&[
                "--repetitions",
                "--words",
                "--spaces",
                "--escape",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(
                "^\\w\\s{3}\\u{2665}{3}\\s\\w{2}(?:\\s\\w{3}\\s\\w){2}(?:\\w{3}\\s){2}\\u{1f4a9}{2}\\.$\n",
            ));
        }

        #[test]
        fn succeeds_with_escape_and_surrogate_option() {
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
                "^\\w\\s{3}\\u{2665}{3}\\s\\w{2}(?:\\s\\w{3}\\s\\w){2}(?:\\w{3}\\s){2}(?:\\u{d83d}\\u{dca9}){2}\\.$\n",
            ));
        }

        #[test]
        fn succeeds_with_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&[
                "--repetitions",
                "--words",
                "--spaces",
                "--verbose",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  \w\s{3}♥{3}\s\w{2}
                  (?:
                    \s\w{3}\s\w
                  ){2}
                  (?:
                    \w{3}\s
                  ){2}
                  💩{2}\.
                $
                "#
            )));
        }

        #[test]
        fn succeeds_with_escape_and_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&[
                "--repetitions",
                "--words",
                "--spaces",
                "--escape",
                "--verbose",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  \w\s{3}\u{2665}{3}\s\w{2}
                  (?:
                    \s\w{3}\s\w
                  ){2}
                  (?:
                    \w{3}\s
                  ){2}
                  \u{1f4a9}{2}\.
                $
                "#
            )));
        }

        #[test]
        fn succeeds_with_escape_and_surrogate_and_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&[
                "--repetitions",
                "--words",
                "--spaces",
                "--escape",
                "--with-surrogates",
                "--verbose",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  \w\s{3}\u{2665}{3}\s\w{2}
                  (?:
                    \s\w{3}\s\w
                  ){2}
                  (?:
                    \w{3}\s
                  ){2}
                  (?:
                    \u{d83d}\u{dca9}
                  ){2}
                  \.
                $
                "#
            )));
        }
    }
}

mod digit_space_word_conversion {
    use super::*;

    mod no_repetition {
        use super::*;

        #[test]
        fn succeeds() {
            let mut grex = init_command();
            grex.args(&["--digits", "--words", "--spaces", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(
                "^\\w\\s\\s\\s♥♥♥\\s\\d\\d\\s\\w\\w\\w\\s\\d\\s\\w\\w\\w\\s\\w\\w\\w\\w\\s\\w\\w\\w\\s💩💩\\.$\n",
            ));
        }

        #[test]
        fn succeeds_with_escape_option() {
            let mut grex = init_command();
            grex.args(&["--digits", "--words", "--spaces", "--escape", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(
                "^\\w\\s\\s\\s\\u{2665}\\u{2665}\\u{2665}\\s\\d\\d\\s\\w\\w\\w\\s\\d\\s\\w\\w\\w\\s\\w\\w\\w\\w\\s\\w\\w\\w\\s\\u{1f4a9}\\u{1f4a9}\\.$\n"
            ));
        }

        #[test]
        fn succeeds_with_escape_and_surrogate_option() {
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
                "^\\w\\s\\s\\s\\u{2665}\\u{2665}\\u{2665}\\s\\d\\d\\s\\w\\w\\w\\s\\d\\s\\w\\w\\w\\s\\w\\w\\w\\w\\s\\w\\w\\w\\s\\u{d83d}\\u{dca9}\\u{d83d}\\u{dca9}\\.$\n"
            ));
        }

        #[test]
        fn succeeds_with_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&["--digits", "--words", "--spaces", "--verbose", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  \w\s\s\s♥♥♥\s\d\d\s\w\w\w\s\d\s\w\w\w\s\w\w\w\w\s\w\w\w\s💩💩\.
                $
                "#
            )));
        }

        #[test]
        fn succeeds_with_escape_and_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&[
                "--digits",
                "--words",
                "--spaces",
                "--escape",
                "--verbose",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  \w\s\s\s\u{2665}\u{2665}\u{2665}\s\d\d\s\w\w\w\s\d\s\w\w\w\s\w\w\w\w\s\w\w\w\s\u{1f4a9}\u{1f4a9}\.
                $
                "#
            )));
        }

        #[test]
        fn succeeds_with_escape_and_surrogate_and_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&[
                "--digits",
                "--words",
                "--spaces",
                "--escape",
                "--with-surrogates",
                "--verbose",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  \w\s\s\s\u{2665}\u{2665}\u{2665}\s\d\d\s\w\w\w\s\d\s\w\w\w\s\w\w\w\w\s\w\w\w\s\u{d83d}\u{dca9}\u{d83d}\u{dca9}\.
                $
                "#
            )));
        }
    }

    mod repetition {
        use super::*;

        #[test]
        fn succeeds() {
            let mut grex = init_command();
            grex.args(&[
                "--repetitions",
                "--digits",
                "--words",
                "--spaces",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(
                "^\\w\\s{3}♥{3}\\s\\d(?:\\d\\s\\w{3}\\s){2}\\w(?:\\w{3}\\s){2}💩{2}\\.$\n",
            ));
        }

        #[test]
        fn succeeds_with_escape_option() {
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
                "^\\w\\s{3}\\u{2665}{3}\\s\\d(?:\\d\\s\\w{3}\\s){2}\\w(?:\\w{3}\\s){2}\\u{1f4a9}{2}\\.$\n",
            ));
        }

        #[test]
        fn succeeds_with_escape_and_surrogate_option() {
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
                "^\\w\\s{3}\\u{2665}{3}\\s\\d(?:\\d\\s\\w{3}\\s){2}\\w(?:\\w{3}\\s){2}(?:\\u{d83d}\\u{dca9}){2}\\.$\n",
            ));
        }

        #[test]
        fn succeeds_with_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&[
                "--repetitions",
                "--digits",
                "--words",
                "--spaces",
                "--verbose",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  \w\s{3}♥{3}\s\d
                  (?:
                    \d\s\w{3}\s
                  ){2}
                  \w
                  (?:
                    \w{3}\s
                  ){2}
                  💩{2}\.
                $
                "#
            )));
        }

        #[test]
        fn succeeds_with_escape_and_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&[
                "--repetitions",
                "--digits",
                "--words",
                "--spaces",
                "--escape",
                "--verbose",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  \w\s{3}\u{2665}{3}\s\d
                  (?:
                    \d\s\w{3}\s
                  ){2}
                  \w
                  (?:
                    \w{3}\s
                  ){2}
                  \u{1f4a9}{2}\.
                $
                "#
            )));
        }

        #[test]
        fn succeeds_with_escape_and_surrogate_and_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&[
                "--repetitions",
                "--digits",
                "--words",
                "--spaces",
                "--escape",
                "--with-surrogates",
                "--verbose",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  \w\s{3}\u{2665}{3}\s\d
                  (?:
                    \d\s\w{3}\s
                  ){2}
                  \w
                  (?:
                    \w{3}\s
                  ){2}
                  (?:
                    \u{d83d}\u{dca9}
                  ){2}
                  \.
                $
                "#
            )));
        }
    }
}

mod non_digit_conversion {
    use super::*;

    mod no_repetition {
        use super::*;

        #[test]
        fn succeeds() {
            let mut grex = init_command();
            grex.args(&["--non-digits", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(
                "^\\D\\D\\D\\D\\D\\D\\D\\D36\\D\\D\\D\\D\\D٣\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D$\n",
            ));
        }

        #[test]
        fn succeeds_with_escape_option() {
            let mut grex = init_command();
            grex.args(&["--non-digits", "--escape", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(
                "^\\D\\D\\D\\D\\D\\D\\D\\D36\\D\\D\\D\\D\\D\\u{663}\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D$\n",
            ));
        }

        #[test]
        fn succeeds_with_escape_and_surrogate_option() {
            let mut grex = init_command();
            grex.args(&["--non-digits", "--escape", "--with-surrogates", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(
                "^\\D\\D\\D\\D\\D\\D\\D\\D36\\D\\D\\D\\D\\D\\u{663}\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D$\n",
            ));
        }

        #[test]
        fn succeeds_with_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&["--non-digits", "--verbose", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  \D\D\D\D\D\D\D\D36\D\D\D\D\D٣\D\D\D\D\D\D\D\D\D\D\D\D\D\D\D\D\D
                $
                "#
            )));
        }

        #[test]
        fn succeeds_with_escape_and_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&["--non-digits", "--escape", "--verbose", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  \D\D\D\D\D\D\D\D36\D\D\D\D\D\u{663}\D\D\D\D\D\D\D\D\D\D\D\D\D\D\D\D\D
                $
                "#
            )));
        }

        #[test]
        fn succeeds_with_escape_and_surrogate_and_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&[
                "--non-digits",
                "--escape",
                "--with-surrogates",
                "--verbose",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  \D\D\D\D\D\D\D\D36\D\D\D\D\D\u{663}\D\D\D\D\D\D\D\D\D\D\D\D\D\D\D\D\D
                $
                "#
            )));
        }
    }

    mod repetition {
        use super::*;

        #[test]
        fn succeeds() {
            let mut grex = init_command();
            grex.args(&["--repetitions", "--non-digits", TEST_CASE]);
            grex.assert()
                .success()
                .stdout(predicate::eq("^\\D{8}36\\D{5}٣\\D{17}$\n"));
        }

        #[test]
        fn succeeds_with_escape_option() {
            let mut grex = init_command();
            grex.args(&["--repetitions", "--non-digits", "--escape", TEST_CASE]);
            grex.assert()
                .success()
                .stdout(predicate::eq("^\\D{8}36\\D{5}\\u{663}\\D{17}$\n"));
        }

        #[test]
        fn succeeds_with_escape_and_surrogate_option() {
            let mut grex = init_command();
            grex.args(&[
                "--repetitions",
                "--non-digits",
                "--escape",
                "--with-surrogates",
                TEST_CASE,
            ]);
            grex.assert()
                .success()
                .stdout(predicate::eq("^\\D{8}36\\D{5}\\u{663}\\D{17}$\n"));
        }

        #[test]
        fn succeeds_with_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&["--repetitions", "--non-digits", "--verbose", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  \D{8}36\D{5}٣\D{17}
                $
                "#
            )));
        }

        #[test]
        fn succeeds_with_escape_and_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&[
                "--repetitions",
                "--non-digits",
                "--escape",
                "--verbose",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  \D{8}36\D{5}\u{663}\D{17}
                $
                "#
            )));
        }

        #[test]
        fn succeeds_with_escape_and_surrogate_and_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&[
                "--repetitions",
                "--non-digits",
                "--escape",
                "--with-surrogates",
                "--verbose",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  \D{8}36\D{5}\u{663}\D{17}
                $
                "#
            )));
        }
    }
}

mod non_space_conversion {
    use super::*;

    mod no_repetition {
        use super::*;

        #[test]
        fn succeeds() {
            let mut grex = init_command();
            grex.args(&["--non-spaces", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(
                "^\\S   \\S\\S\\S \\S\\S \\S\\S\\S \\S \\S\\S\\S \\S\\S\\S\\S \\S\\S\\S \\S\\S\\S$\n",
            ));
        }

        #[test]
        fn succeeds_with_escape_option() {
            let mut grex = init_command();
            grex.args(&["--non-spaces", "--escape", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(
                "^\\S   \\S\\S\\S \\S\\S \\S\\S\\S \\S \\S\\S\\S \\S\\S\\S\\S \\S\\S\\S \\S\\S\\S$\n",
            ));
        }

        #[test]
        fn succeeds_with_escape_and_surrogate_option() {
            let mut grex = init_command();
            grex.args(&["--non-spaces", "--escape", "--with-surrogates", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(
                "^\\S   \\S\\S\\S \\S\\S \\S\\S\\S \\S \\S\\S\\S \\S\\S\\S\\S \\S\\S\\S \\S\\S\\S$\n",
            ));
        }

        #[test]
        fn succeeds_with_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&["--non-spaces", "--verbose", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  \S\ \ \ \S\S\S\ \S\S\ \S\S\S\ \S\ \S\S\S\ \S\S\S\S\ \S\S\S\ \S\S\S
                $
                "#
            )));
        }

        #[test]
        fn succeeds_with_escape_and_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&["--non-spaces", "--escape", "--verbose", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  \S\ \ \ \S\S\S\ \S\S\ \S\S\S\ \S\ \S\S\S\ \S\S\S\S\ \S\S\S\ \S\S\S
                $
                "#
            )));
        }

        #[test]
        fn succeeds_with_escape_and_surrogate_and_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&[
                "--non-spaces",
                "--escape",
                "--with-surrogates",
                "--verbose",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  \S\ \ \ \S\S\S\ \S\S\ \S\S\S\ \S\ \S\S\S\ \S\S\S\S\ \S\S\S\ \S\S\S
                $
                "#
            )));
        }
    }

    mod repetition {
        use super::*;

        #[test]
        fn succeeds() {
            let mut grex = init_command();
            grex.args(&["--repetitions", "--non-spaces", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(
                "^\\S {3}\\S(?:\\S{2} ){2}\\S{3} (?:\\S(?: \\S{3}){2}){2}$\n",
            ));
        }

        #[test]
        fn succeeds_with_escape_option() {
            let mut grex = init_command();
            grex.args(&["--repetitions", "--non-spaces", "--escape", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(
                "^\\S {3}\\S(?:\\S{2} ){2}\\S{3} (?:\\S(?: \\S{3}){2}){2}$\n",
            ));
        }

        #[test]
        fn succeeds_with_escape_and_surrogate_option() {
            let mut grex = init_command();
            grex.args(&[
                "--repetitions",
                "--non-spaces",
                "--escape",
                "--with-surrogates",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(
                "^\\S {3}\\S(?:\\S{2} ){2}\\S{3} (?:\\S(?: \\S{3}){2}){2}$\n",
            ));
        }

        #[test]
        fn succeeds_with_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&["--repetitions", "--non-spaces", "--verbose", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  \S\ {3}\S
                  (?:
                    \S{2}\ 
                  ){2}
                  \S{3}\ 
                  (?:
                    \S
                    (?:
                      \ \S{3}
                    ){2}
                  ){2}
                $
                "#
            )));
        }

        #[test]
        fn succeeds_with_escape_and_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&[
                "--repetitions",
                "--non-spaces",
                "--escape",
                "--verbose",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  \S\ {3}\S
                  (?:
                    \S{2}\ 
                  ){2}
                  \S{3}\ 
                  (?:
                    \S
                    (?:
                      \ \S{3}
                    ){2}
                  ){2}
                $
                "#
            )));
        }

        #[test]
        fn succeeds_with_escape_and_surrogate_and_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&[
                "--repetitions",
                "--non-spaces",
                "--escape",
                "--with-surrogates",
                "--verbose",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  \S\ {3}\S
                  (?:
                    \S{2}\ 
                  ){2}
                  \S{3}\ 
                  (?:
                    \S
                    (?:
                      \ \S{3}
                    ){2}
                  ){2}
                $
                "#
            )));
        }
    }
}

mod non_word_conversion {
    use super::*;

    mod no_repetition {
        use super::*;

        #[test]
        fn succeeds() {
            let mut grex = init_command();
            grex.args(&["--non-words", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(
                "^I\\W\\W\\W\\W\\W\\W\\W36\\Wand\\W٣\\Wand\\Wy̆y̆\\Wand\\W\\W\\W\\W$\n",
            ));
        }

        #[test]
        fn succeeds_with_escape_option() {
            let mut grex = init_command();
            grex.args(&["--non-words", "--escape", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(
                "^I\\W\\W\\W\\W\\W\\W\\W36\\Wand\\W\\u{663}\\Wand\\Wy\\u{306}y\\u{306}\\Wand\\W\\W\\W\\W$\n",
            ));
        }

        #[test]
        fn succeeds_with_escape_and_surrogate_option() {
            let mut grex = init_command();
            grex.args(&["--non-words", "--escape", "--with-surrogates", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(
                "^I\\W\\W\\W\\W\\W\\W\\W36\\Wand\\W\\u{663}\\Wand\\Wy\\u{306}y\\u{306}\\Wand\\W\\W\\W\\W$\n",
            ));
        }

        #[test]
        fn succeeds_with_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&["--non-words", "--verbose", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  I\W\W\W\W\W\W\W36\Wand\W٣\Wand\Wy̆y̆\Wand\W\W\W\W
                $
                "#
            )));
        }

        #[test]
        fn succeeds_with_escape_and_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&["--non-words", "--escape", "--verbose", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  I\W\W\W\W\W\W\W36\Wand\W\u{663}\Wand\Wy\u{306}y\u{306}\Wand\W\W\W\W
                $
                "#
            )));
        }

        #[test]
        fn succeeds_with_escape_and_surrogate_and_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&[
                "--non-words",
                "--escape",
                "--with-surrogates",
                "--verbose",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  I\W\W\W\W\W\W\W36\Wand\W\u{663}\Wand\Wy\u{306}y\u{306}\Wand\W\W\W\W
                $
                "#
            )));
        }
    }

    mod repetition {
        use super::*;

        #[test]
        fn succeeds() {
            let mut grex = init_command();
            grex.args(&["--repetitions", "--non-words", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(
                "^I\\W{7}36\\Wand\\W٣\\Wand\\W(?:y̆){2}\\Wand\\W{4}$\n",
            ));
        }

        #[test]
        fn succeeds_with_escape_option() {
            let mut grex = init_command();
            grex.args(&["--repetitions", "--non-words", "--escape", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(
                "^I\\W{7}36\\Wand\\W\\u{663}\\Wand\\W(?:y\\u{306}){2}\\Wand\\W{4}$\n",
            ));
        }

        #[test]
        fn succeeds_with_escape_and_surrogate_option() {
            let mut grex = init_command();
            grex.args(&[
                "--repetitions",
                "--non-words",
                "--escape",
                "--with-surrogates",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(
                "^I\\W{7}36\\Wand\\W\\u{663}\\Wand\\W(?:y\\u{306}){2}\\Wand\\W{4}$\n",
            ));
        }

        #[test]
        fn succeeds_with_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&["--repetitions", "--non-words", "--verbose", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  I\W{7}36\Wand\W٣\Wand\W
                  (?:
                    y̆
                  ){2}
                  \Wand\W{4}
                $
                "#
            )));
        }

        #[test]
        fn succeeds_with_escape_and_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&[
                "--repetitions",
                "--non-words",
                "--escape",
                "--verbose",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  I\W{7}36\Wand\W\u{663}\Wand\W
                  (?:
                    y\u{306}
                  ){2}
                  \Wand\W{4}
                $
                "#
            )));
        }

        #[test]
        fn succeeds_with_escape_and_surrogate_and_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&[
                "--repetitions",
                "--non-words",
                "--escape",
                "--with-surrogates",
                "--verbose",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  I\W{7}36\Wand\W\u{663}\Wand\W
                  (?:
                    y\u{306}
                  ){2}
                  \Wand\W{4}
                $
                "#
            )));
        }
    }
}

mod non_digit_non_space_conversion {
    use super::*;

    mod no_repetition {
        use super::*;

        #[test]
        fn succeeds() {
            let mut grex = init_command();
            grex.args(&["--non-digits", "--non-spaces", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(
                "^\\D\\D\\D\\D\\D\\D\\D\\D\\S\\S\\D\\D\\D\\D\\D\\S\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D$\n",
            ));
        }

        #[test]
        fn succeeds_with_escape_option() {
            let mut grex = init_command();
            grex.args(&["--non-digits", "--non-spaces", "--escape", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(
                "^\\D\\D\\D\\D\\D\\D\\D\\D\\S\\S\\D\\D\\D\\D\\D\\S\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D$\n",
            ));
        }

        #[test]
        fn succeeds_with_escape_and_surrogate_option() {
            let mut grex = init_command();
            grex.args(&[
                "--non-digits",
                "--non-spaces",
                "--escape",
                "--with-surrogates",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(
                "^\\D\\D\\D\\D\\D\\D\\D\\D\\S\\S\\D\\D\\D\\D\\D\\S\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D$\n",
            ));
        }

        #[test]
        fn succeeds_with_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&["--non-digits", "--non-spaces", "--verbose", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  \D\D\D\D\D\D\D\D\S\S\D\D\D\D\D\S\D\D\D\D\D\D\D\D\D\D\D\D\D\D\D\D\D
                $
                "#
            )));
        }

        #[test]
        fn succeeds_with_escape_and_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&[
                "--non-digits",
                "--non-spaces",
                "--escape",
                "--verbose",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  \D\D\D\D\D\D\D\D\S\S\D\D\D\D\D\S\D\D\D\D\D\D\D\D\D\D\D\D\D\D\D\D\D
                $
                "#
            )));
        }

        #[test]
        fn succeeds_with_escape_and_surrogate_and_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&[
                "--non-digits",
                "--non-spaces",
                "--escape",
                "--with-surrogates",
                "--verbose",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  \D\D\D\D\D\D\D\D\S\S\D\D\D\D\D\S\D\D\D\D\D\D\D\D\D\D\D\D\D\D\D\D\D
                $
                "#
            )));
        }
    }

    mod repetition {
        use super::*;

        #[test]
        fn succeeds() {
            let mut grex = init_command();
            grex.args(&["--repetitions", "--non-digits", "--non-spaces", TEST_CASE]);
            grex.assert()
                .success()
                .stdout(predicate::eq("^\\D{8}\\S{2}\\D{5}\\S\\D{17}$\n"));
        }

        #[test]
        fn succeeds_with_escape_option() {
            let mut grex = init_command();
            grex.args(&[
                "--repetitions",
                "--non-digits",
                "--non-spaces",
                "--escape",
                TEST_CASE,
            ]);
            grex.assert()
                .success()
                .stdout(predicate::eq("^\\D{8}\\S{2}\\D{5}\\S\\D{17}$\n"));
        }

        #[test]
        fn succeeds_with_escape_and_surrogate_option() {
            let mut grex = init_command();
            grex.args(&[
                "--repetitions",
                "--non-digits",
                "--non-spaces",
                "--escape",
                "--with-surrogates",
                TEST_CASE,
            ]);
            grex.assert()
                .success()
                .stdout(predicate::eq("^\\D{8}\\S{2}\\D{5}\\S\\D{17}$\n"));
        }

        #[test]
        fn succeeds_with_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&[
                "--repetitions",
                "--non-digits",
                "--non-spaces",
                "--verbose",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  \D{8}\S{2}\D{5}\S\D{17}
                $
                "#
            )));
        }

        #[test]
        fn succeeds_with_escape_and_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&[
                "--repetitions",
                "--non-digits",
                "--non-spaces",
                "--escape",
                "--verbose",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  \D{8}\S{2}\D{5}\S\D{17}
                $
                "#
            )));
        }

        #[test]
        fn succeeds_with_escape_and_surrogate_and_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&[
                "--repetitions",
                "--non-digits",
                "--non-spaces",
                "--escape",
                "--with-surrogates",
                "--verbose",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  \D{8}\S{2}\D{5}\S\D{17}
                $
                "#
            )));
        }
    }
}

mod non_digit_non_word_conversion {
    use super::*;

    mod no_repetition {
        use super::*;

        #[test]
        fn succeeds() {
            let mut grex = init_command();
            grex.args(&["--non-digits", "--non-words", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(
                "^\\D\\D\\D\\D\\D\\D\\D\\D36\\D\\D\\D\\D\\D٣\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D$\n",
            ));
        }

        #[test]
        fn succeeds_with_escape_option() {
            let mut grex = init_command();
            grex.args(&["--non-digits", "--non-words", "--escape", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(
                "^\\D\\D\\D\\D\\D\\D\\D\\D36\\D\\D\\D\\D\\D\\u{663}\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D$\n",
            ));
        }

        #[test]
        fn succeeds_with_escape_and_surrogate_option() {
            let mut grex = init_command();
            grex.args(&[
                "--non-digits",
                "--non-words",
                "--escape",
                "--with-surrogates",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(
                "^\\D\\D\\D\\D\\D\\D\\D\\D36\\D\\D\\D\\D\\D\\u{663}\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D$\n",
            ));
        }

        #[test]
        fn succeeds_with_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&["--non-digits", "--non-words", "--verbose", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  \D\D\D\D\D\D\D\D36\D\D\D\D\D٣\D\D\D\D\D\D\D\D\D\D\D\D\D\D\D\D\D
                $
                "#
            )));
        }

        #[test]
        fn succeeds_with_escape_and_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&[
                "--non-digits",
                "--non-words",
                "--escape",
                "--verbose",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  \D\D\D\D\D\D\D\D36\D\D\D\D\D\u{663}\D\D\D\D\D\D\D\D\D\D\D\D\D\D\D\D\D
                $
                "#
            )));
        }

        #[test]
        fn succeeds_with_escape_and_surrogate_and_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&[
                "--non-digits",
                "--non-words",
                "--escape",
                "--with-surrogates",
                "--verbose",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  \D\D\D\D\D\D\D\D36\D\D\D\D\D\u{663}\D\D\D\D\D\D\D\D\D\D\D\D\D\D\D\D\D
                $
                "#
            )));
        }
    }

    mod repetition {
        use super::*;

        #[test]
        fn succeeds() {
            let mut grex = init_command();
            grex.args(&["--repetitions", "--non-digits", "--non-words", TEST_CASE]);
            grex.assert()
                .success()
                .stdout(predicate::eq("^\\D{8}36\\D{5}٣\\D{17}$\n"));
        }

        #[test]
        fn succeeds_with_escape_option() {
            let mut grex = init_command();
            grex.args(&[
                "--repetitions",
                "--non-digits",
                "--non-words",
                "--escape",
                TEST_CASE,
            ]);
            grex.assert()
                .success()
                .stdout(predicate::eq("^\\D{8}36\\D{5}\\u{663}\\D{17}$\n"));
        }

        #[test]
        fn succeeds_with_escape_and_surrogate_option() {
            let mut grex = init_command();
            grex.args(&[
                "--repetitions",
                "--non-digits",
                "--non-words",
                "--escape",
                "--with-surrogates",
                TEST_CASE,
            ]);
            grex.assert()
                .success()
                .stdout(predicate::eq("^\\D{8}36\\D{5}\\u{663}\\D{17}$\n"));
        }

        #[test]
        fn succeeds_with_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&[
                "--repetitions",
                "--non-digits",
                "--non-words",
                "--verbose",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  \D{8}36\D{5}٣\D{17}
                $
                "#
            )));
        }

        #[test]
        fn succeeds_with_escape_and_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&[
                "--repetitions",
                "--non-digits",
                "--non-words",
                "--escape",
                "--verbose",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  \D{8}36\D{5}\u{663}\D{17}
                $
                "#
            )));
        }

        #[test]
        fn succeeds_with_escape_and_surrogate_and_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&[
                "--repetitions",
                "--non-digits",
                "--non-words",
                "--escape",
                "--with-surrogates",
                "--verbose",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  \D{8}36\D{5}\u{663}\D{17}
                $
                "#
            )));
        }
    }
}

mod non_space_non_word_conversion {
    use super::*;

    mod no_repetition {
        use super::*;

        #[test]
        fn succeeds() {
            let mut grex = init_command();
            grex.args(&["--non-spaces", "--non-words", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(
                "^\\S\\W\\W\\W\\W\\W\\W\\W\\S\\S\\W\\S\\S\\S\\W\\S\\W\\S\\S\\S\\W\\S\\S\\S\\S\\W\\S\\S\\S\\W\\W\\W\\W$\n",
            ));
        }

        #[test]
        fn succeeds_with_escape_option() {
            let mut grex = init_command();
            grex.args(&["--non-spaces", "--non-words", "--escape", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(
                "^\\S\\W\\W\\W\\W\\W\\W\\W\\S\\S\\W\\S\\S\\S\\W\\S\\W\\S\\S\\S\\W\\S\\S\\S\\S\\W\\S\\S\\S\\W\\W\\W\\W$\n",
            ));
        }

        #[test]
        fn succeeds_with_escape_and_surrogate_option() {
            let mut grex = init_command();
            grex.args(&[
                "--non-spaces",
                "--non-words",
                "--escape",
                "--with-surrogates",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(
                "^\\S\\W\\W\\W\\W\\W\\W\\W\\S\\S\\W\\S\\S\\S\\W\\S\\W\\S\\S\\S\\W\\S\\S\\S\\S\\W\\S\\S\\S\\W\\W\\W\\W$\n",
            ));
        }

        #[test]
        fn succeeds_with_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&["--non-spaces", "--non-words", "--verbose", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  \S\W\W\W\W\W\W\W\S\S\W\S\S\S\W\S\W\S\S\S\W\S\S\S\S\W\S\S\S\W\W\W\W
                $
                "#
            )));
        }

        #[test]
        fn succeeds_with_escape_and_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&[
                "--non-spaces",
                "--non-words",
                "--escape",
                "--verbose",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  \S\W\W\W\W\W\W\W\S\S\W\S\S\S\W\S\W\S\S\S\W\S\S\S\S\W\S\S\S\W\W\W\W
                $
                "#
            )));
        }

        #[test]
        fn succeeds_with_escape_and_surrogate_and_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&[
                "--non-spaces",
                "--non-words",
                "--escape",
                "--with-surrogates",
                "--verbose",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  \S\W\W\W\W\W\W\W\S\S\W\S\S\S\W\S\W\S\S\S\W\S\S\S\S\W\S\S\S\W\W\W\W
                $
                "#
            )));
        }
    }

    mod repetition {
        use super::*;

        #[test]
        fn succeeds() {
            let mut grex = init_command();
            grex.args(&["--repetitions", "--non-spaces", "--non-words", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(
                "^\\S\\W{7}\\S(?:\\S\\W\\S{3}\\W){2}\\S{4}\\W\\S{3}\\W{4}$\n",
            ));
        }

        #[test]
        fn succeeds_with_escape_option() {
            let mut grex = init_command();
            grex.args(&[
                "--repetitions",
                "--non-spaces",
                "--non-words",
                "--escape",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(
                "^\\S\\W{7}\\S(?:\\S\\W\\S{3}\\W){2}\\S{4}\\W\\S{3}\\W{4}$\n",
            ));
        }

        #[test]
        fn succeeds_with_escape_and_surrogate_option() {
            let mut grex = init_command();
            grex.args(&[
                "--repetitions",
                "--non-spaces",
                "--non-words",
                "--escape",
                "--with-surrogates",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(
                "^\\S\\W{7}\\S(?:\\S\\W\\S{3}\\W){2}\\S{4}\\W\\S{3}\\W{4}$\n",
            ));
        }

        #[test]
        fn succeeds_with_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&[
                "--repetitions",
                "--non-spaces",
                "--non-words",
                "--verbose",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  \S\W{7}\S
                  (?:
                    \S\W\S{3}\W
                  ){2}
                  \S{4}\W\S{3}\W{4}
                $
                "#
            )));
        }

        #[test]
        fn succeeds_with_escape_and_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&[
                "--repetitions",
                "--non-spaces",
                "--non-words",
                "--escape",
                "--verbose",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  \S\W{7}\S
                  (?:
                    \S\W\S{3}\W
                  ){2}
                  \S{4}\W\S{3}\W{4}
                $
                "#
            )));
        }

        #[test]
        fn succeeds_with_escape_and_surrogate_and_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&[
                "--repetitions",
                "--non-spaces",
                "--non-words",
                "--escape",
                "--with-surrogates",
                "--verbose",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  \S\W{7}\S
                  (?:
                    \S\W\S{3}\W
                  ){2}
                  \S{4}\W\S{3}\W{4}
                $
                "#
            )));
        }
    }
}

mod non_digit_non_space_non_word_conversion {
    use super::*;

    mod no_repetition {
        use super::*;

        #[test]
        fn succeeds() {
            let mut grex = init_command();
            grex.args(&["--non-digits", "--non-spaces", "--non-words", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(
                "^\\D\\D\\D\\D\\D\\D\\D\\D\\S\\S\\D\\D\\D\\D\\D\\S\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D$\n",
            ));
        }

        #[test]
        fn succeeds_with_escape_option() {
            let mut grex = init_command();
            grex.args(&[
                "--non-digits",
                "--non-spaces",
                "--non-words",
                "--escape",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(
                "^\\D\\D\\D\\D\\D\\D\\D\\D\\S\\S\\D\\D\\D\\D\\D\\S\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D$\n",
            ));
        }

        #[test]
        fn succeeds_with_escape_and_surrogate_option() {
            let mut grex = init_command();
            grex.args(&[
                "--non-digits",
                "--non-spaces",
                "--non-words",
                "--escape",
                "--with-surrogates",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(
                "^\\D\\D\\D\\D\\D\\D\\D\\D\\S\\S\\D\\D\\D\\D\\D\\S\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D$\n",
            ));
        }

        #[test]
        fn succeeds_with_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&[
                "--non-digits",
                "--non-spaces",
                "--non-words",
                "--verbose",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  \D\D\D\D\D\D\D\D\S\S\D\D\D\D\D\S\D\D\D\D\D\D\D\D\D\D\D\D\D\D\D\D\D
                $
                "#
            )));
        }

        #[test]
        fn succeeds_with_escape_and_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&[
                "--non-digits",
                "--non-spaces",
                "--non-words",
                "--escape",
                "--verbose",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  \D\D\D\D\D\D\D\D\S\S\D\D\D\D\D\S\D\D\D\D\D\D\D\D\D\D\D\D\D\D\D\D\D
                $
                "#
            )));
        }

        #[test]
        fn succeeds_with_escape_and_surrogate_and_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&[
                "--non-digits",
                "--non-spaces",
                "--non-words",
                "--escape",
                "--with-surrogates",
                "--verbose",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  \D\D\D\D\D\D\D\D\S\S\D\D\D\D\D\S\D\D\D\D\D\D\D\D\D\D\D\D\D\D\D\D\D
                $
                "#
            )));
        }
    }

    mod repetition {
        use super::*;

        #[test]
        fn succeeds() {
            let mut grex = init_command();
            grex.args(&[
                "--repetitions",
                "--non-digits",
                "--non-spaces",
                "--non-words",
                TEST_CASE,
            ]);
            grex.assert()
                .success()
                .stdout(predicate::eq("^\\D{8}\\S{2}\\D{5}\\S\\D{17}$\n"));
        }

        #[test]
        fn succeeds_with_escape_option() {
            let mut grex = init_command();
            grex.args(&[
                "--repetitions",
                "--non-digits",
                "--non-spaces",
                "--non-words",
                "--escape",
                TEST_CASE,
            ]);
            grex.assert()
                .success()
                .stdout(predicate::eq("^\\D{8}\\S{2}\\D{5}\\S\\D{17}$\n"));
        }

        #[test]
        fn succeeds_with_escape_and_surrogate_option() {
            let mut grex = init_command();
            grex.args(&[
                "--repetitions",
                "--non-digits",
                "--non-spaces",
                "--non-words",
                "--escape",
                "--with-surrogates",
                TEST_CASE,
            ]);
            grex.assert()
                .success()
                .stdout(predicate::eq("^\\D{8}\\S{2}\\D{5}\\S\\D{17}$\n"));
        }

        #[test]
        fn succeeds_with_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&[
                "--repetitions",
                "--non-digits",
                "--non-spaces",
                "--non-words",
                "--verbose",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  \D{8}\S{2}\D{5}\S\D{17}
                $
                "#
            )));
        }

        #[test]
        fn succeeds_with_escape_and_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&[
                "--repetitions",
                "--non-digits",
                "--non-spaces",
                "--non-words",
                "--escape",
                "--verbose",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  \D{8}\S{2}\D{5}\S\D{17}
                $
                "#
            )));
        }

        #[test]
        fn succeeds_with_escape_and_surrogate_and_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&[
                "--repetitions",
                "--non-digits",
                "--non-spaces",
                "--non-words",
                "--escape",
                "--with-surrogates",
                "--verbose",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  \D{8}\S{2}\D{5}\S\D{17}
                $
                "#
            )));
        }
    }
}

mod digit_non_digit_conversion {
    use super::*;

    mod no_repetition {
        use super::*;

        #[test]
        fn succeeds() {
            let mut grex = init_command();
            grex.args(&["--digits", "--non-digits", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(
                "^\\D\\D\\D\\D\\D\\D\\D\\D\\d\\d\\D\\D\\D\\D\\D\\d\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D$\n",
            ));
        }

        #[test]
        fn succeeds_with_escape_option() {
            let mut grex = init_command();
            grex.args(&["--digits", "--non-digits", "--escape", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(
                "^\\D\\D\\D\\D\\D\\D\\D\\D\\d\\d\\D\\D\\D\\D\\D\\d\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D$\n",
            ));
        }

        #[test]
        fn succeeds_with_escape_and_surrogate_option() {
            let mut grex = init_command();
            grex.args(&[
                "--digits",
                "--non-digits",
                "--escape",
                "--with-surrogates",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(
                "^\\D\\D\\D\\D\\D\\D\\D\\D\\d\\d\\D\\D\\D\\D\\D\\d\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D$\n",
            ));
        }

        #[test]
        fn succeeds_with_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&["--digits", "--non-digits", "--verbose", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  \D\D\D\D\D\D\D\D\d\d\D\D\D\D\D\d\D\D\D\D\D\D\D\D\D\D\D\D\D\D\D\D\D
                $
                "#
            )));
        }

        #[test]
        fn succeeds_with_escape_and_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&[
                "--digits",
                "--non-digits",
                "--escape",
                "--verbose",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  \D\D\D\D\D\D\D\D\d\d\D\D\D\D\D\d\D\D\D\D\D\D\D\D\D\D\D\D\D\D\D\D\D
                $
                "#
            )));
        }

        #[test]
        fn succeeds_with_escape_and_surrogate_and_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&[
                "--digits",
                "--non-digits",
                "--escape",
                "--with-surrogates",
                "--verbose",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  \D\D\D\D\D\D\D\D\d\d\D\D\D\D\D\d\D\D\D\D\D\D\D\D\D\D\D\D\D\D\D\D\D
                $
                "#
            )));
        }
    }

    mod repetition {
        use super::*;

        #[test]
        fn succeeds() {
            let mut grex = init_command();
            grex.args(&["--repetitions", "--digits", "--non-digits", TEST_CASE]);
            grex.assert()
                .success()
                .stdout(predicate::eq("^\\D{8}\\d{2}\\D{5}\\d\\D{17}$\n"));
        }

        #[test]
        fn succeeds_with_escape_option() {
            let mut grex = init_command();
            grex.args(&[
                "--repetitions",
                "--digits",
                "--non-digits",
                "--escape",
                TEST_CASE,
            ]);
            grex.assert()
                .success()
                .stdout(predicate::eq("^\\D{8}\\d{2}\\D{5}\\d\\D{17}$\n"));
        }

        #[test]
        fn succeeds_with_escape_and_surrogate_option() {
            let mut grex = init_command();
            grex.args(&[
                "--repetitions",
                "--digits",
                "--non-digits",
                "--escape",
                "--with-surrogates",
                TEST_CASE,
            ]);
            grex.assert()
                .success()
                .stdout(predicate::eq("^\\D{8}\\d{2}\\D{5}\\d\\D{17}$\n"));
        }

        #[test]
        fn succeeds_with_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&[
                "--repetitions",
                "--digits",
                "--non-digits",
                "--verbose",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  \D{8}\d{2}\D{5}\d\D{17}
                $
                "#
            )));
        }

        #[test]
        fn succeeds_with_escape_and_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&[
                "--repetitions",
                "--digits",
                "--non-digits",
                "--escape",
                "--verbose",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  \D{8}\d{2}\D{5}\d\D{17}
                $
                "#
            )));
        }

        #[test]
        fn succeeds_with_escape_and_surrogate_and_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&[
                "--repetitions",
                "--digits",
                "--non-digits",
                "--escape",
                "--with-surrogates",
                "--verbose",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  \D{8}\d{2}\D{5}\d\D{17}
                $
                "#
            )));
        }
    }
}

mod space_non_space_conversion {
    use super::*;

    mod no_repetition {
        use super::*;

        #[test]
        fn succeeds() {
            let mut grex = init_command();
            grex.args(&["--spaces", "--non-spaces", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(
                "^\\S\\s\\s\\s\\S\\S\\S\\s\\S\\S\\s\\S\\S\\S\\s\\S\\s\\S\\S\\S\\s\\S\\S\\S\\S\\s\\S\\S\\S\\s\\S\\S\\S$\n",
            ));
        }

        #[test]
        fn succeeds_with_escape_option() {
            let mut grex = init_command();
            grex.args(&["--spaces", "--non-spaces", "--escape", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(
                "^\\S\\s\\s\\s\\S\\S\\S\\s\\S\\S\\s\\S\\S\\S\\s\\S\\s\\S\\S\\S\\s\\S\\S\\S\\S\\s\\S\\S\\S\\s\\S\\S\\S$\n",
            ));
        }

        #[test]
        fn succeeds_with_escape_and_surrogate_option() {
            let mut grex = init_command();
            grex.args(&[
                "--spaces",
                "--non-spaces",
                "--escape",
                "--with-surrogates",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(
                "^\\S\\s\\s\\s\\S\\S\\S\\s\\S\\S\\s\\S\\S\\S\\s\\S\\s\\S\\S\\S\\s\\S\\S\\S\\S\\s\\S\\S\\S\\s\\S\\S\\S$\n",
            ));
        }

        #[test]
        fn succeeds_with_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&["--spaces", "--non-spaces", "--verbose", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  \S\s\s\s\S\S\S\s\S\S\s\S\S\S\s\S\s\S\S\S\s\S\S\S\S\s\S\S\S\s\S\S\S
                $
                "#
            )));
        }

        #[test]
        fn succeeds_with_escape_and_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&[
                "--spaces",
                "--non-spaces",
                "--escape",
                "--verbose",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  \S\s\s\s\S\S\S\s\S\S\s\S\S\S\s\S\s\S\S\S\s\S\S\S\S\s\S\S\S\s\S\S\S
                $
                "#
            )));
        }

        #[test]
        fn succeeds_with_escape_and_surrogate_and_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&[
                "--spaces",
                "--non-spaces",
                "--escape",
                "--with-surrogates",
                "--verbose",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  \S\s\s\s\S\S\S\s\S\S\s\S\S\S\s\S\s\S\S\S\s\S\S\S\S\s\S\S\S\s\S\S\S
                $
                "#
            )));
        }
    }

    mod repetition {
        use super::*;

        #[test]
        fn succeeds() {
            let mut grex = init_command();
            grex.args(&["--repetitions", "--spaces", "--non-spaces", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(
                "^\\S\\s{3}\\S(?:\\S{2}\\s){2}\\S{3}\\s(?:\\S(?:\\s\\S{3}){2}){2}$\n",
            ));
        }

        #[test]
        fn succeeds_with_escape_option() {
            let mut grex = init_command();
            grex.args(&[
                "--repetitions",
                "--spaces",
                "--non-spaces",
                "--escape",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(
                "^\\S\\s{3}\\S(?:\\S{2}\\s){2}\\S{3}\\s(?:\\S(?:\\s\\S{3}){2}){2}$\n",
            ));
        }

        #[test]
        fn succeeds_with_escape_and_surrogate_option() {
            let mut grex = init_command();
            grex.args(&[
                "--repetitions",
                "--spaces",
                "--non-spaces",
                "--escape",
                "--with-surrogates",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(
                "^\\S\\s{3}\\S(?:\\S{2}\\s){2}\\S{3}\\s(?:\\S(?:\\s\\S{3}){2}){2}$\n",
            ));
        }

        #[test]
        fn succeeds_with_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&[
                "--repetitions",
                "--spaces",
                "--non-spaces",
                "--verbose",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  \S\s{3}\S
                  (?:
                    \S{2}\s
                  ){2}
                  \S{3}\s
                  (?:
                    \S
                    (?:
                      \s\S{3}
                    ){2}
                  ){2}
                $
                "#
            )));
        }

        #[test]
        fn succeeds_with_escape_and_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&[
                "--repetitions",
                "--spaces",
                "--non-spaces",
                "--escape",
                "--verbose",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  \S\s{3}\S
                  (?:
                    \S{2}\s
                  ){2}
                  \S{3}\s
                  (?:
                    \S
                    (?:
                      \s\S{3}
                    ){2}
                  ){2}
                $
                "#
            )));
        }

        #[test]
        fn succeeds_with_escape_and_surrogate_and_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&[
                "--repetitions",
                "--spaces",
                "--non-spaces",
                "--escape",
                "--with-surrogates",
                "--verbose",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  \S\s{3}\S
                  (?:
                    \S{2}\s
                  ){2}
                  \S{3}\s
                  (?:
                    \S
                    (?:
                      \s\S{3}
                    ){2}
                  ){2}
                $
                "#
            )));
        }
    }
}

mod word_non_word_conversion {
    use super::*;

    mod no_repetition {
        use super::*;

        #[test]
        fn succeeds() {
            let mut grex = init_command();
            grex.args(&["--words", "--non-words", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(
                "^\\w\\W\\W\\W\\W\\W\\W\\W\\w\\w\\W\\w\\w\\w\\W\\w\\W\\w\\w\\w\\W\\w\\w\\w\\w\\W\\w\\w\\w\\W\\W\\W\\W$\n",
            ));
        }

        #[test]
        fn succeeds_with_escape_option() {
            let mut grex = init_command();
            grex.args(&["--words", "--non-words", "--escape", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(
                "^\\w\\W\\W\\W\\W\\W\\W\\W\\w\\w\\W\\w\\w\\w\\W\\w\\W\\w\\w\\w\\W\\w\\w\\w\\w\\W\\w\\w\\w\\W\\W\\W\\W$\n",
            ));
        }

        #[test]
        fn succeeds_with_escape_and_surrogate_option() {
            let mut grex = init_command();
            grex.args(&[
                "--words",
                "--non-words",
                "--escape",
                "--with-surrogates",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(
                "^\\w\\W\\W\\W\\W\\W\\W\\W\\w\\w\\W\\w\\w\\w\\W\\w\\W\\w\\w\\w\\W\\w\\w\\w\\w\\W\\w\\w\\w\\W\\W\\W\\W$\n",
            ));
        }

        #[test]
        fn succeeds_with_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&["--words", "--non-words", "--verbose", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  \w\W\W\W\W\W\W\W\w\w\W\w\w\w\W\w\W\w\w\w\W\w\w\w\w\W\w\w\w\W\W\W\W
                $
                "#
            )));
        }

        #[test]
        fn succeeds_with_escape_and_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&["--words", "--non-words", "--escape", "--verbose", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  \w\W\W\W\W\W\W\W\w\w\W\w\w\w\W\w\W\w\w\w\W\w\w\w\w\W\w\w\w\W\W\W\W
                $
                "#
            )));
        }

        #[test]
        fn succeeds_with_escape_and_surrogate_and_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&[
                "--words",
                "--non-words",
                "--escape",
                "--with-surrogates",
                "--verbose",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  \w\W\W\W\W\W\W\W\w\w\W\w\w\w\W\w\W\w\w\w\W\w\w\w\w\W\w\w\w\W\W\W\W
                $
                "#
            )));
        }
    }

    mod repetition {
        use super::*;

        #[test]
        fn succeeds() {
            let mut grex = init_command();
            grex.args(&["--repetitions", "--words", "--non-words", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(
                "^\\w\\W{7}\\w(?:\\w\\W\\w{3}\\W){2}\\w{4}\\W\\w{3}\\W{4}$\n",
            ));
        }

        #[test]
        fn succeeds_with_escape_option() {
            let mut grex = init_command();
            grex.args(&[
                "--repetitions",
                "--words",
                "--non-words",
                "--escape",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(
                "^\\w\\W{7}\\w(?:\\w\\W\\w{3}\\W){2}\\w{4}\\W\\w{3}\\W{4}$\n",
            ));
        }

        #[test]
        fn succeeds_with_escape_and_surrogate_option() {
            let mut grex = init_command();
            grex.args(&[
                "--repetitions",
                "--words",
                "--non-words",
                "--escape",
                "--with-surrogates",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(
                "^\\w\\W{7}\\w(?:\\w\\W\\w{3}\\W){2}\\w{4}\\W\\w{3}\\W{4}$\n",
            ));
        }

        #[test]
        fn succeeds_with_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&[
                "--repetitions",
                "--words",
                "--non-words",
                "--verbose",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  \w\W{7}\w
                  (?:
                    \w\W\w{3}\W
                  ){2}
                  \w{4}\W\w{3}\W{4}
                $
                "#
            )));
        }

        #[test]
        fn succeeds_with_escape_and_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&[
                "--repetitions",
                "--words",
                "--non-words",
                "--escape",
                "--verbose",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  \w\W{7}\w
                  (?:
                    \w\W\w{3}\W
                  ){2}
                  \w{4}\W\w{3}\W{4}
                $
                "#
            )));
        }

        #[test]
        fn succeeds_with_escape_and_surrogate_and_verbose_mode_option() {
            let mut grex = init_command();
            grex.args(&[
                "--repetitions",
                "--words",
                "--non-words",
                "--escape",
                "--with-surrogates",
                "--verbose",
                TEST_CASE,
            ]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  \w\W{7}\w
                  (?:
                    \w\W\w{3}\W
                  ){2}
                  \w{4}\W\w{3}\W{4}
                $
                "#
            )));
        }
    }
}

mod anchor_conversion {
    use super::*;

    mod no_verbose {
        use super::*;

        #[test]
        fn succeeds_with_no_start_anchor_option() {
            let mut grex = init_command();
            grex.args(&["--no-start-anchor", TEST_CASE]);
            grex.assert()
                .success()
                .stdout(predicate::eq("I   ♥♥♥ 36 and ٣ and y̆y̆ and 💩💩\\.$\n"));
        }

        #[test]
        fn succeeds_with_no_end_anchor_option() {
            let mut grex = init_command();
            grex.args(&["--no-end-anchor", TEST_CASE]);
            grex.assert()
                .success()
                .stdout(predicate::eq("^I   ♥♥♥ 36 and ٣ and y̆y̆ and 💩💩\\.\n"));
        }

        #[test]
        fn succeeds_with_no_anchors_option() {
            let mut grex = init_command();
            grex.args(&["--no-anchors", TEST_CASE]);
            grex.assert()
                .success()
                .stdout(predicate::eq("I   ♥♥♥ 36 and ٣ and y̆y̆ and 💩💩\\.\n"));
        }
    }

    mod verbose {
        use super::*;

        #[test]
        fn succeeds_with_verbose_mode_and_no_start_anchor_option() {
            let mut grex = init_command();
            grex.args(&["--verbose", "--no-start-anchor", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                  I\ \ \ ♥♥♥\ 36\ and\ ٣\ and\ y̆y̆\ and\ 💩💩\.
                $
                "#,
            )));
        }

        #[test]
        fn succeeds_with_verbose_mode_and_no_end_anchor_option() {
            let mut grex = init_command();
            grex.args(&["--verbose", "--no-end-anchor", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                ^
                  I\ \ \ ♥♥♥\ 36\ and\ ٣\ and\ y̆y̆\ and\ 💩💩\.
                "#,
            )));
        }

        #[test]
        fn succeeds_with_verbose_mode_and_no_anchors_option() {
            let mut grex = init_command();
            grex.args(&["--verbose", "--no-anchors", TEST_CASE]);
            grex.assert().success().stdout(predicate::eq(indoc!(
                r#"
                (?x)
                  I\ \ \ ♥♥♥\ 36\ and\ ٣\ and\ y̆y̆\ and\ 💩💩\.
                "#,
            )));
        }
    }
}

fn init_command() -> Command {
    Command::cargo_bin("grex").unwrap()
}
