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
use indoc::indoc;
use regex::Regex;
use rstest::rstest;
use std::io::Write;
use tempfile::NamedTempFile;

mod no_conversion {
    use super::*;

    mod no_repetition {
        use super::*;

        #[rstest(test_cases, expected_output,
            case(vec![""], "^$"),
            case(vec![" "], "^ $"),
            case(vec!["   "], "^   $"),
            case(vec!["["], "^\\[$"),
            case(vec!["a", "("], "^[(a]$"),
            case(vec!["a", "\n"], "^[\\na]$"),
            case(vec!["a", "["], "^[\\[a]$"),
            case(vec!["a", "-", "c", "!"], "^[!\\-ac]$"),
            case(vec!["a", "b"], "^[ab]$"),
            case(vec!["a", "b", "c"], "^[a-c]$"),
            case(vec!["a", "c", "d", "e", "f"], "^[ac-f]$"),
            case(vec!["a", "b", "x", "d", "e"], "^[abdex]$"),
            case(vec!["a", "b", "x", "de"], "^(?:de|[abx])$"),
            case(vec!["a", "b", "c", "x", "d", "e"], "^[a-ex]$"),
            case(vec!["a", "b", "c", "x", "de"], "^(?:de|[a-cx])$"),
            case(vec!["a", "b", "c", "d", "e", "f", "o", "x", "y", "z"], "^[a-fox-z]$"),
            case(vec!["a", "b", "d", "e", "f", "o", "x", "y", "z"], "^[abd-fox-z]$"),
            case(vec!["1", "2"], "^[12]$"),
            case(vec!["1", "2", "3"], "^[1-3]$"),
            case(vec!["1", "3", "4", "5", "6"], "^[13-6]$"),
            case(vec!["1", "2", "8", "4", "5"], "^[12458]$"),
            case(vec!["1", "2", "8", "45"], "^(?:45|[128])$"),
            case(vec!["1", "2", "3", "8", "4", "5"], "^[1-58]$"),
            case(vec!["1", "2", "3", "8", "45"], "^(?:45|[1-38])$"),
            case(vec!["1", "2", "3", "5", "7", "8", "9"], "^[1-357-9]$"),
            case(vec!["a", "b", "bc"], "^(?:bc?|a)$"),
            case(vec!["a", "b", "bcd"], "^(?:b(?:cd)?|a)$"),
            case(vec!["a", "ab", "abc"], "^a(?:bc?)?$"),
            case(vec!["ac", "bc"], "^[ab]c$"),
            case(vec!["ab", "ac"], "^a[bc]$"),
            case(vec!["bc", "abc"], "^a?bc$"),
            case(vec!["ac", "abc"], "^ab?c$"),
            case(vec!["abc", "abxyc"], "^ab(?:xy)?c$"),
            case(vec!["ab", "abc"], "^abc?$"),
            case(vec!["abx", "cdx"], "^(?:ab|cd)x$"),
            case(vec!["abd", "acd"], "^a[bc]d$"),
            case(vec!["abc", "abcd"], "^abcd?$"),
            case(vec!["abc", "abcde"], "^abc(?:de)?$"),
            case(vec!["ade", "abcde"], "^a(?:bc)?de$"),
            case(vec!["abcxy", "adexy"], "^a(?:bc|de)xy$"),
            case(vec!["axy", "abcxy", "adexy"], "^a(?:(?:bc)?|de)xy$"), // goal: "^a(bc|de)?xy$"
            case(vec!["abcxy", "abcw", "efgh"], "^(?:abc(?:xy|w)|efgh)$"),
            case(vec!["abcxy", "efgh", "abcw"], "^(?:abc(?:xy|w)|efgh)$"),
            case(vec!["efgh", "abcxy", "abcw"], "^(?:abc(?:xy|w)|efgh)$"),
            case(vec!["abxy", "cxy", "efgh"], "^(?:(?:ab|c)xy|efgh)$"),
            case(vec!["abxy", "efgh", "cxy"], "^(?:(?:ab|c)xy|efgh)$"),
            case(vec!["efgh", "abxy", "cxy"], "^(?:(?:ab|c)xy|efgh)$"),
            case(vec!["aaacaac", "aac"], "^aa(?:acaa)?c$"),
            case(vec!["a", "ä", "o", "ö", "u", "ü"], "^[aouäöü]$"),
            case(vec!["y̆", "a", "z"], "^(?:y̆|[az])$"), // goal: "^[az]|y\\u{306}$"
            case(vec!["a", "b\n", "c"], "^(?:b\\n|[ac])$"),
            case(vec!["a", "b\\n", "c"], "^(?:b\\\\n|[ac])$"),
            case(vec!["[a-z]", "(d,e,f)"], "^(?:\\(d,e,f\\)|\\[a\\-z\\])$"),
            case(vec!["3.5", "4.5", "4,5"], "^(?:3\\.5|4[,.]5)$"),
            case(vec!["\u{b}"], "^\\v$"), // U+000B Line Tabulation
            case(vec!["\\u{b}"], "^\\\\u\\{b\\}$"),
            case(vec!["\u{c}"], "^\\f$"), // U+000C Form Feed
            case(vec!["\\u{c}"], "^\\\\u\\{c\\}$"),
            case(vec!["\u{200b}"], "^​$"),
            case(vec!["I ♥ cake"], "^I ♥ cake$"),
            case(vec!["I \u{2665} cake"], "^I ♥ cake$"),
            case(vec!["I \\u{2665} cake"], "^I \\\\u\\{2665\\} cake$"),
            case(vec!["I \\u2665 cake"], "^I \\\\u2665 cake$"),
            case(vec!["My ♥ is yours.", "My 💩 is yours."], "^My [♥💩] is yours\\.$"),
            case(vec!["[\u{c3e}"], "^\\[\u{c3e}$"),
            case(vec!["\\\u{10376}"], "^\\\\\u{10376}$"),
            case(vec!["I   ♥♥♥ 36 and ٣ and y̆y̆ and 💩💩."], "^I   ♥♥♥ 36 and ٣ and y̆y̆ and 💩💩\\.$"),
            case(vec!["\u{890}\0"], "^\u{890}\0$"),
            case(vec!["\u{890}\\0"], "^\u{890}\\\\0$"),
            case(vec!["\u{890}\\\0"], "^\u{890}\\\\\0$"),
            case(vec!["\u{890}\\\\0"], "^\u{890}\\\\\\\\0$"),
            case(vec!["\\𑇂"], "^\\\\𑇂$"),
            case(vec!["𑇂\\"], "^𑇂\\\\$")
        )]
        fn succeeds(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases).build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
            assert_that_regexp_matches_test_cases(expected_output, test_cases);
        }

        #[rstest(test_cases, expected_output,
            case(vec!["ABC", "abc", "AbC", "aBc"], "(?i)^abc$"),
            case(vec!["ABC", "zBC", "abc", "AbC", "aBc"], "(?i)^[az]bc$"),
            case(vec!["Ä@Ö€Ü", "ä@ö€ü", "Ä@ö€Ü", "ä@Ö€ü"], "(?i)^ä@ö€ü$"),
        )]
        fn succeeds_with_ignore_case_option(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_case_insensitive_matching()
                .build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
            assert_that_regexp_matches_test_cases(expected_output, test_cases);
        }

        #[rstest(test_cases, expected_output,
            case(vec!["My ♥ and 💩 is yours."], "^My \\u{2665} and \\u{1f4a9} is yours\\.$"),
            case(vec!["My ♥ is yours.", "My 💩 is yours."], "^My (?:\\u{2665}|\\u{1f4a9}) is yours\\.$"),
            case(
                vec!["I   ♥♥♥ 36 and ٣ and y̆y̆ and 💩💩."],
                "^I   \\u{2665}\\u{2665}\\u{2665} 36 and \\u{663} and y\\u{306}y\\u{306} and \\u{1f4a9}\\u{1f4a9}\\.$"
            )
        )]
        fn succeeds_with_escape_option(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_escaping_of_non_ascii_chars(false)
                .build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
            assert_that_regexp_matches_test_cases(expected_output, test_cases);
        }

        #[rstest(test_cases, expected_output,
            case(vec!["My ♥ and 💩 is yours."], "^My \\u{2665} and \\u{d83d}\\u{dca9} is yours\\.$"),
            case(vec!["My ♥ is yours.", "My 💩 is yours."], "^My (?:\\u{2665}|\\u{d83d}\\u{dca9}) is yours\\.$"),
            case(
                vec!["I   ♥♥♥ 36 and ٣ and y̆y̆ and 💩💩."],
                "^I   \\u{2665}\\u{2665}\\u{2665} 36 and \\u{663} and y\\u{306}y\\u{306} and \\u{d83d}\\u{dca9}\\u{d83d}\\u{dca9}\\.$"
            )
        )]
        fn succeeds_with_escape_and_surrogate_option(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_escaping_of_non_ascii_chars(true)
                .build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
        }

        #[rstest(test_cases, expected_output,
            case(vec!["a", "b", "bc"], "^(bc?|a)$"),
            case(vec!["a", "b", "bcd"], "^(b(cd)?|a)$"),
            case(vec!["a", "ab", "abc"], "^a(bc?)?$"),
            case(vec!["efgh", "abcxy", "abcw"], "^(abc(xy|w)|efgh)$"),
        )]
        fn succeeds_with_capturing_groups_option(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_capturing_groups()
                .build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
            assert_that_regexp_matches_test_cases(expected_output, test_cases);
        }

        #[rstest(test_cases, expected_output,
            case(vec![""], indoc!(
                r#"
                (?x)
                ^
                $"#
            )),
            case(vec![" "], indoc!(
                r#"
                (?x)
                ^
                  \ 
                $"#
            )),
            case(vec!["   "], indoc!(
                r#"
                (?x)
                ^
                  \ \ \ 
                $"#
            )),
            case(vec!["\u{200b}"], indoc!(
                r#"
                (?x)
                ^
                  ​
                $"#
            )),
            case(vec!["a", "b", "c"], indoc!(
                r#"
                (?x)
                ^
                  [a-c]
                $"#
            )),
            case(vec!["a", "b", "bc"], indoc!(
                r#"
                (?x)
                ^
                  (?:
                    bc?
                    |
                    a
                  )
                $"#
            )),
            case(vec!["a", "ab", "abc"], indoc!(
                r#"
                (?x)
                ^
                  a
                  (?:
                    bc?
                  )?
                $"#
            )),
            case(vec!["a", "b", "bcd"], indoc!(
                r#"
                (?x)
                ^
                  (?:
                    b
                    (?:
                      cd
                    )?
                    |
                    a
                  )
                $"#
            )),
            case(vec!["a", "b", "x", "de"], indoc!(
                r#"
                (?x)
                ^
                  (?:
                    de
                    |
                    [abx]
                  )
                $"#
            )),
            case(vec!["[a-z]", "(d,e,f)"], indoc!(
                r#"
                (?x)
                ^
                  (?:
                    \(d,e,f\)
                    |
                    \[a\-z\]
                  )
                $"#
            )),
            case(vec!["3.5", "4.5", "4,5"], indoc!(
                r#"
                (?x)
                ^
                  (?:
                    3\.5
                    |
                    4[,.]5
                  )
                $"#
            )),
            case(vec!["Ga", "G)"], indoc!(
                r#"
                (?x)
                ^
                  G[)a]
                $"#
            )),
            case(vec!["aG", ")G"], indoc!(
                r#"
                (?x)
                ^
                  [)a]G
                $"#
            )),
            case(vec!["Ga", "G)", "G("], indoc!(
                r#"
                (?x)
                ^
                  G[()a]
                $"#
            )),
            case(vec!["aG", ")G", "(G"], indoc!(
                r#"
                (?x)
                ^
                  [()a]G
                $"#
            )),
            case(vec!["aaacaac", "aac"], indoc!(
                r#"
                (?x)
                ^
                  aa
                  (?:
                    acaa
                  )?
                  c
                $"#
            )),
        )]
        fn succeeds_with_verbose_mode_option(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases).with_verbose_mode().build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
            assert_that_regexp_matches_test_cases(expected_output, test_cases);
        }

        #[rstest(test_cases, expected_output,
            case(vec!["ABC", "abc", "AbC", "aBc"], indoc!(
                r#"
                (?ix)
                ^
                  abc
                $"#
            )),
            case(vec!["ABC", "zBC", "abc", "AbC", "aBc"], indoc!(
                r#"
                (?ix)
                ^
                  [az]bc
                $"#
            )),
            case(vec!["Ä@Ö€Ü", "ä@ö€ü", "Ä@ö€Ü", "ä@Ö€ü"], indoc!(
                r#"
                (?ix)
                ^
                  ä@ö€ü
                $"#
            ))
        )]
        fn succeeds_with_ignore_case_and_verbose_mode_option(
            test_cases: Vec<&str>,
            expected_output: &str,
        ) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_case_insensitive_matching()
                .with_verbose_mode()
                .build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
            assert_that_regexp_matches_test_cases(expected_output, test_cases);
        }

        #[test]
        fn succeeds_with_file_input() {
            let mut file = NamedTempFile::new().unwrap();
            writeln!(file, "a\nb\nc\r\nxyz").unwrap();

            let expected_output = "^(?:xyz|[a-c])$";
            let test_cases = vec!["a", "b", "c", "xyz"];

            let regexp = RegExpBuilder::from_file(file.path()).build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
            assert_that_regexp_matches_test_cases(expected_output, test_cases);
        }
    }

    mod repetition {
        use super::*;

        #[rstest(test_cases, expected_output,
            case(vec![""], "^$"),
            case(vec![" "], "^ $"),
            case(vec!["   "], "^ {3}$"),
            case(vec!["a"], "^a$"),
            case(vec!["aa"], "^a{2}$"),
            case(vec!["aaa"], "^a{3}$"),
            case(vec!["aaa aaa"], "^a{3} a{3}$"),
            case(vec!["ababab ababab"], "^(?:ab){3} (?:ab){3}$"),
            case(vec!["ababab  ababab"], "^(?:ab){3} {2}(?:ab){3}$"),
            case(vec!["a ababab ababab"], "^a(?: (?:ab){3}){2}$"),
            case(vec!["ababab ababab a"], "^a(?:b(?:ab){2} a){2}$"),
            case(vec!["ababababab abab ababab"], "^ababab(?:(?:ab){2} ){2}(?:ab){3}$"),
            case(vec!["a", "aa"], "^a{1,2}$"),
            case(vec!["aaa", "a", "aa"], "^a{1,3}$"),
            case(vec!["aaaa", "a", "aa"], "^(?:a{1,2}|a{4})$"),
            case(vec!["a", "aa", "aaa", "aaaa", "aaab"], "^(?:a{3}b|a{1,4})$"),
            case(vec!["baabaaaaaabb"], "^ba{2}ba{6}b{2}$"),
            case(vec!["aabbaabbaaa"], "^(?:a{2}b{2}){2}a{3}$"),
            case(vec!["aabbaa"], "^a{2}b{2}a{2}$"),
            case(vec!["aabbabb"], "^a(?:ab{2}){2}$"),
            case(vec!["ababab"], "^(?:ab){3}$"),
            case(vec!["abababa"], "^a(?:ba){3}$"),
            case(vec!["aababab"], "^a(?:ab){3}$"),
            case(vec!["abababaa"], "^(?:ab){3}a{2}$"),
            case(vec!["aaaaaabbbbb"], "^a{6}b{5}$"),
            case(vec!["aabaababab"], "^a{2}ba(?:ab){3}$"),
            case(vec!["aaaaaaabbbbbba"], "^a{7}b{6}a$"),
            case(vec!["abaaaabaaba"], "^abaaa(?:aba){2}$"),
            case(vec!["bbaababb"], "^b{2}a{2}bab{2}$"),
            case(vec!["b", "ba"], "^ba?$"),
            case(vec!["b", "ba", "baa"], "^b(?:a{1,2})?$"),
            case(vec!["b", "ba", "baaa", "baa"], "^b(?:a{1,3})?$"),
            case(vec!["b", "ba", "baaaa", "baa"], "^b(?:a{1,2}|a{4})?$"),
            case(vec!["axy", "abcxyxy", "adexy"], "^a(?:(?:de)?xy|bc(?:xy){2})$"),
            case(vec!["xy̆y̆y̆y̆z"], "^x(?:y̆){4}z$"),
            case(vec!["xy̆y̆z", "xy̆y̆y̆z"], "^x(?:y̆){2,3}z$"),
            case(vec!["xy̆y̆z", "xy̆y̆y̆y̆z"], "^x(?:(?:y̆){2}|(?:y̆){4})z$"),
            case(vec!["zyxx", "yxx"], "^z?yx{2}$"),
            case(vec!["zyxx", "yxx", "yxxx"], "^(?:zyx{2}|yx{2,3})$"),
            case(vec!["zyxxx", "yxx", "yxxx"], "^(?:zyx{3}|yx{2,3})$"),
            case(vec!["a", "b\n\n", "c"], "^(?:b\\n{2}|[ac])$"),
            case(vec!["a", "b\nb\nb", "c"], "^(?:b(?:\\nb){2}|[ac])$"),
            case(vec!["a", "b\nx\nx", "c"], "^(?:b(?:\\nx){2}|[ac])$"),
            case(vec!["a", "b\n\t\n\t", "c"], "^(?:b(?:\\n\\t){2}|[ac])$"),
            case(vec!["a", "b\n", "b\n\n", "b\n\n\n", "c"], "^(?:b\\n{1,3}|[ac])$"),
            case(vec!["4.5", "3.55"], "^(?:4\\.5|3\\.5{2})$"),
            case(vec!["4.5", "4.55"], "^4\\.5{1,2}$"),
            case(vec!["4.5", "4.55", "3.5"], "^(?:3\\.5|4\\.5{1,2})$"),
            case(vec!["4.5", "44.5", "44.55", "4.55"], "^4{1,2}\\.5{1,2}$"),
            case(vec!["I ♥♥ cake"], "^I ♥{2} cake$"),
            case(vec!["I ♥ cake", "I ♥♥ cake"], "^I ♥{1,2} cake$"),
            case(vec!["I \u{2665}\u{2665} cake"], "^I ♥{2} cake$"),
            case(vec!["I \\u{2665} cake"], "^I \\\\u\\{26{2}5\\} cake$"),
            case(vec!["I \\u{2665}\\u{2665} cake"], "^I (?:\\\\u\\{26{2}5\\}){2} cake$"),
            case(vec!["I \\u2665\\u2665 cake"], "^I (?:\\\\u26{2}5){2} cake$"),
            case(vec!["My ♥♥♥ is yours.", "My 💩💩 is yours."], "^My (?:💩{2}|♥{3}) is yours\\.$"),
            case(vec!["I   ♥♥♥ 36 and ٣ and y̆y̆ and 💩💩."], "^I {3}♥{3} 36 and ٣ and (?:y̆){2} and 💩{2}\\.$")
        )]
        fn succeeds(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of_repetitions()
                .build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
            assert_that_regexp_matches_test_cases(expected_output, test_cases);
        }

        #[rstest(test_cases, expected_output,
            case(vec!["AAAAB", "aaaab", "AaAaB", "aAaAB"], "(?i)^a{4}b$"),
            case(vec!["ÄÖÜäöü@Ö€", "äöüÄöÜ@ö€"], "(?i)^(?:äöü){2}@ö€$"),
        )]
        fn succeeds_with_ignore_case_option(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of_repetitions()
                .with_case_insensitive_matching()
                .build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
            assert_that_regexp_matches_test_cases(expected_output, test_cases);
        }

        #[rstest(test_cases, expected_output,
            case(vec!["My ♥♥♥ and 💩💩 is yours."], "^My \\u{2665}{3} and \\u{1f4a9}{2} is yours\\.$"),
            case(vec!["My ♥♥♥ is yours.", "My 💩💩 is yours."], "^My (?:\\u{1f4a9}{2}|\\u{2665}{3}) is yours\\.$"),
            case(
                vec!["I   ♥♥♥ 36 and ٣ and y̆y̆ and 💩💩."],
                "^I {3}\\u{2665}{3} 36 and \\u{663} and (?:y\\u{306}){2} and \\u{1f4a9}{2}\\.$"
            )
        )]
        fn succeeds_with_escape_option(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of_repetitions()
                .with_escaping_of_non_ascii_chars(false)
                .build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
            assert_that_regexp_matches_test_cases(expected_output, test_cases);
        }

        #[rstest(test_cases, expected_output,
            case(vec!["My ♥♥♥ and 💩💩 is yours."], "^My \\u{2665}{3} and (?:\\u{d83d}\\u{dca9}){2} is yours\\.$"),
            case(vec!["My ♥♥♥ is yours.", "My 💩💩 is yours."], "^My (?:(?:\\u{d83d}\\u{dca9}){2}|\\u{2665}{3}) is yours\\.$"),
            case(
                vec!["I   ♥♥♥ 36 and ٣ and y̆y̆ and 💩💩."],
                "^I {3}\\u{2665}{3} 36 and \\u{663} and (?:y\\u{306}){2} and (?:\\u{d83d}\\u{dca9}){2}\\.$"
            )
        )]
        fn succeeds_with_escape_and_surrogate_option(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of_repetitions()
                .with_escaping_of_non_ascii_chars(true)
                .build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
        }

        #[rstest(test_cases, expected_output,
            case(vec!["   "], indoc!(
                r#"
                (?x)
                ^
                  \ {3}
                $"#
            )),
            case(vec!["aa"], indoc!(
                r#"
                (?x)
                ^
                  a{2}
                $"#
            )),
            case(vec!["aaa", "a", "aa"], indoc!(
                r#"
                (?x)
                ^
                  a{1,3}
                $"#
            )),
            case(vec!["aaaa", "a", "aa"], indoc!(
                r#"
                (?x)
                ^
                  (?:
                    a{1,2}
                    |
                    a{4}
                  )
                $"#
            )),
            case(vec!["ababab"], indoc!(
                r#"
                (?x)
                ^
                  (?:
                    ab
                  ){3}
                $"#
            )),
            case(vec!["abababa"], indoc!(
                r#"
                (?x)
                ^
                  a
                  (?:
                    ba
                  ){3}
                $"#
            )),
            case(vec!["abababaa"], indoc!(
                r#"
                (?x)
                ^
                  (?:
                    ab
                  ){3}
                  a{2}
                $"#
            )),
            case(vec!["aabaababab"], indoc!(
                r#"
                (?x)
                ^
                  a{2}ba
                  (?:
                    ab
                  ){3}
                $"#
            )),
            case(vec!["abaaaabaaba"], indoc!(
                r#"
                (?x)
                ^
                  abaaa
                  (?:
                    aba
                  ){2}
                $"#
            )),
            case(vec!["xy̆y̆z", "xy̆y̆y̆y̆z"], indoc!(
                r#"
                (?x)
                ^
                  x
                  (?:
                    (?:
                      y̆
                    ){2}
                    |
                    (?:
                      y̆
                    ){4}
                  )
                  z
                $"#
            )),
            case(vec!["a", "b\n\t\n\t", "c"], indoc!(
                r#"
                (?x)
                ^
                  (?:
                    b
                    (?:
                      \n\t
                    ){2}
                    |
                    [ac]
                  )
                $"#
            )),
            case(vec!["My ♥♥♥ is yours.", "My 💩💩 is yours."], indoc!(
                r#"
                (?x)
                ^
                  My\ 
                  (?:
                    💩{2}
                    |
                    ♥{3}
                  )
                  \ is\ yours\.
                $"#
            ))
        )]
        fn succeeds_with_verbose_mode_option(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of_repetitions()
                .with_verbose_mode()
                .build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
            assert_that_regexp_matches_test_cases(expected_output, test_cases);
        }

        #[rstest(test_cases, expected_output,
            case(vec![""], "^$"),
            case(vec![" "], "^ $"),
            case(vec!["   "], "^   $"),
            case(vec!["    "], "^ {4}$"),
            case(vec!["      "], "^ {6}$"),
            case(vec!["a"], "^a$"),
            case(vec!["aa"], "^aa$"),
            case(vec!["aaa"], "^aaa$"),
            case(vec!["aaaa"], "^a{4}$"),
            case(vec!["aaaaa"], "^a{5}$"),
            case(vec!["ababababab abab ababab"], "^(?:ab){5} abab ababab$"),
            case(vec!["aabbaaaabbbabbbbba"], "^aabba{4}bbbab{5}a$"),
            case(vec!["baabaaaaaabb"], "^baaba{6}bb$"),
            case(vec!["ababab"], "^ababab$"),
            case(vec!["abababab"], "^(?:ab){4}$"),
            case(vec!["abababa"], "^abababa$"),
            case(vec!["ababababa"], "^a(?:ba){4}$"),
            case(vec!["aababab"], "^aababab$"),
            case(vec!["aabababab"], "^a(?:ab){4}$"),
            case(vec!["xy̆y̆z", "xy̆y̆y̆y̆z"], "^x(?:y̆y̆|(?:y̆){4})z$"),
            case(vec!["aaa", "a", "aa"], "^a(?:aa?)?$"),
            case(vec!["a", "aa", "aaa", "aaaa"], "^(?:aaa|aa?|a{4})$"),
            case(vec!["a", "aa", "aaa", "aaaa", "aaaaa", "aaaaaa"], "^(?:aaa|aa?|a{4,6})$")
        )]
        fn succeeds_with_increased_minimum_repetitions(
            test_cases: Vec<&str>,
            expected_output: &str,
        ) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of_repetitions()
                .with_minimum_repetitions(3)
                .build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
            assert_that_regexp_matches_test_cases(expected_output, test_cases);
        }

        #[rstest(test_cases, expected_output,
            case(vec!["aaa"], "^aaa$"),
            case(vec!["ababab"], "^ababab$"),
            case(vec!["abcabcabc"], "^(?:abc){3}$"),
            case(vec!["abcabcabc", "dede"], "^(?:dede|(?:abc){3})$"),
            case(vec!["abcabcabc", "defgdefg"], "^(?:(?:defg){2}|(?:abc){3})$"),
            case(vec!["ababababab abab ababab"], "^ababab(?:abab ){2}ababab$")
        )]
        fn succeeds_with_increased_minimum_substring_length(
            test_cases: Vec<&str>,
            expected_output: &str,
        ) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of_repetitions()
                .with_minimum_substring_length(3)
                .build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
            assert_that_regexp_matches_test_cases(expected_output, test_cases);
        }

        #[rstest(test_cases, expected_output,
            case(vec!["abababab"], "^abababab$"),
            case(vec!["abcabcabc"], "^abcabcabc$"),
            case(vec!["abcabcabcabc"], "^(?:abc){4}$"),
            case(vec!["aaaaaaaaaaaa"], "^aaaaaaaaaaaa$"),
            case(vec!["abababab", "abcabcabcabc"], "^(?:abababab|(?:abc){4})$"),
            case(vec!["ababababab abab ababab"], "^ababababab abab ababab$")
        )]
        fn succeeds_with_increased_minimum_repetitions_and_substring_length(
            test_cases: Vec<&str>,
            expected_output: &str,
        ) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of_repetitions()
                .with_minimum_repetitions(3)
                .with_minimum_substring_length(3)
                .build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
            assert_that_regexp_matches_test_cases(expected_output, test_cases);
        }
    }
}

mod digit_conversion {
    use super::*;

    mod no_repetition {
        use super::*;

        #[rstest(test_cases, expected_output,
            case(vec![""], "^$"),
            case(vec!["a"], "^a$"),
            case(vec!["1"], "^\\d$"),
            case(vec!["-1"], "^\\-\\d$"),
            case(vec!["12"], "^\\d\\d$"),
            case(vec!["1", "2"], "^\\d$"),
            case(vec!["1", "23"], "^\\d(?:\\d)?$"),
            case(vec!["1", "234"], "^\\d(?:\\d\\d)?$"),
            case(vec!["8", "234"], "^\\d(?:\\d\\d)?$"),
            case(vec!["890", "34"], "^\\d\\d(?:\\d)?$"),
            case(vec!["abc123"], "^abc\\d\\d\\d$"),
            case(vec!["a1b2c3"], "^a\\db\\dc\\d$"),
            case(vec!["abc", "123"], "^(?:\\d\\d\\d|abc)$"),
            case(vec!["١", "٣", "٥"], "^\\d$"), // Arabic digits: ١ = 1, ٣ = 3, ٥ = 5
            case(vec!["١٣٥"], "^\\d\\d\\d$"),
            case(vec!["a٣3", "b5٥"], "^[ab]\\d\\d$"),
            case(vec!["I ♥ 123"], "^I ♥ \\d\\d\\d$"),
            case(vec!["I   ♥♥♥ 36 and ٣ and y̆y̆ and 💩💩."], "^I   ♥♥♥ \\d\\d and \\d and y̆y̆ and 💩💩\\.$")
        )]
        fn succeeds(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of_digits()
                .build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
            assert_that_regexp_matches_test_cases(expected_output, test_cases);
        }

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and y̆y̆ and 💩💩."],
                "^I   \\u{2665}\\u{2665}\\u{2665} \\d\\d and \\d and y\\u{306}y\\u{306} and \\u{1f4a9}\\u{1f4a9}\\.$"
            )
        )]
        fn succeeds_with_escape_option(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of_digits()
                .with_escaping_of_non_ascii_chars(false)
                .build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
            assert_that_regexp_matches_test_cases(expected_output, test_cases);
        }

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and y̆y̆ and 💩💩."],
                "^I   \\u{2665}\\u{2665}\\u{2665} \\d\\d and \\d and y\\u{306}y\\u{306} and \\u{d83d}\\u{dca9}\\u{d83d}\\u{dca9}\\.$"
            )
        )]
        fn succeeds_with_escape_and_surrogate_option(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of_digits()
                .with_escaping_of_non_ascii_chars(true)
                .build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
        }
    }

    mod repetition {
        use super::*;

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and y̆y̆ and 💩💩."],
                "^I {3}♥{3} \\d(?:\\d and ){2}(?:y̆){2} and 💩{2}\\.$"
            )
        )]
        fn succeeds(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of_repetitions()
                .with_conversion_of_digits()
                .build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
            assert_that_regexp_matches_test_cases(expected_output, test_cases);
        }

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and y̆y̆ and 💩💩."],
                "^I {3}\\u{2665}{3} \\d(?:\\d and ){2}(?:y\\u{306}){2} and \\u{1f4a9}{2}\\.$"
            )
        )]
        fn succeeds_with_escape_option(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of_repetitions()
                .with_conversion_of_digits()
                .with_escaping_of_non_ascii_chars(false)
                .build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
            assert_that_regexp_matches_test_cases(expected_output, test_cases);
        }

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and y̆y̆ and 💩💩."],
                "^I {3}\\u{2665}{3} \\d(?:\\d and ){2}(?:y\\u{306}){2} and (?:\\u{d83d}\\u{dca9}){2}\\.$"
            )
        )]
        fn succeeds_with_escape_and_surrogate_option(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of_repetitions()
                .with_conversion_of_digits()
                .with_escaping_of_non_ascii_chars(true)
                .build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
        }

        #[rstest(test_cases, expected_output,
            case(vec!["1"], "^\\d$"),
            case(vec!["12"], "^\\d\\d$"),
            case(vec!["123"], "^\\d{3}$"),
            case(vec!["1", "12", "123"], "^(?:\\d\\d|\\d|\\d{3})$"),
            case(vec!["12", "123", "1234"], "^(?:\\d\\d|\\d{3,4})$"),
            case(vec!["123", "1234", "12345"], "^\\d{3,5}$"),
            case(vec!["I   ♥♥♥ 36 and ٣ and y̆y̆ and 💩💩."], "^I {3}♥{3} \\d\\d and \\d and y̆y̆ and 💩💩\\.$")
        )]
        fn succeeds_with_increased_minimum_repetitions(
            test_cases: Vec<&str>,
            expected_output: &str,
        ) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of_repetitions()
                .with_conversion_of_digits()
                .with_minimum_repetitions(2)
                .build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
            assert_that_regexp_matches_test_cases(expected_output, test_cases);
        }
    }
}

mod space_conversion {
    use super::*;

    mod no_repetition {
        use super::*;

        #[rstest(test_cases, expected_output,
            case(vec![""], "^$"),
            case(vec![" "], "^\\s$"),
            case(vec!["   "], "^\\s\\s\\s$"),
            case(vec!["\n"], "^\\s$"),
            case(vec!["\u{c}"], "^\\s$"), // form feed \f
            case(vec!["\u{b}"], "^\\s$"), // vertical tab \v
            case(vec!["\n", "\r"], "^\\s$"),
            case(vec!["\n\t", "\r"], "^\\s(?:\\s)?$"),
            case(vec!["a"], "^a$"),
            case(vec!["1"], "^1$"),
            case(vec!["I ♥ 123"], "^I\\s♥\\s123$"),
            case(vec!["I   ♥♥♥ 36 and ٣ and y̆y̆ and 💩💩."], "^I\\s\\s\\s♥♥♥\\s36\\sand\\s٣\\sand\\sy̆y̆\\sand\\s💩💩\\.$")
        )]
        fn succeeds(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of_whitespace()
                .build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
            assert_that_regexp_matches_test_cases(expected_output, test_cases);
        }

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and y̆y̆ and 💩💩."],
                "^I\\s\\s\\s\\u{2665}\\u{2665}\\u{2665}\\s36\\sand\\s\\u{663}\\sand\\sy\\u{306}y\\u{306}\\sand\\s\\u{1f4a9}\\u{1f4a9}\\.$"
            )
        )]
        fn succeeds_with_escape_option(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of_whitespace()
                .with_escaping_of_non_ascii_chars(false)
                .build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
            assert_that_regexp_matches_test_cases(expected_output, test_cases);
        }

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and y̆y̆ and 💩💩."],
                "^I\\s\\s\\s\\u{2665}\\u{2665}\\u{2665}\\s36\\sand\\s\\u{663}\\sand\\sy\\u{306}y\\u{306}\\sand\\s\\u{d83d}\\u{dca9}\\u{d83d}\\u{dca9}\\.$"
            )
        )]
        fn succeeds_with_escape_and_surrogate_option(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of_whitespace()
                .with_escaping_of_non_ascii_chars(true)
                .build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
        }
    }

    mod repetition {
        use super::*;

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and y̆y̆ and 💩💩."],
                "^I\\s{3}♥{3}\\s36\\sand\\s٣\\sand\\s(?:y̆){2}\\sand\\s💩{2}\\.$"
            )
        )]
        fn succeeds(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of_repetitions()
                .with_conversion_of_whitespace()
                .build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
            assert_that_regexp_matches_test_cases(expected_output, test_cases);
        }

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and y̆y̆ and 💩💩."],
                "^I\\s{3}\\u{2665}{3}\\s36\\sand\\s\\u{663}\\sand\\s(?:y\\u{306}){2}\\sand\\s\\u{1f4a9}{2}\\.$"
            )
        )]
        fn succeeds_with_escape_option(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of_repetitions()
                .with_conversion_of_whitespace()
                .with_escaping_of_non_ascii_chars(false)
                .build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
            assert_that_regexp_matches_test_cases(expected_output, test_cases);
        }

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and y̆y̆ and 💩💩."],
                "^I\\s{3}\\u{2665}{3}\\s36\\sand\\s\\u{663}\\sand\\s(?:y\\u{306}){2}\\sand\\s(?:\\u{d83d}\\u{dca9}){2}\\.$"
            )
        )]
        fn succeeds_with_escape_and_surrogate_option(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of_repetitions()
                .with_conversion_of_whitespace()
                .with_escaping_of_non_ascii_chars(true)
                .build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
        }

        #[rstest(test_cases, expected_output,
            case(vec![" "], "^\\s$"),
            case(vec!["  "], "^\\s\\s$"),
            case(vec!["   "], "^\\s{3}$"),
            case(vec![" ", "  ", "   "], "^(?:\\s\\s|\\s|\\s{3})$"),
            case(vec!["  ", "   ", "    "], "^(?:\\s\\s|\\s{3,4})$"),
            case(vec!["   ", "    ", "     "], "^\\s{3,5}$"),
            case(
                vec!["I   ♥♥♥ 36 and ٣ and y̆y̆ and 💩💩."],
                "^I\\s{3}♥{3}\\s36\\sand\\s٣\\sand\\sy\u{306}y\u{306}\\sand\\s💩💩\\.$"
            )
        )]
        fn succeeds_with_increased_minimum_repetitions(
            test_cases: Vec<&str>,
            expected_output: &str,
        ) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of_repetitions()
                .with_conversion_of_whitespace()
                .with_minimum_repetitions(2)
                .build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
            assert_that_regexp_matches_test_cases(expected_output, test_cases);
        }
    }
}

mod word_conversion {
    use super::*;

    mod no_repetition {
        use super::*;

        #[rstest(test_cases, expected_output,
            case(vec![""], "^$"),
            case(vec![" "], "^ $"),
            case(vec!["a"], "^\\w$"),
            case(vec!["1"], "^\\w$"),
            case(vec!["-1"], "^\\-\\w$"),
            case(vec!["1", "2"], "^\\w$"),
            case(vec!["ä", "ß"], "^\\w$"),
            case(vec!["abc", "1234"], "^\\w\\w\\w(?:\\w)?$"),
            case(vec!["١", "٣", "٥"], "^\\w$"), // Arabic digits: ١ = 1, ٣ = 3, ٥ = 5
            case(vec!["١٣٥"], "^\\w\\w\\w$"),
            case(vec!["a٣3", "b5٥"], "^\\w\\w\\w$"),
            case(vec!["I ♥ 123"], "^\\w ♥ \\w\\w\\w$"),
            case(
                vec!["I   ♥♥♥ 36 and ٣ and y̆y̆ and 💩💩."],
                "^\\w   ♥♥♥ \\w\\w \\w\\w\\w \\w \\w\\w\\w \\w\\w\\w\\w \\w\\w\\w 💩💩\\.$"
            )
        )]
        fn succeeds(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of_words()
                .build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
            assert_that_regexp_matches_test_cases(expected_output, test_cases);
        }

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and y̆y̆ and 💩💩."],
                "^\\w   \\u{2665}\\u{2665}\\u{2665} \\w\\w \\w\\w\\w \\w \\w\\w\\w \\w\\w\\w\\w \\w\\w\\w \\u{1f4a9}\\u{1f4a9}\\.$"
            )
        )]
        fn succeeds_with_escape_option(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of_words()
                .with_escaping_of_non_ascii_chars(false)
                .build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
            assert_that_regexp_matches_test_cases(expected_output, test_cases);
        }

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and y̆y̆ and 💩💩."],
                "^\\w   \\u{2665}\\u{2665}\\u{2665} \\w\\w \\w\\w\\w \\w \\w\\w\\w \\w\\w\\w\\w \\w\\w\\w \\u{d83d}\\u{dca9}\\u{d83d}\\u{dca9}\\.$"
            )
        )]
        fn succeeds_with_escape_and_surrogate_option(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of_words()
                .with_escaping_of_non_ascii_chars(true)
                .build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
        }
    }

    mod repetition {
        use super::*;

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and y̆y̆ and 💩💩."],
                "^\\w {3}♥{3} \\w{2}(?: \\w{3} \\w){2}(?:\\w{3} ){2}💩{2}\\.$"
            )
        )]
        fn succeeds(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of_repetitions()
                .with_conversion_of_words()
                .build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
            assert_that_regexp_matches_test_cases(expected_output, test_cases);
        }

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and y̆y̆ and 💩💩."],
                "^\\w {3}\\u{2665}{3} \\w{2}(?: \\w{3} \\w){2}(?:\\w{3} ){2}\\u{1f4a9}{2}\\.$"
            )
        )]
        fn succeeds_with_escape_option(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of_repetitions()
                .with_conversion_of_words()
                .with_escaping_of_non_ascii_chars(false)
                .build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
            assert_that_regexp_matches_test_cases(expected_output, test_cases);
        }

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and y̆y̆ and 💩💩."],
                "^\\w {3}\\u{2665}{3} \\w{2}(?: \\w{3} \\w){2}(?:\\w{3} ){2}(?:\\u{d83d}\\u{dca9}){2}\\.$"
            )
        )]
        fn succeeds_with_escape_and_surrogate_option(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of_repetitions()
                .with_conversion_of_words()
                .with_escaping_of_non_ascii_chars(true)
                .build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
        }

        #[rstest(test_cases, expected_output,
            case(vec!["a"], "^\\w$"),
            case(vec!["ab"], "^\\w\\w$"),
            case(vec!["abc"], "^\\w{3}$"),
            case(vec!["a", "ab", "abc"], "^(?:\\w\\w|\\w|\\w{3})$"),
            case(vec!["ab", "abc", "abcd"], "^(?:\\w\\w|\\w{3,4})$"),
            case(vec!["abc", "abcd", "abcde"], "^\\w{3,5}$"),
            case(
                vec!["I   ♥♥♥ 36 and ٣ and y̆y̆ and 💩💩."],
                "^\\w {3}♥{3} \\w\\w \\w{3} \\w \\w{3} \\w{4} \\w{3} 💩💩\\.$"
            )
        )]
        fn succeeds_with_increased_minimum_repetitions(
            test_cases: Vec<&str>,
            expected_output: &str,
        ) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of_repetitions()
                .with_conversion_of_words()
                .with_minimum_repetitions(2)
                .build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
            assert_that_regexp_matches_test_cases(expected_output, test_cases);
        }
    }
}

mod digit_space_conversion {
    use super::*;

    mod no_repetition {
        use super::*;

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and y̆y̆ and 💩💩."],
                "^I\\s\\s\\s♥♥♥\\s\\d\\d\\sand\\s\\d\\sand\\sy̆y̆\\sand\\s💩💩\\.$"
            )
        )]
        fn succeeds(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of_digits()
                .with_conversion_of_whitespace()
                .build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
            assert_that_regexp_matches_test_cases(expected_output, test_cases);
        }

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and y̆y̆ and 💩💩."],
                "^I\\s\\s\\s\\u{2665}\\u{2665}\\u{2665}\\s\\d\\d\\sand\\s\\d\\sand\\sy\\u{306}y\\u{306}\\sand\\s\\u{1f4a9}\\u{1f4a9}\\.$"
            )
        )]
        fn succeeds_with_escape_option(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of_digits()
                .with_conversion_of_whitespace()
                .with_escaping_of_non_ascii_chars(false)
                .build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
            assert_that_regexp_matches_test_cases(expected_output, test_cases);
        }

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and y̆y̆ and 💩💩."],
                "^I\\s\\s\\s\\u{2665}\\u{2665}\\u{2665}\\s\\d\\d\\sand\\s\\d\\sand\\sy\\u{306}y\\u{306}\\sand\\s\\u{d83d}\\u{dca9}\\u{d83d}\\u{dca9}\\.$"
            )
        )]
        fn succeeds_with_escape_and_surrogate_option(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of_digits()
                .with_conversion_of_whitespace()
                .with_escaping_of_non_ascii_chars(true)
                .build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
        }
    }

    mod repetition {
        use super::*;

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and y̆y̆ and 💩💩."],
                "^I\\s{3}♥{3}\\s\\d(?:\\d\\sand\\s){2}(?:y̆){2}\\sand\\s💩{2}\\.$"
            )
        )]
        fn succeeds(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of_repetitions()
                .with_conversion_of_digits()
                .with_conversion_of_whitespace()
                .build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
            assert_that_regexp_matches_test_cases(expected_output, test_cases);
        }

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and y̆y̆ and 💩💩."],
                "^I\\s{3}\\u{2665}{3}\\s\\d(?:\\d\\sand\\s){2}(?:y\\u{306}){2}\\sand\\s\\u{1f4a9}{2}\\.$"
            )
        )]
        fn succeeds_with_escape_option(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of_repetitions()
                .with_conversion_of_digits()
                .with_conversion_of_whitespace()
                .with_escaping_of_non_ascii_chars(false)
                .build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
            assert_that_regexp_matches_test_cases(expected_output, test_cases);
        }

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and y̆y̆ and 💩💩."],
                "^I\\s{3}\\u{2665}{3}\\s\\d(?:\\d\\sand\\s){2}(?:y\\u{306}){2}\\sand\\s(?:\\u{d83d}\\u{dca9}){2}\\.$"
            )
        )]
        fn succeeds_with_escape_and_surrogate_option(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of_repetitions()
                .with_conversion_of_digits()
                .with_conversion_of_whitespace()
                .with_escaping_of_non_ascii_chars(true)
                .build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
        }

        #[rstest(test_cases, expected_output,
            case(vec!["1\n"], "^\\d\\s$"),
            case(vec!["1\n1\n"], "^\\d\\s\\d\\s$"),
            case(vec!["1\n1\n1\n"], "^(?:\\d\\s){3}$"),
            case(vec!["1\n", "1\n1\n", "1\n1\n1\n"], "^(?:\\d\\s\\d\\s|\\d\\s|(?:\\d\\s){3})$"),
            case(vec!["1\n1\n", "1\n1\n1\n", "1\n1\n1\n1\n"], "^(?:\\d\\s\\d\\s|(?:\\d\\s){3,4})$"),
            case(vec!["1\n1\n1\n", "1\n1\n1\n1\n", "1\n1\n1\n1\n1\n"], "^(?:\\d\\s){3,5}$"),
            case(
                vec!["I   ♥♥♥ 36 and ٣ and y̆y̆ and 💩💩."],
                "^I\\s{3}♥{3}\\s\\d\\d\\sand\\s\\d\\sand\\sy̆y̆\\sand\\s💩💩\\.$"
            )
        )]
        fn succeeds_with_increased_minimum_repetitions(
            test_cases: Vec<&str>,
            expected_output: &str,
        ) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of_repetitions()
                .with_conversion_of_digits()
                .with_conversion_of_whitespace()
                .with_minimum_repetitions(2)
                .build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
            assert_that_regexp_matches_test_cases(expected_output, test_cases);
        }

        #[rstest(test_cases, expected_output,
            case(vec!["1\n1\n"], "^1\\n1\\n$"),
            case(vec!["1\n\n1\n\n"], "^(?:1\\n\\n){2}$")
        )]
        fn succeeds_with_increased_minimum_substring_length(
            test_cases: Vec<&str>,
            expected_output: &str,
        ) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of_repetitions()
                .with_minimum_substring_length(3)
                .build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
            assert_that_regexp_matches_test_cases(expected_output, test_cases);
        }

        #[rstest(test_cases, expected_output,
            case(vec!["1\n1\n"], "^1\\n1\\n$"),
            case(vec!["1\n1\n1\n"], "^1\\n1\\n1\\n$"),
            case(vec!["1\n\n1\n\n"], "^1\\n\\n1\\n\\n$"),
            case(vec!["1\n\n1\n\n1\n\n"], "^(?:1\\n\\n){3}$")
        )]
        fn succeeds_with_increased_minimum_repetitions_and_substring_length(
            test_cases: Vec<&str>,
            expected_output: &str,
        ) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of_repetitions()
                .with_minimum_repetitions(2)
                .with_minimum_substring_length(3)
                .build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
            assert_that_regexp_matches_test_cases(expected_output, test_cases);
        }
    }
}

mod digit_word_conversion {
    use super::*;

    mod no_repetition {
        use super::*;

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and y̆y̆ and 💩💩."],
                "^\\w   ♥♥♥ \\d\\d \\w\\w\\w \\d \\w\\w\\w \\w\\w\\w\\w \\w\\w\\w 💩💩\\.$"
            )
        )]
        fn succeeds(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of_digits()
                .with_conversion_of_words()
                .build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
            assert_that_regexp_matches_test_cases(expected_output, test_cases);
        }

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and y̆y̆ and 💩💩."],
                "^\\w   \\u{2665}\\u{2665}\\u{2665} \\d\\d \\w\\w\\w \\d \\w\\w\\w \\w\\w\\w\\w \\w\\w\\w \\u{1f4a9}\\u{1f4a9}\\.$"
            )
        )]
        fn succeeds_with_escape_option(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of_digits()
                .with_conversion_of_words()
                .with_escaping_of_non_ascii_chars(false)
                .build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
            assert_that_regexp_matches_test_cases(expected_output, test_cases);
        }

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and y̆y̆ and 💩💩."],
                "^\\w   \\u{2665}\\u{2665}\\u{2665} \\d\\d \\w\\w\\w \\d \\w\\w\\w \\w\\w\\w\\w \\w\\w\\w \\u{d83d}\\u{dca9}\\u{d83d}\\u{dca9}\\.$"
            )
        )]
        fn succeeds_with_escape_and_surrogate_option(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of_digits()
                .with_conversion_of_words()
                .with_escaping_of_non_ascii_chars(true)
                .build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
        }
    }

    mod repetition {
        use super::*;

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and y̆y̆ and 💩💩."],
                "^\\w {3}♥{3} \\d(?:\\d \\w{3} ){2}\\w(?:\\w{3} ){2}💩{2}\\.$"
            )
        )]
        fn succeeds(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of_repetitions()
                .with_conversion_of_digits()
                .with_conversion_of_words()
                .build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
            assert_that_regexp_matches_test_cases(expected_output, test_cases);
        }

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and y̆y̆ and 💩💩."],
                "^\\w {3}\\u{2665}{3} \\d(?:\\d \\w{3} ){2}\\w(?:\\w{3} ){2}\\u{1f4a9}{2}\\.$"
            )
        )]
        fn succeeds_with_escape_option(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of_repetitions()
                .with_conversion_of_digits()
                .with_conversion_of_words()
                .with_escaping_of_non_ascii_chars(false)
                .build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
            assert_that_regexp_matches_test_cases(expected_output, test_cases);
        }

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and y̆y̆ and 💩💩."],
                "^\\w {3}\\u{2665}{3} \\d(?:\\d \\w{3} ){2}\\w(?:\\w{3} ){2}(?:\\u{d83d}\\u{dca9}){2}\\.$"
            )
        )]
        fn succeeds_with_escape_and_surrogate_option(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of_repetitions()
                .with_conversion_of_digits()
                .with_conversion_of_words()
                .with_escaping_of_non_ascii_chars(true)
                .build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
        }
    }
}

mod space_word_conversion {
    use super::*;

    mod no_repetition {
        use super::*;

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and y̆y̆ and 💩💩."],
                "^\\w\\s\\s\\s♥♥♥\\s\\w\\w\\s\\w\\w\\w\\s\\w\\s\\w\\w\\w\\s\\w\\w\\w\\w\\s\\w\\w\\w\\s💩💩\\.$"
            )
        )]
        fn succeeds(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of_whitespace()
                .with_conversion_of_words()
                .build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
            assert_that_regexp_matches_test_cases(expected_output, test_cases);
        }

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and y̆y̆ and 💩💩."],
                "^\\w\\s\\s\\s\\u{2665}\\u{2665}\\u{2665}\\s\\w\\w\\s\\w\\w\\w\\s\\w\\s\\w\\w\\w\\s\\w\\w\\w\\w\\s\\w\\w\\w\\s\\u{1f4a9}\\u{1f4a9}\\.$"
            )
        )]
        fn succeeds_with_escape_option(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of_whitespace()
                .with_conversion_of_words()
                .with_escaping_of_non_ascii_chars(false)
                .build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
            assert_that_regexp_matches_test_cases(expected_output, test_cases);
        }

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and y̆y̆ and 💩💩."],
                "^\\w\\s\\s\\s\\u{2665}\\u{2665}\\u{2665}\\s\\w\\w\\s\\w\\w\\w\\s\\w\\s\\w\\w\\w\\s\\w\\w\\w\\w\\s\\w\\w\\w\\s\\u{d83d}\\u{dca9}\\u{d83d}\\u{dca9}\\.$"
            )
        )]
        fn succeeds_with_escape_and_surrogate_option(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of_whitespace()
                .with_conversion_of_words()
                .with_escaping_of_non_ascii_chars(true)
                .build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
        }
    }

    mod repetition {
        use super::*;

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and y̆y̆ and 💩💩."],
                "^\\w\\s{3}♥{3}\\s\\w{2}(?:\\s\\w{3}\\s\\w){2}(?:\\w{3}\\s){2}💩{2}\\.$"
            )
        )]
        fn succeeds(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of_repetitions()
                .with_conversion_of_whitespace()
                .with_conversion_of_words()
                .build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
            assert_that_regexp_matches_test_cases(expected_output, test_cases);
        }

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and y̆y̆ and 💩💩."],
                "^\\w\\s{3}\\u{2665}{3}\\s\\w{2}(?:\\s\\w{3}\\s\\w){2}(?:\\w{3}\\s){2}\\u{1f4a9}{2}\\.$"
            )
        )]
        fn succeeds_with_escape_option(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of_repetitions()
                .with_conversion_of_whitespace()
                .with_conversion_of_words()
                .with_escaping_of_non_ascii_chars(false)
                .build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
            assert_that_regexp_matches_test_cases(expected_output, test_cases);
        }

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and y̆y̆ and 💩💩."],
                "^\\w\\s{3}\\u{2665}{3}\\s\\w{2}(?:\\s\\w{3}\\s\\w){2}(?:\\w{3}\\s){2}(?:\\u{d83d}\\u{dca9}){2}\\.$"
            )
        )]
        fn succeeds_with_escape_and_surrogate_option(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of_repetitions()
                .with_conversion_of_whitespace()
                .with_conversion_of_words()
                .with_escaping_of_non_ascii_chars(true)
                .build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
        }
    }
}

mod digit_space_word_conversion {
    use super::*;

    mod no_repetition {
        use super::*;

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and y̆y̆ and 💩💩."],
                "^\\w\\s\\s\\s♥♥♥\\s\\d\\d\\s\\w\\w\\w\\s\\d\\s\\w\\w\\w\\s\\w\\w\\w\\w\\s\\w\\w\\w\\s💩💩\\.$"
            )
        )]
        fn succeeds(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of_digits()
                .with_conversion_of_whitespace()
                .with_conversion_of_words()
                .build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
            assert_that_regexp_matches_test_cases(expected_output, test_cases);
        }

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and y̆y̆ and 💩💩."],
                "^\\w\\s\\s\\s\\u{2665}\\u{2665}\\u{2665}\\s\\d\\d\\s\\w\\w\\w\\s\\d\\s\\w\\w\\w\\s\\w\\w\\w\\w\\s\\w\\w\\w\\s\\u{1f4a9}\\u{1f4a9}\\.$"
            )
        )]
        fn succeeds_with_escape_option(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of_digits()
                .with_conversion_of_whitespace()
                .with_conversion_of_words()
                .with_escaping_of_non_ascii_chars(false)
                .build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
            assert_that_regexp_matches_test_cases(expected_output, test_cases);
        }

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and y̆y̆ and 💩💩."],
                "^\\w\\s\\s\\s\\u{2665}\\u{2665}\\u{2665}\\s\\d\\d\\s\\w\\w\\w\\s\\d\\s\\w\\w\\w\\s\\w\\w\\w\\w\\s\\w\\w\\w\\s\\u{d83d}\\u{dca9}\\u{d83d}\\u{dca9}\\.$"
            )
        )]
        fn succeeds_with_escape_and_surrogate_option(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of_digits()
                .with_conversion_of_whitespace()
                .with_conversion_of_words()
                .with_escaping_of_non_ascii_chars(true)
                .build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
        }
    }

    mod repetition {
        use super::*;

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and y̆y̆ and 💩💩."],
                "^\\w\\s{3}♥{3}\\s\\d(?:\\d\\s\\w{3}\\s){2}\\w(?:\\w{3}\\s){2}💩{2}\\.$"
            )
        )]
        fn succeeds(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of_repetitions()
                .with_conversion_of_digits()
                .with_conversion_of_whitespace()
                .with_conversion_of_words()
                .build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
            assert_that_regexp_matches_test_cases(expected_output, test_cases);
        }

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and y̆y̆ and 💩💩."],
                "^\\w\\s{3}\\u{2665}{3}\\s\\d(?:\\d\\s\\w{3}\\s){2}\\w(?:\\w{3}\\s){2}\\u{1f4a9}{2}\\.$"
            )
        )]
        fn succeeds_with_escape_option(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of_repetitions()
                .with_conversion_of_digits()
                .with_conversion_of_whitespace()
                .with_conversion_of_words()
                .with_escaping_of_non_ascii_chars(false)
                .build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
            assert_that_regexp_matches_test_cases(expected_output, test_cases);
        }

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and y̆y̆ and 💩💩."],
                "^\\w\\s{3}\\u{2665}{3}\\s\\d(?:\\d\\s\\w{3}\\s){2}\\w(?:\\w{3}\\s){2}(?:\\u{d83d}\\u{dca9}){2}\\.$"
            )
        )]
        fn succeeds_with_escape_and_surrogate_option(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of_repetitions()
                .with_conversion_of_digits()
                .with_conversion_of_whitespace()
                .with_conversion_of_words()
                .with_escaping_of_non_ascii_chars(true)
                .build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
        }
    }
}

mod non_digit_conversion {
    use super::*;

    mod no_repetition {
        use super::*;

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and y̆y̆ and 💩💩."],
                "^\\D\\D\\D\\D\\D\\D\\D\\D36\\D\\D\\D\\D\\D٣\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D$"
            )
        )]
        fn succeeds(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of_non_digits()
                .build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
            assert_that_regexp_matches_test_cases(expected_output, test_cases);
        }

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and y̆y̆ and 💩💩."],
                "^\\D\\D\\D\\D\\D\\D\\D\\D36\\D\\D\\D\\D\\D\\u{663}\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D$"
            )
        )]
        fn succeeds_with_escape_option(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of_non_digits()
                .with_escaping_of_non_ascii_chars(false)
                .build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
            assert_that_regexp_matches_test_cases(expected_output, test_cases);
        }
    }

    mod repetition {
        use super::*;

        #[rstest(test_cases, expected_output,
            case(vec!["I   ♥♥♥ 36 and ٣ and y̆y̆ and 💩💩."], "^\\D{8}36\\D{5}٣\\D{17}$")
        )]
        fn succeeds(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of_repetitions()
                .with_conversion_of_non_digits()
                .build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
            assert_that_regexp_matches_test_cases(expected_output, test_cases);
        }

        #[rstest(test_cases, expected_output,
            case(vec!["I   ♥♥♥ 36 and ٣ and y̆y̆ and 💩💩."], "^\\D{8}36\\D{5}\\u{663}\\D{17}$")
        )]
        fn succeeds_with_escape_option(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of_repetitions()
                .with_conversion_of_non_digits()
                .with_escaping_of_non_ascii_chars(false)
                .build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
            assert_that_regexp_matches_test_cases(expected_output, test_cases);
        }
    }
}

mod non_space_conversion {
    use super::*;

    mod no_repetition {
        use super::*;

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and y̆y̆ and 💩💩."],
                "^\\S   \\S\\S\\S \\S\\S \\S\\S\\S \\S \\S\\S\\S \\S\\S\\S\\S \\S\\S\\S \\S\\S\\S$"
            )
        )]
        fn succeeds(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of_non_whitespace()
                .build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
            assert_that_regexp_matches_test_cases(expected_output, test_cases);
        }
    }

    mod repetition {
        use super::*;

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and y̆y̆ and 💩💩."],
                "^\\S {3}\\S(?:\\S{2} ){2}\\S{3} (?:\\S(?: \\S{3}){2}){2}$"
            )
        )]
        fn succeeds(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of_repetitions()
                .with_conversion_of_non_whitespace()
                .build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
            assert_that_regexp_matches_test_cases(expected_output, test_cases);
        }
    }
}

mod non_word_conversion {
    use super::*;

    mod no_repetition {
        use super::*;

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and y̆y̆ and 💩💩."],
                "^I\\W\\W\\W\\W\\W\\W\\W36\\Wand\\W٣\\Wand\\Wy̆y̆\\Wand\\W\\W\\W\\W$"
            )
        )]
        fn succeeds(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of_non_words()
                .build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
            assert_that_regexp_matches_test_cases(expected_output, test_cases);
        }

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and y̆y̆ and 💩💩."],
                "^I\\W\\W\\W\\W\\W\\W\\W36\\Wand\\W\\u{663}\\Wand\\Wy\\u{306}y\\u{306}\\Wand\\W\\W\\W\\W$"
            )
        )]
        fn succeeds_with_escape_option(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of_non_words()
                .with_escaping_of_non_ascii_chars(false)
                .build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
            assert_that_regexp_matches_test_cases(expected_output, test_cases);
        }
    }

    mod repetition {
        use super::*;

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and y̆y̆ and 💩💩."],
                "^I\\W{7}36\\Wand\\W٣\\Wand\\W(?:y̆){2}\\Wand\\W{4}$"
            )
        )]
        fn succeeds(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of_repetitions()
                .with_conversion_of_non_words()
                .build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
            assert_that_regexp_matches_test_cases(expected_output, test_cases);
        }

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and y̆y̆ and 💩💩."],
                "^I\\W{7}36\\Wand\\W\\u{663}\\Wand\\W(?:y\\u{306}){2}\\Wand\\W{4}$"
            )
        )]
        fn succeeds_with_escape_option(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of_repetitions()
                .with_conversion_of_non_words()
                .with_escaping_of_non_ascii_chars(false)
                .build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
            assert_that_regexp_matches_test_cases(expected_output, test_cases);
        }
    }
}

mod non_digit_non_space_conversion {
    use super::*;

    mod no_repetition {
        use super::*;

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and y̆y̆ and 💩💩."],
                "^\\D\\D\\D\\D\\D\\D\\D\\D\\S\\S\\D\\D\\D\\D\\D\\S\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D$"
            )
        )]
        fn succeeds(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of_non_digits()
                .with_conversion_of_non_whitespace()
                .build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
            assert_that_regexp_matches_test_cases(expected_output, test_cases);
        }
    }

    mod repetition {
        use super::*;

        #[rstest(test_cases, expected_output,
            case(vec!["I   ♥♥♥ 36 and ٣ and y̆y̆ and 💩💩."], "^\\D{8}\\S{2}\\D{5}\\S\\D{17}$")
        )]
        fn succeeds(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of_repetitions()
                .with_conversion_of_non_digits()
                .with_conversion_of_non_whitespace()
                .build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
            assert_that_regexp_matches_test_cases(expected_output, test_cases);
        }
    }
}

mod non_digit_non_word_conversion {
    use super::*;

    mod no_repetition {
        use super::*;

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and y̆y̆ and 💩💩."],
                "^\\D\\D\\D\\D\\D\\D\\D\\D36\\D\\D\\D\\D\\D٣\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D$"
            )
        )]
        fn succeeds(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of_non_digits()
                .with_conversion_of_non_words()
                .build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
            assert_that_regexp_matches_test_cases(expected_output, test_cases);
        }
    }

    mod repetition {
        use super::*;

        #[rstest(test_cases, expected_output,
            case(vec!["I   ♥♥♥ 36 and ٣ and y̆y̆ and 💩💩."], "^\\D{8}36\\D{5}٣\\D{17}$")
        )]
        fn succeeds(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of_repetitions()
                .with_conversion_of_non_digits()
                .with_conversion_of_non_words()
                .build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
            assert_that_regexp_matches_test_cases(expected_output, test_cases);
        }
    }
}

mod non_space_non_word_conversion {
    use super::*;

    mod no_repetition {
        use super::*;

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and y̆y̆ and 💩💩."],
                "^\\S\\W\\W\\W\\W\\W\\W\\W\\S\\S\\W\\S\\S\\S\\W\\S\\W\\S\\S\\S\\W\\S\\S\\S\\S\\W\\S\\S\\S\\W\\W\\W\\W$"
            )
        )]
        fn succeeds(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of_non_whitespace()
                .with_conversion_of_non_words()
                .build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
            assert_that_regexp_matches_test_cases(expected_output, test_cases);
        }
    }

    mod repetition {
        use super::*;

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and y̆y̆ and 💩💩."],
                "^\\S\\W{7}\\S(?:\\S\\W\\S{3}\\W){2}\\S{4}\\W\\S{3}\\W{4}$"
            )
        )]
        fn succeeds(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of_repetitions()
                .with_conversion_of_non_whitespace()
                .with_conversion_of_non_words()
                .build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
            assert_that_regexp_matches_test_cases(expected_output, test_cases);
        }
    }
}

mod non_digit_non_space_non_word_conversion {
    use super::*;

    mod no_repetition {
        use super::*;

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and y̆y̆ and 💩💩."],
                "^\\D\\D\\D\\D\\D\\D\\D\\D\\S\\S\\D\\D\\D\\D\\D\\S\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D$"
            )
        )]
        fn succeeds(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of_non_digits()
                .with_conversion_of_non_whitespace()
                .with_conversion_of_non_words()
                .build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
            assert_that_regexp_matches_test_cases(expected_output, test_cases);
        }
    }

    mod repetition {
        use super::*;

        #[rstest(test_cases, expected_output,
            case(vec!["I   ♥♥♥ 36 and ٣ and y̆y̆ and 💩💩."], "^\\D{8}\\S{2}\\D{5}\\S\\D{17}$")
        )]
        fn succeeds(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of_repetitions()
                .with_conversion_of_non_digits()
                .with_conversion_of_non_whitespace()
                .with_conversion_of_non_words()
                .build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
            assert_that_regexp_matches_test_cases(expected_output, test_cases);
        }
    }
}

mod digit_non_digit_conversion {
    use super::*;

    mod no_repetition {
        use super::*;

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and y̆y̆ and 💩💩."],
                "^\\D\\D\\D\\D\\D\\D\\D\\D\\d\\d\\D\\D\\D\\D\\D\\d\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D\\D$"
            )
        )]
        fn succeeds(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of_digits()
                .with_conversion_of_non_digits()
                .build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
            assert_that_regexp_matches_test_cases(expected_output, test_cases);
        }
    }

    mod repetition {
        use super::*;

        #[rstest(test_cases, expected_output,
            case(vec!["I   ♥♥♥ 36 and ٣ and y̆y̆ and 💩💩."], "^\\D{8}\\d{2}\\D{5}\\d\\D{17}$")
        )]
        fn succeeds(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of_repetitions()
                .with_conversion_of_digits()
                .with_conversion_of_non_digits()
                .build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
            assert_that_regexp_matches_test_cases(expected_output, test_cases);
        }
    }
}

mod space_non_space_conversion {
    use super::*;

    mod no_repetition {
        use super::*;

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and y̆y̆ and 💩💩."],
                "^\\S\\s\\s\\s\\S\\S\\S\\s\\S\\S\\s\\S\\S\\S\\s\\S\\s\\S\\S\\S\\s\\S\\S\\S\\S\\s\\S\\S\\S\\s\\S\\S\\S$"
            )
        )]
        fn succeeds(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of_whitespace()
                .with_conversion_of_non_whitespace()
                .build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
            assert_that_regexp_matches_test_cases(expected_output, test_cases);
        }
    }

    mod repetition {
        use super::*;

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and y̆y̆ and 💩💩."],
                "^\\S\\s{3}\\S(?:\\S{2}\\s){2}\\S{3}\\s(?:\\S(?:\\s\\S{3}){2}){2}$"
            )
        )]
        fn succeeds(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of_repetitions()
                .with_conversion_of_whitespace()
                .with_conversion_of_non_whitespace()
                .build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
            assert_that_regexp_matches_test_cases(expected_output, test_cases);
        }
    }
}

mod word_non_word_conversion {
    use super::*;

    mod no_repetition {
        use super::*;

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and y̆y̆ and 💩💩."],
                "^\\w\\W\\W\\W\\W\\W\\W\\W\\w\\w\\W\\w\\w\\w\\W\\w\\W\\w\\w\\w\\W\\w\\w\\w\\w\\W\\w\\w\\w\\W\\W\\W\\W$"
            )
        )]
        fn succeeds(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of_words()
                .with_conversion_of_non_words()
                .build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
            assert_that_regexp_matches_test_cases(expected_output, test_cases);
        }
    }

    mod repetition {
        use super::*;

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and y̆y̆ and 💩💩."],
                "^\\w\\W{7}\\w(?:\\w\\W\\w{3}\\W){2}\\w{4}\\W\\w{3}\\W{4}$"
            )
        )]
        fn succeeds(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of_repetitions()
                .with_conversion_of_words()
                .with_conversion_of_non_words()
                .build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
            assert_that_regexp_matches_test_cases(expected_output, test_cases);
        }
    }
}

mod anchor_conversion {
    use super::*;

    mod no_verbose {
        use super::*;

        #[rstest(test_cases, expected_output,
            case(vec!["aaacaac", "aac"], "aa(?:acaa)?c$"),
            case(vec!["My ♥♥♥ and 💩💩 is yours."], "My ♥♥♥ and 💩💩 is yours\\.$"),
        )]
        fn succeeds_without_start_anchor_option(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .without_start_anchor()
                .build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
            assert_that_regexp_matches_test_cases(expected_output, test_cases);
        }

        #[rstest(test_cases, expected_output,
            case(vec!["aaacaac", "aac"], "^aa(?:acaa)?c"),
            case(vec!["My ♥♥♥ and 💩💩 is yours."], "^My ♥♥♥ and 💩💩 is yours\\."),
        )]
        fn succeeds_without_end_anchor_option(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .without_end_anchor()
                .build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
            assert_that_regexp_matches_test_cases(expected_output, test_cases);
        }

        #[rstest(test_cases, expected_output,
            case(vec!["bab", "b", "cb", "bba"], "(?:b(?:ba|ab)?|cb)"),
            case(vec!["a", "aba", "baaa", "aaab"], "(?:baaa|a(?:aab|ba)?)"),
            case(vec!["a", "abab", "bbb", "aaac"], "(?:a(?:bab|aac)?|bbb)"),
            case(vec!["aaacaac", "aac"], "aa(?:acaa)?c"),
            case(vec!["My ♥♥♥ and 💩💩 is yours."], "My ♥♥♥ and 💩💩 is yours\\."),
            case(
                // https://github.com/pemistahl/grex/issues/31
                vec!["agbhd", "eibcd", "egbcd", "fbjbf", "agbh", "eibc", "egbc", "ebc", "fbc", "cd", "f", "c", "abcd", "ebcd", "fbcd"],
                "(?:a(?:gbhd?|bcd)|e(?:ibcd?|gbcd?|bcd?)|f(?:b(?:jbf|cd?))?|cd?)"
            )
        )]
        fn succeeds_without_anchors(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases).without_anchors().build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
            assert_that_regexp_matches_test_cases(expected_output, test_cases);
        }
    }

    mod verbose {
        use super::*;

        #[rstest(test_cases, expected_output,
            case(vec!["My ♥♥♥ and 💩💩 is yours."], indoc!(
                r#"
                (?x)
                  My\ ♥♥♥\ and\ 💩💩\ is\ yours\.
                $"#
            ))
        )]
        fn succeeds_with_verbose_mode_and_without_start_anchor_option(
            test_cases: Vec<&str>,
            expected_output: &str,
        ) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_verbose_mode()
                .without_start_anchor()
                .build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
            assert_that_regexp_matches_test_cases(expected_output, test_cases);
        }

        #[rstest(test_cases, expected_output,
            case(vec!["My ♥♥♥ and 💩💩 is yours."], indoc!(
                r#"
                (?x)
                ^
                  My\ ♥♥♥\ and\ 💩💩\ is\ yours\."#
            ))
        )]
        fn succeeds_with_verbose_mode_and_without_end_anchor_option(
            test_cases: Vec<&str>,
            expected_output: &str,
        ) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_verbose_mode()
                .without_end_anchor()
                .build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
            assert_that_regexp_matches_test_cases(expected_output, test_cases);
        }

        #[rstest(test_cases, expected_output,
            case(vec!["aaacaac", "aac"], indoc!(
                r#"
                (?x)
                  aa
                  (?:
                    acaa
                  )?
                  c"#
            )),
            case(vec!["My ♥♥♥ and 💩💩 is yours."], indoc!(
                r#"
                (?x)
                  My\ ♥♥♥\ and\ 💩💩\ is\ yours\."#
            ))
        )]
        fn succeeds_with_verbose_mode_and_without_anchors_option(
            test_cases: Vec<&str>,
            expected_output: &str,
        ) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_verbose_mode()
                .without_anchors()
                .build();
            assert_that_regexp_is_correct(regexp, expected_output, &test_cases);
            assert_that_regexp_matches_test_cases(expected_output, test_cases);
        }
    }
}

fn assert_that_regexp_is_correct(regexp: String, expected_output: &str, test_cases: &[&str]) {
    assert_eq!(
        regexp, expected_output,
        "\n\ninput: {:?}\nexpected: {}\nactual: {}\n\n",
        test_cases, expected_output, regexp
    );
}

fn assert_that_regexp_matches_test_cases(expected_output: &str, test_cases: Vec<&str>) {
    let regexp = Regex::new(expected_output).unwrap();
    for test_case in test_cases {
        let substrings = regexp
            .find_iter(test_case)
            .map(|m| m.as_str())
            .collect::<Vec<_>>();

        assert_eq!(
            substrings.len(),
            1,
            "expression '{}' does not match test case '{}' entirely but {} of its substrings: {:?}",
            expected_output,
            test_case,
            substrings.len(),
            substrings
        );
    }
}
