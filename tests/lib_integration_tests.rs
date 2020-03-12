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
            case(vec!["a", "b", "x", "de"], "^(de|[abx])$"),
            case(vec!["a", "b", "c", "x", "d", "e"], "^[a-ex]$"),
            case(vec!["a", "b", "c", "x", "de"], "^(de|[a-cx])$"),
            case(vec!["a", "b", "c", "d", "e", "f", "o", "x", "y", "z"], "^[a-fox-z]$"),
            case(vec!["a", "b", "d", "e", "f", "o", "x", "y", "z"], "^[abd-fox-z]$"),
            case(vec!["1", "2"], "^[12]$"),
            case(vec!["1", "2", "3"], "^[1-3]$"),
            case(vec!["1", "3", "4", "5", "6"], "^[13-6]$"),
            case(vec!["1", "2", "8", "4", "5"], "^[12458]$"),
            case(vec!["1", "2", "8", "45"], "^(45|[128])$"),
            case(vec!["1", "2", "3", "8", "4", "5"], "^[1-58]$"),
            case(vec!["1", "2", "3", "8", "45"], "^(45|[1-38])$"),
            case(vec!["1", "2", "3", "5", "7", "8", "9"], "^[1-357-9]$"),
            case(vec!["a", "b", "bc"], "^(bc?|a)$"),
            case(vec!["a", "b", "bcd"], "^(b(cd)?|a)$"),
            case(vec!["a", "ab", "abc"], "^a(bc?)?$"),
            case(vec!["ac", "bc"], "^[ab]c$"),
            case(vec!["ab", "ac"], "^a[bc]$"),
            case(vec!["abx", "cdx"], "^(ab|cd)x$"),
            case(vec!["abd", "acd"], "^a[bc]d$"),
            case(vec!["abc", "abcd"], "^abcd?$"),
            case(vec!["abc", "abcde"], "^abc(de)?$"),
            case(vec!["ade", "abcde"], "^a(bc)?de$"),
            case(vec!["abcxy", "adexy"], "^a(bc|de)xy$"),
            case(vec!["axy", "abcxy", "adexy"], "^a((bc)?|de)xy$"), // goal: "^a(bc|de)?xy$"
            case(vec!["abcxy", "abcw", "efgh"], "^(abc(xy|w)|efgh)$"),
            case(vec!["abcxy", "efgh", "abcw"], "^(abc(xy|w)|efgh)$"),
            case(vec!["efgh", "abcxy", "abcw"], "^(abc(xy|w)|efgh)$"),
            case(vec!["abxy", "cxy", "efgh"], "^((ab|c)xy|efgh)$"),
            case(vec!["abxy", "efgh", "cxy"], "^((ab|c)xy|efgh)$"),
            case(vec!["efgh", "abxy", "cxy"], "^((ab|c)xy|efgh)$"),
            case(vec!["a", "ä", "o", "ö", "u", "ü"], "^[aouäöü]$"),
            case(vec!["y̆", "a", "z"], "^(y̆|[az])$"), // goal: "^[az]|y\\u{306}$"
            case(vec!["a", "b\n", "c"], "^(b\\n|[ac])$"),
            case(vec!["a", "b\\n", "c"], "^(b\\\\n|[ac])$"),
            case(vec!["[a-z]", "(d,e,f)"], "^(\\(d,e,f\\)|\\[a\\-z\\])$"),
            case(vec!["3.5", "4.5", "4,5"], "^(3\\.5|4[,.]5)$"),
            case(vec!["I ♥ cake"], "^I ♥ cake$"),
            case(vec!["I \u{2665} cake"], "^I ♥ cake$"),
            case(vec!["I \\u{2665} cake"], "^I \\\\u\\{2665\\} cake$"),
            case(vec!["I \\u2665 cake"], "^I \\\\u2665 cake$"),
            case(vec!["My ♥ is yours.", "My 💩 is yours."], "^My [♥💩] is yours\\.$"),
            case(vec!["[\u{c3e}"], "^\\[\u{c3e}$"),
            case(vec!["\\\u{10376}"], "^\\\\\u{10376}$"),
            case(vec!["I   ♥♥♥ 36 and ٣ and 💩💩."], "^I   ♥♥♥ 36 and ٣ and 💩💩\\.$")
        )]
        fn succeeds(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases).build();
            test_if_regexp_is_correct(regexp, expected_output, &test_cases);
            test_if_regexp_matches_test_cases(expected_output, test_cases);
        }

        #[rstest(test_cases, expected_output,
            case(vec!["My ♥ and 💩 is yours."], "^My \\u{2665} and \\u{1f4a9} is yours\\.$"),
            case(vec!["My ♥ is yours.", "My 💩 is yours."], "^My (\\u{2665}|\\u{1f4a9}) is yours\\.$"),
            case(vec!["I   ♥♥♥ 36 and ٣ and 💩💩."], "^I   \\u{2665}\\u{2665}\\u{2665} 36 and \\u{663} and \\u{1f4a9}\\u{1f4a9}\\.$")
        )]
        fn succeeds_with_escape_option(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_escaping_of_non_ascii_chars(false)
                .build();
            test_if_regexp_is_correct(regexp, expected_output, &test_cases);
            test_if_regexp_matches_test_cases(expected_output, test_cases);
        }

        #[rstest(test_cases, expected_output,
            case(vec!["My ♥ and 💩 is yours."], "^My \\u{2665} and \\u{d83d}\\u{dca9} is yours\\.$"),
            case(vec!["My ♥ is yours.", "My 💩 is yours."], "^My (\\u{2665}|\\u{d83d}\\u{dca9}) is yours\\.$"),
            case(vec!["I   ♥♥♥ 36 and ٣ and 💩💩."], "^I   \\u{2665}\\u{2665}\\u{2665} 36 and \\u{663} and \\u{d83d}\\u{dca9}\\u{d83d}\\u{dca9}\\.$")
        )]
        fn succeeds_with_escape_and_surrogate_option(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_escaping_of_non_ascii_chars(true)
                .build();
            test_if_regexp_is_correct(regexp, expected_output, &test_cases);
        }

        #[test]
        #[allow(unused_must_use)]
        fn succeeds_with_file_input() {
            let mut file = NamedTempFile::new().unwrap();
            writeln!(file, "a\nb\nc\r\nxyz");

            let expected_output = "^(xyz|[a-c])$";
            let test_cases = vec!["a", "b", "c", "xyz"];

            let regexp = RegExpBuilder::from_file(file.path()).build();
            test_if_regexp_is_correct(regexp, expected_output, &test_cases);
            test_if_regexp_matches_test_cases(expected_output, test_cases);
        }

        #[test]
        #[should_panic(expected = "The specified file could not be found")]
        fn fails_when_file_does_not_exist() {
            RegExpBuilder::from_file("/path/to/non-existing/file");
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
            case(vec!["a", "aa"], "^a{1,2}$"),
            case(vec!["aaa", "a", "aa"], "^a{1,3}$"),
            case(vec!["aaaa", "a", "aa"], "^(a{1,2}|a{4})$"),
            case(vec!["a", "aa", "aaa", "aaaa", "aaab"], "^(a{3}b|a{1,4})$"),
            case(vec!["baabaaaaaabb"], "^ba{2}ba{6}b{2}$"),
            case(vec!["aabbaabbaaa"], "^(a{2}b{2}){2}a{3}$"),
            case(vec!["aabbaa"], "^a{2}b{2}a{2}$"),
            case(vec!["aabbabb"], "^a(ab{2}){2}$"),
            case(vec!["ababab"], "^(ab){3}$"),
            case(vec!["abababa"], "^a(ba){3}$"),
            case(vec!["aababab"], "^a(ab){3}$"),
            case(vec!["abababaa"], "^(ab){3}a{2}$"),
            case(vec!["aaaaaabbbbb"], "^a{6}b{5}$"),
            case(vec!["aabaababab"], "^(a{2}b){2}abab$"), // goal: ^(a{2}b){2}(ab)2$
            case(vec!["aaaaaaabbbbbba"], "^a{7}b{6}a$"),
            case(vec!["abaaaabaaba"], "^abaa(a{2}b){2}a$"),
            case(vec!["bbaababb"], "^b{2}a{2}bab{2}$"),
            case(vec!["b", "ba"], "^ba?$"),
            case(vec!["b", "ba", "baa"], "^b(a{1,2})?$"),
            case(vec!["b", "ba", "baaa", "baa"], "^b(a{1,3})?$"),
            case(vec!["b", "ba", "baaaa", "baa"], "^b(a{1,2}|a{4})?$"),
            case(vec!["axy", "abcxyxy", "adexy"], "^a((de)?xy|bc(xy){2})$"),
            case(vec!["xy̆y̆y̆y̆z"], "^x(y̆){4}z$"),
            case(vec!["xy̆y̆z", "xy̆y̆y̆z"], "^x(y̆){2,3}z$"),
            case(vec!["xy̆y̆z", "xy̆y̆y̆y̆z"], "^x((y̆){2}|(y̆){4})z$"),
            case(vec!["zyxx", "yxx"], "^z?yx{2}$"),
            case(vec!["zyxx", "yxx", "yxxx"], "^(zyx{2}|yx{2,3})$"),
            case(vec!["zyxxx", "yxx", "yxxx"], "^(zyx{3}|yx{2,3})$"),
            case(vec!["a", "b\n\n", "c"], "^(b\\n{2}|[ac])$"),
            case(vec!["a", "b\nb\nb", "c"], "^(b(\\nb){2}|[ac])$"),
            case(vec!["a", "b\nx\nx", "c"], "^(b(\\nx){2}|[ac])$"),
            case(vec!["a", "b\n\t\n\t", "c"], "^(b(\\n\\t){2}|[ac])$"),
            case(vec!["a", "b\n", "b\n\n", "b\n\n\n", "c"], "^(b\\n{1,3}|[ac])$"),
            case(vec!["4.5", "3.55"], "^(4\\.5|3\\.5{2})$"),
            case(vec!["4.5", "4.55"], "^4\\.5{1,2}$"),
            case(vec!["4.5", "4.55", "3.5"], "^(3\\.5|4\\.5{1,2})$"),
            case(vec!["4.5", "44.5", "44.55", "4.55"], "^4{1,2}\\.5{1,2}$"),
            case(vec!["I ♥♥ cake"], "^I ♥{2} cake$"),
            case(vec!["I ♥ cake", "I ♥♥ cake"], "^I ♥{1,2} cake$"),
            case(vec!["I \u{2665}\u{2665} cake"], "^I ♥{2} cake$"),
            case(vec!["I \\u{2665} cake"], "^I \\\\u\\{26{2}5\\} cake$"),
            case(vec!["I \\u{2665}\\u{2665} cake"], "^I (\\\\u\\{26{2}5\\}){2} cake$"),
            case(vec!["I \\u2665\\u2665 cake"], "^I (\\\\u26{2}5){2} cake$"),
            case(vec!["My ♥♥♥ is yours.", "My 💩💩 is yours."], "^My (💩{2}|♥{3}) is yours\\.$"),
            case(vec!["I   ♥♥♥ 36 and ٣ and 💩💩."], "^I {3}♥{3} 36 and ٣ and 💩{2}\\.$")
        )]
        fn succeeds(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of(&[Feature::Repetition])
                .build();
            test_if_regexp_is_correct(regexp, expected_output, &test_cases);
            test_if_regexp_matches_test_cases(expected_output, test_cases);
        }

        #[rstest(test_cases, expected_output,
            case(vec!["My ♥♥♥ and 💩💩 is yours."], "^My \\u{2665}{3} and \\u{1f4a9}{2} is yours\\.$"),
            case(vec!["My ♥♥♥ is yours.", "My 💩💩 is yours."], "^My (\\u{1f4a9}{2}|\\u{2665}{3}) is yours\\.$"),
            case(vec!["I   ♥♥♥ 36 and ٣ and 💩💩."], "^I {3}\\u{2665}{3} 36 and \\u{663} and \\u{1f4a9}{2}\\.$")
        )]
        fn succeeds_with_escape_option(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of(&[Feature::Repetition])
                .with_escaping_of_non_ascii_chars(false)
                .build();
            test_if_regexp_is_correct(regexp, expected_output, &test_cases);
            test_if_regexp_matches_test_cases(expected_output, test_cases);
        }

        #[rstest(test_cases, expected_output,
            case(vec!["My ♥♥♥ and 💩💩 is yours."], "^My \\u{2665}{3} and (\\u{d83d}\\u{dca9}){2} is yours\\.$"),
            case(vec!["My ♥♥♥ is yours.", "My 💩💩 is yours."], "^My ((\\u{d83d}\\u{dca9}){2}|\\u{2665}{3}) is yours\\.$"),
            case(vec!["I   ♥♥♥ 36 and ٣ and 💩💩."], "^I {3}\\u{2665}{3} 36 and \\u{663} and (\\u{d83d}\\u{dca9}){2}\\.$")
        )]
        fn succeeds_with_escape_and_surrogate_option(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of(&[Feature::Repetition])
                .with_escaping_of_non_ascii_chars(true)
                .build();
            test_if_regexp_is_correct(regexp, expected_output, &test_cases);
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
            case(vec!["1", "23"], "^\\d(\\d)?$"),
            case(vec!["1", "234"], "^\\d(\\d\\d)?$"),
            case(vec!["8", "234"], "^\\d(\\d\\d)?$"),
            case(vec!["890", "34"], "^\\d\\d(\\d)?$"),
            case(vec!["abc123"], "^abc\\d\\d\\d$"),
            case(vec!["a1b2c3"], "^a\\db\\dc\\d$"),
            case(vec!["abc", "123"], "^(\\d\\d\\d|abc)$"),
            case(vec!["١", "٣", "٥"], "^\\d$"), // Arabic digits: ١ = 1, ٣ = 3, ٥ = 5
            case(vec!["١٣٥"], "^\\d\\d\\d$"),
            case(vec!["a٣3", "b5٥"], "^[ab]\\d\\d$"),
            case(vec!["I ♥ 123"], "^I ♥ \\d\\d\\d$"),
            case(vec!["I   ♥♥♥ 36 and ٣ and 💩💩."], "^I   ♥♥♥ \\d\\d and \\d and 💩💩\\.$")
        )]
        fn succeeds(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of(&[Feature::Digit])
                .build();
            test_if_regexp_is_correct(regexp, expected_output, &test_cases);
            test_if_regexp_matches_test_cases(expected_output, test_cases);
        }

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and 💩💩."],
                "^I   \\u{2665}\\u{2665}\\u{2665} \\d\\d and \\d and \\u{1f4a9}\\u{1f4a9}\\.$"
            )
        )]
        fn succeeds_with_escape_option(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of(&[Feature::Digit])
                .with_escaping_of_non_ascii_chars(false)
                .build();
            test_if_regexp_is_correct(regexp, expected_output, &test_cases);
            test_if_regexp_matches_test_cases(expected_output, test_cases);
        }

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and 💩💩."],
                "^I   \\u{2665}\\u{2665}\\u{2665} \\d\\d and \\d and \\u{d83d}\\u{dca9}\\u{d83d}\\u{dca9}\\.$"
            )
        )]
        fn succeeds_with_escape_and_surrogate_option(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of(&[Feature::Digit])
                .with_escaping_of_non_ascii_chars(true)
                .build();
            test_if_regexp_is_correct(regexp, expected_output, &test_cases);
        }
    }

    mod repetition {
        use super::*;

        #[rstest(test_cases, expected_output,
            case(vec!["I   ♥♥♥ 36 and ٣ and 💩💩."], "^I {3}♥{3} \\d(\\d and ){2}💩{2}\\.$")
        )]
        fn succeeds(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of(&[Feature::Repetition, Feature::Digit])
                .build();
            test_if_regexp_is_correct(regexp, expected_output, &test_cases);
            test_if_regexp_matches_test_cases(expected_output, test_cases);
        }

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and 💩💩."],
                "^I {3}\\u{2665}{3} \\d(\\d and ){2}\\u{1f4a9}{2}\\.$"
            )
        )]
        fn succeeds_with_escape_option(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of(&[Feature::Repetition, Feature::Digit])
                .with_escaping_of_non_ascii_chars(false)
                .build();
            test_if_regexp_is_correct(regexp, expected_output, &test_cases);
            test_if_regexp_matches_test_cases(expected_output, test_cases);
        }

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and 💩💩."],
                "^I {3}\\u{2665}{3} \\d(\\d and ){2}(\\u{d83d}\\u{dca9}){2}\\.$"
            )
        )]
        fn succeeds_with_escape_and_surrogate_option(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of(&[Feature::Repetition, Feature::Digit])
                .with_escaping_of_non_ascii_chars(true)
                .build();
            test_if_regexp_is_correct(regexp, expected_output, &test_cases);
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
            case(vec!["\n\t", "\r"], "^\\s(\\s)?$"),
            case(vec!["a"], "^a$"),
            case(vec!["1"], "^1$"),
            case(vec!["I ♥ 123"], "^I\\s♥\\s123$"),
            case(vec!["I   ♥♥♥ 36 and ٣ and 💩💩."], "^I\\s\\s\\s♥♥♥\\s36\\sand\\s٣\\sand\\s💩💩\\.$")
        )]
        fn succeeds(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of(&[Feature::Space])
                .build();
            test_if_regexp_is_correct(regexp, expected_output, &test_cases);
            test_if_regexp_matches_test_cases(expected_output, test_cases);
        }

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and 💩💩."],
                "^I\\s\\s\\s\\u{2665}\\u{2665}\\u{2665}\\s36\\sand\\s\\u{663}\\sand\\s\\u{1f4a9}\\u{1f4a9}\\.$"
            )
        )]
        fn succeeds_with_escape_option(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of(&[Feature::Space])
                .with_escaping_of_non_ascii_chars(false)
                .build();
            test_if_regexp_is_correct(regexp, expected_output, &test_cases);
            test_if_regexp_matches_test_cases(expected_output, test_cases);
        }

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and 💩💩."],
                "^I\\s\\s\\s\\u{2665}\\u{2665}\\u{2665}\\s36\\sand\\s\\u{663}\\sand\\s\\u{d83d}\\u{dca9}\\u{d83d}\\u{dca9}\\.$"
            )
        )]
        fn succeeds_with_escape_and_surrogate_option(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of(&[Feature::Space])
                .with_escaping_of_non_ascii_chars(true)
                .build();
            test_if_regexp_is_correct(regexp, expected_output, &test_cases);
        }
    }

    mod repetition {
        use super::*;

        #[rstest(test_cases, expected_output,
            case(vec!["I   ♥♥♥ 36 and ٣ and 💩💩."], "^I\\s{3}♥{3}\\s36\\sand\\s٣\\sand\\s💩{2}\\.$")
        )]
        fn succeeds(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of(&[Feature::Repetition, Feature::Space])
                .build();
            test_if_regexp_is_correct(regexp, expected_output, &test_cases);
            test_if_regexp_matches_test_cases(expected_output, test_cases);
        }

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and 💩💩."],
                "^I\\s{3}\\u{2665}{3}\\s36\\sand\\s\\u{663}\\sand\\s\\u{1f4a9}{2}\\.$"
            )
        )]
        fn succeeds_with_escape_option(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of(&[Feature::Repetition, Feature::Space])
                .with_escaping_of_non_ascii_chars(false)
                .build();
            test_if_regexp_is_correct(regexp, expected_output, &test_cases);
            test_if_regexp_matches_test_cases(expected_output, test_cases);
        }

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and 💩💩."],
                "^I\\s{3}\\u{2665}{3}\\s36\\sand\\s\\u{663}\\sand\\s(\\u{d83d}\\u{dca9}){2}\\.$"
            )
        )]
        fn succeeds_with_escape_and_surrogate_option(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of(&[Feature::Repetition, Feature::Space])
                .with_escaping_of_non_ascii_chars(true)
                .build();
            test_if_regexp_is_correct(regexp, expected_output, &test_cases);
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
            case(vec!["abc", "1234"], "^\\w\\w\\w(\\w)?$"),
            case(vec!["١", "٣", "٥"], "^\\w$"), // Arabic digits: ١ = 1, ٣ = 3, ٥ = 5
            case(vec!["١٣٥"], "^\\w\\w\\w$"),
            case(vec!["a٣3", "b5٥"], "^\\w\\w\\w$"),
            case(vec!["I ♥ 123"], "^\\w ♥ \\w\\w\\w$"),
            case(vec!["I   ♥♥♥ 36 and ٣ and 💩💩."], "^\\w   ♥♥♥ \\w\\w \\w\\w\\w \\w \\w\\w\\w 💩💩\\.$")
        )]
        fn succeeds(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of(&[Feature::Word])
                .build();
            test_if_regexp_is_correct(regexp, expected_output, &test_cases);
            test_if_regexp_matches_test_cases(expected_output, test_cases);
        }

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and 💩💩."],
                "^\\w   \\u{2665}\\u{2665}\\u{2665} \\w\\w \\w\\w\\w \\w \\w\\w\\w \\u{1f4a9}\\u{1f4a9}\\.$"
            )
        )]
        fn succeeds_with_escape_option(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of(&[Feature::Word])
                .with_escaping_of_non_ascii_chars(false)
                .build();
            test_if_regexp_is_correct(regexp, expected_output, &test_cases);
            test_if_regexp_matches_test_cases(expected_output, test_cases);
        }

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and 💩💩."],
                "^\\w   \\u{2665}\\u{2665}\\u{2665} \\w\\w \\w\\w\\w \\w \\w\\w\\w \\u{d83d}\\u{dca9}\\u{d83d}\\u{dca9}\\.$"
            )
        )]
        fn succeeds_with_escape_and_surrogate_option(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of(&[Feature::Word])
                .with_escaping_of_non_ascii_chars(true)
                .build();
            test_if_regexp_is_correct(regexp, expected_output, &test_cases);
        }
    }

    mod repetition {
        use super::*;

        #[rstest(test_cases, expected_output,
            case(vec!["I   ♥♥♥ 36 and ٣ and 💩💩."], "^\\w {3}♥{3} \\w(\\w \\w{3} ){2}💩{2}\\.$")
        )]
        fn succeeds(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of(&[Feature::Repetition, Feature::Word])
                .build();
            test_if_regexp_is_correct(regexp, expected_output, &test_cases);
            test_if_regexp_matches_test_cases(expected_output, test_cases);
        }

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and 💩💩."],
                "^\\w {3}\\u{2665}{3} \\w(\\w \\w{3} ){2}\\u{1f4a9}{2}\\.$"
            )
        )]
        fn succeeds_with_escape_option(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of(&[Feature::Repetition, Feature::Word])
                .with_escaping_of_non_ascii_chars(false)
                .build();
            test_if_regexp_is_correct(regexp, expected_output, &test_cases);
            test_if_regexp_matches_test_cases(expected_output, test_cases);
        }

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and 💩💩."],
                "^\\w {3}\\u{2665}{3} \\w(\\w \\w{3} ){2}(\\u{d83d}\\u{dca9}){2}\\.$"
            )
        )]
        fn succeeds_with_escape_and_surrogate_option(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of(&[Feature::Repetition, Feature::Word])
                .with_escaping_of_non_ascii_chars(true)
                .build();
            test_if_regexp_is_correct(regexp, expected_output, &test_cases);
        }
    }
}

mod digit_space_conversion {
    use super::*;

    mod no_repetition {
        use super::*;

        #[rstest(test_cases, expected_output,
            case(vec!["I   ♥♥♥ 36 and ٣ and 💩💩."], "^I\\s\\s\\s♥♥♥\\s\\d\\d\\sand\\s\\d\\sand\\s💩💩\\.$")
        )]
        fn succeeds(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of(&[Feature::Digit, Feature::Space])
                .build();
            test_if_regexp_is_correct(regexp, expected_output, &test_cases);
            test_if_regexp_matches_test_cases(expected_output, test_cases);
        }

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and 💩💩."],
                "^I\\s\\s\\s\\u{2665}\\u{2665}\\u{2665}\\s\\d\\d\\sand\\s\\d\\sand\\s\\u{1f4a9}\\u{1f4a9}\\.$"
            )
        )]
        fn succeeds_with_escape_option(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of(&[Feature::Digit, Feature::Space])
                .with_escaping_of_non_ascii_chars(false)
                .build();
            test_if_regexp_is_correct(regexp, expected_output, &test_cases);
            test_if_regexp_matches_test_cases(expected_output, test_cases);
        }

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and 💩💩."],
                "^I\\s\\s\\s\\u{2665}\\u{2665}\\u{2665}\\s\\d\\d\\sand\\s\\d\\sand\\s\\u{d83d}\\u{dca9}\\u{d83d}\\u{dca9}\\.$"
            )
        )]
        fn succeeds_with_escape_and_surrogate_option(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of(&[Feature::Digit, Feature::Space])
                .with_escaping_of_non_ascii_chars(true)
                .build();
            test_if_regexp_is_correct(regexp, expected_output, &test_cases);
        }
    }

    mod repetition {
        use super::*;

        #[rstest(test_cases, expected_output,
            case(vec!["I   ♥♥♥ 36 and ٣ and 💩💩."], "^I\\s{3}♥{3}\\s\\d(\\d\\sand\\s){2}💩{2}\\.$")
        )]
        fn succeeds(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of(&[Feature::Repetition, Feature::Digit, Feature::Space])
                .build();
            test_if_regexp_is_correct(regexp, expected_output, &test_cases);
            test_if_regexp_matches_test_cases(expected_output, test_cases);
        }

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and 💩💩."],
                "^I\\s{3}\\u{2665}{3}\\s\\d(\\d\\sand\\s){2}\\u{1f4a9}{2}\\.$"
            )
        )]
        fn succeeds_with_escape_option(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of(&[Feature::Repetition, Feature::Digit, Feature::Space])
                .with_escaping_of_non_ascii_chars(false)
                .build();
            test_if_regexp_is_correct(regexp, expected_output, &test_cases);
            test_if_regexp_matches_test_cases(expected_output, test_cases);
        }

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and 💩💩."],
                "^I\\s{3}\\u{2665}{3}\\s\\d(\\d\\sand\\s){2}(\\u{d83d}\\u{dca9}){2}\\.$"
            )
        )]
        fn succeeds_with_escape_and_surrogate_option(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of(&[Feature::Repetition, Feature::Digit, Feature::Space])
                .with_escaping_of_non_ascii_chars(true)
                .build();
            test_if_regexp_is_correct(regexp, expected_output, &test_cases);
        }
    }
}

mod digit_word_conversion {
    use super::*;

    mod no_repetition {
        use super::*;

        #[rstest(test_cases, expected_output,
            case(vec!["I   ♥♥♥ 36 and ٣ and 💩💩."], "^\\w   ♥♥♥ \\d\\d \\w\\w\\w \\d \\w\\w\\w 💩💩\\.$")
        )]
        fn succeeds(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of(&[Feature::Digit, Feature::Word])
                .build();
            test_if_regexp_is_correct(regexp, expected_output, &test_cases);
            test_if_regexp_matches_test_cases(expected_output, test_cases);
        }

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and 💩💩."],
                "^\\w   \\u{2665}\\u{2665}\\u{2665} \\d\\d \\w\\w\\w \\d \\w\\w\\w \\u{1f4a9}\\u{1f4a9}\\.$"
            )
        )]
        fn succeeds_with_escape_option(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of(&[Feature::Digit, Feature::Word])
                .with_escaping_of_non_ascii_chars(false)
                .build();
            test_if_regexp_is_correct(regexp, expected_output, &test_cases);
            test_if_regexp_matches_test_cases(expected_output, test_cases);
        }

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and 💩💩."],
                "^\\w   \\u{2665}\\u{2665}\\u{2665} \\d\\d \\w\\w\\w \\d \\w\\w\\w \\u{d83d}\\u{dca9}\\u{d83d}\\u{dca9}\\.$"
            )
        )]
        fn succeeds_with_escape_and_surrogate_option(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of(&[Feature::Digit, Feature::Word])
                .with_escaping_of_non_ascii_chars(true)
                .build();
            test_if_regexp_is_correct(regexp, expected_output, &test_cases);
        }
    }

    mod repetition {
        use super::*;

        #[rstest(test_cases, expected_output,
            case(vec!["I   ♥♥♥ 36 and ٣ and 💩💩."], "^\\w {3}♥{3} \\d(\\d \\w{3} ){2}💩{2}\\.$")
        )]
        fn succeeds(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of(&[Feature::Repetition, Feature::Digit, Feature::Word])
                .build();
            test_if_regexp_is_correct(regexp, expected_output, &test_cases);
            test_if_regexp_matches_test_cases(expected_output, test_cases);
        }

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and 💩💩."],
                "^\\w {3}\\u{2665}{3} \\d(\\d \\w{3} ){2}\\u{1f4a9}{2}\\.$"
            )
        )]
        fn succeeds_with_escape_option(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of(&[Feature::Repetition, Feature::Digit, Feature::Word])
                .with_escaping_of_non_ascii_chars(false)
                .build();
            test_if_regexp_is_correct(regexp, expected_output, &test_cases);
            test_if_regexp_matches_test_cases(expected_output, test_cases);
        }

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and 💩💩."],
                "^\\w {3}\\u{2665}{3} \\d(\\d \\w{3} ){2}(\\u{d83d}\\u{dca9}){2}\\.$"
            )
        )]
        fn succeeds_with_escape_and_surrogate_option(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of(&[Feature::Repetition, Feature::Digit, Feature::Word])
                .with_escaping_of_non_ascii_chars(true)
                .build();
            test_if_regexp_is_correct(regexp, expected_output, &test_cases);
        }
    }
}

mod space_word_conversion {
    use super::*;

    mod no_repetition {
        use super::*;

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and 💩💩."],
                "^\\w\\s\\s\\s♥♥♥\\s\\w\\w\\s\\w\\w\\w\\s\\w\\s\\w\\w\\w\\s💩💩\\.$"
            )
        )]
        fn succeeds(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of(&[Feature::Space, Feature::Word])
                .build();
            test_if_regexp_is_correct(regexp, expected_output, &test_cases);
            test_if_regexp_matches_test_cases(expected_output, test_cases);
        }

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and 💩💩."],
                "^\\w\\s\\s\\s\\u{2665}\\u{2665}\\u{2665}\\s\\w\\w\\s\\w\\w\\w\\s\\w\\s\\w\\w\\w\\s\\u{1f4a9}\\u{1f4a9}\\.$"
            )
        )]
        fn succeeds_with_escape_option(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of(&[Feature::Space, Feature::Word])
                .with_escaping_of_non_ascii_chars(false)
                .build();
            test_if_regexp_is_correct(regexp, expected_output, &test_cases);
            test_if_regexp_matches_test_cases(expected_output, test_cases);
        }

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and 💩💩."],
                "^\\w\\s\\s\\s\\u{2665}\\u{2665}\\u{2665}\\s\\w\\w\\s\\w\\w\\w\\s\\w\\s\\w\\w\\w\\s\\u{d83d}\\u{dca9}\\u{d83d}\\u{dca9}\\.$"
            )
        )]
        fn succeeds_with_escape_and_surrogate_option(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of(&[Feature::Space, Feature::Word])
                .with_escaping_of_non_ascii_chars(true)
                .build();
            test_if_regexp_is_correct(regexp, expected_output, &test_cases);
        }
    }

    mod repetition {
        use super::*;

        #[rstest(test_cases, expected_output,
            case(vec!["I   ♥♥♥ 36 and ٣ and 💩💩."], "^\\w\\s{3}♥{3}\\s\\w(\\w\\s\\w{3}\\s){2}💩{2}\\.$")
        )]
        fn succeeds(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of(&[Feature::Repetition, Feature::Space, Feature::Word])
                .build();
            test_if_regexp_is_correct(regexp, expected_output, &test_cases);
            test_if_regexp_matches_test_cases(expected_output, test_cases);
        }

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and 💩💩."],
                "^\\w\\s{3}\\u{2665}{3}\\s\\w(\\w\\s\\w{3}\\s){2}\\u{1f4a9}{2}\\.$"
            )
        )]
        fn succeeds_with_escape_option(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of(&[Feature::Repetition, Feature::Space, Feature::Word])
                .with_escaping_of_non_ascii_chars(false)
                .build();
            test_if_regexp_is_correct(regexp, expected_output, &test_cases);
            test_if_regexp_matches_test_cases(expected_output, test_cases);
        }

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and 💩💩."],
                "^\\w\\s{3}\\u{2665}{3}\\s\\w(\\w\\s\\w{3}\\s){2}(\\u{d83d}\\u{dca9}){2}\\.$"
            )
        )]
        fn succeeds_with_escape_and_surrogate_option(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of(&[Feature::Repetition, Feature::Space, Feature::Word])
                .with_escaping_of_non_ascii_chars(true)
                .build();
            test_if_regexp_is_correct(regexp, expected_output, &test_cases);
        }
    }
}

mod digit_space_word_conversion {
    use super::*;

    mod no_repetition {
        use super::*;

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and 💩💩."],
                "^\\w\\s\\s\\s♥♥♥\\s\\d\\d\\s\\w\\w\\w\\s\\d\\s\\w\\w\\w\\s💩💩\\.$"
            )
        )]
        fn succeeds(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of(&[Feature::Digit, Feature::Space, Feature::Word])
                .build();
            test_if_regexp_is_correct(regexp, expected_output, &test_cases);
            test_if_regexp_matches_test_cases(expected_output, test_cases);
        }

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and 💩💩."],
                "^\\w\\s\\s\\s\\u{2665}\\u{2665}\\u{2665}\\s\\d\\d\\s\\w\\w\\w\\s\\d\\s\\w\\w\\w\\s\\u{1f4a9}\\u{1f4a9}\\.$"
            )
        )]
        fn succeeds_with_escape_option(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of(&[Feature::Digit, Feature::Space, Feature::Word])
                .with_escaping_of_non_ascii_chars(false)
                .build();
            test_if_regexp_is_correct(regexp, expected_output, &test_cases);
            test_if_regexp_matches_test_cases(expected_output, test_cases);
        }

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and 💩💩."],
                "^\\w\\s\\s\\s\\u{2665}\\u{2665}\\u{2665}\\s\\d\\d\\s\\w\\w\\w\\s\\d\\s\\w\\w\\w\\s\\u{d83d}\\u{dca9}\\u{d83d}\\u{dca9}\\.$"
            )
        )]
        fn succeeds_with_escape_and_surrogate_option(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of(&[Feature::Digit, Feature::Space, Feature::Word])
                .with_escaping_of_non_ascii_chars(true)
                .build();
            test_if_regexp_is_correct(regexp, expected_output, &test_cases);
        }
    }

    mod repetition {
        use super::*;

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and 💩💩."],
                "^\\w\\s{3}♥{3}\\s\\d(\\d\\s\\w{3}\\s){2}💩{2}\\.$"
            )
        )]
        fn succeeds(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of(&[
                    Feature::Repetition,
                    Feature::Digit,
                    Feature::Space,
                    Feature::Word,
                ])
                .build();
            test_if_regexp_is_correct(regexp, expected_output, &test_cases);
            test_if_regexp_matches_test_cases(expected_output, test_cases);
        }

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and 💩💩."],
                "^\\w\\s{3}\\u{2665}{3}\\s\\d(\\d\\s\\w{3}\\s){2}\\u{1f4a9}{2}\\.$"
            )
        )]
        fn succeeds_with_escape_option(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of(&[
                    Feature::Repetition,
                    Feature::Digit,
                    Feature::Space,
                    Feature::Word,
                ])
                .with_escaping_of_non_ascii_chars(false)
                .build();
            test_if_regexp_is_correct(regexp, expected_output, &test_cases);
            test_if_regexp_matches_test_cases(expected_output, test_cases);
        }

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and 💩💩."],
                "^\\w\\s{3}\\u{2665}{3}\\s\\d(\\d\\s\\w{3}\\s){2}(\\u{d83d}\\u{dca9}){2}\\.$"
            )
        )]
        fn succeeds_with_escape_and_surrogate_option(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of(&[
                    Feature::Repetition,
                    Feature::Digit,
                    Feature::Space,
                    Feature::Word,
                ])
                .with_escaping_of_non_ascii_chars(true)
                .build();
            test_if_regexp_is_correct(regexp, expected_output, &test_cases);
        }
    }
}

mod non_digit_conversion {
    use super::*;

    mod no_repetition {
        use super::*;

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and 💩💩."],
                "^\\D\\D\\D\\D\\D\\D\\D\\D36\\D\\D\\D\\D\\D٣\\D\\D\\D\\D\\D\\D\\D\\D$"
            )
        )]
        fn succeeds(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of(&[Feature::NonDigit])
                .build();
            test_if_regexp_is_correct(regexp, expected_output, &test_cases);
            test_if_regexp_matches_test_cases(expected_output, test_cases);
        }

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and 💩💩."],
                "^\\D\\D\\D\\D\\D\\D\\D\\D36\\D\\D\\D\\D\\D\\u{663}\\D\\D\\D\\D\\D\\D\\D\\D$"
            )
        )]
        fn succeeds_with_escape_option(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of(&[Feature::NonDigit])
                .with_escaping_of_non_ascii_chars(false)
                .build();
            test_if_regexp_is_correct(regexp, expected_output, &test_cases);
            test_if_regexp_matches_test_cases(expected_output, test_cases);
        }
    }

    mod repetition {
        use super::*;

        #[rstest(test_cases, expected_output,
            case(vec!["I   ♥♥♥ 36 and ٣ and 💩💩."], "^\\D{8}36\\D{5}٣\\D{8}$")
        )]
        fn succeeds(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of(&[Feature::Repetition, Feature::NonDigit])
                .build();
            test_if_regexp_is_correct(regexp, expected_output, &test_cases);
            test_if_regexp_matches_test_cases(expected_output, test_cases);
        }

        #[rstest(test_cases, expected_output,
            case(vec!["I   ♥♥♥ 36 and ٣ and 💩💩."], "^\\D{8}36\\D{5}\\u{663}\\D{8}$")
        )]
        fn succeeds_with_escape_option(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of(&[Feature::Repetition, Feature::NonDigit])
                .with_escaping_of_non_ascii_chars(false)
                .build();
            test_if_regexp_is_correct(regexp, expected_output, &test_cases);
            test_if_regexp_matches_test_cases(expected_output, test_cases);
        }
    }
}

mod non_space_conversion {
    use super::*;

    mod no_repetition {
        use super::*;

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and 💩💩."],
                "^\\S   \\S\\S\\S \\S\\S \\S\\S\\S \\S \\S\\S\\S \\S\\S\\S$"
            )
        )]
        fn succeeds(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of(&[Feature::NonSpace])
                .build();
            test_if_regexp_is_correct(regexp, expected_output, &test_cases);
            test_if_regexp_matches_test_cases(expected_output, test_cases);
        }
    }

    mod repetition {
        use super::*;

        #[rstest(test_cases, expected_output,
            case(vec!["I   ♥♥♥ 36 and ٣ and 💩💩."], "^\\S {3}\\S{3} \\S(\\S \\S{3} ){2}\\S{3}$")
        )]
        fn succeeds(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of(&[Feature::Repetition, Feature::NonSpace])
                .build();
            test_if_regexp_is_correct(regexp, expected_output, &test_cases);
            test_if_regexp_matches_test_cases(expected_output, test_cases);
        }
    }
}

mod non_word_conversion {
    use super::*;

    mod no_repetition {
        use super::*;

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and 💩💩."],
                "^I\\W\\W\\W\\W\\W\\W\\W36\\Wand\\W٣\\Wand\\W\\W\\W\\W$"
            )
        )]
        fn succeeds(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of(&[Feature::NonWord])
                .build();
            test_if_regexp_is_correct(regexp, expected_output, &test_cases);
            test_if_regexp_matches_test_cases(expected_output, test_cases);
        }

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and 💩💩."],
                "^I\\W\\W\\W\\W\\W\\W\\W36\\Wand\\W\\u{663}\\Wand\\W\\W\\W\\W$"
            )
        )]
        fn succeeds_with_escape_option(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of(&[Feature::NonWord])
                .with_escaping_of_non_ascii_chars(false)
                .build();
            test_if_regexp_is_correct(regexp, expected_output, &test_cases);
            test_if_regexp_matches_test_cases(expected_output, test_cases);
        }
    }

    mod repetition {
        use super::*;

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and 💩💩."],
                "^I\\W{7}36\\Wand\\W٣\\Wand\\W{4}$"
            )
        )]
        fn succeeds(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of(&[Feature::Repetition, Feature::NonWord])
                .build();
            test_if_regexp_is_correct(regexp, expected_output, &test_cases);
            test_if_regexp_matches_test_cases(expected_output, test_cases);
        }

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and 💩💩."],
                "^I\\W{7}36\\Wand\\W\\u{663}\\Wand\\W{4}$"
            )
        )]
        fn succeeds_with_escape_option(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of(&[Feature::Repetition, Feature::NonWord])
                .with_escaping_of_non_ascii_chars(false)
                .build();
            test_if_regexp_is_correct(regexp, expected_output, &test_cases);
            test_if_regexp_matches_test_cases(expected_output, test_cases);
        }
    }
}

mod non_digit_non_space_conversion {
    use super::*;

    mod no_repetition {
        use super::*;

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and 💩💩."],
                "^\\D\\D\\D\\D\\D\\D\\D\\D\\S\\S\\D\\D\\D\\D\\D\\S\\D\\D\\D\\D\\D\\D\\D\\D$"
            )
        )]
        fn succeeds(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of(&[Feature::NonDigit, Feature::NonSpace])
                .build();
            test_if_regexp_is_correct(regexp, expected_output, &test_cases);
            test_if_regexp_matches_test_cases(expected_output, test_cases);
        }
    }

    mod repetition {
        use super::*;

        #[rstest(test_cases, expected_output,
            case(vec!["I   ♥♥♥ 36 and ٣ and 💩💩."], "^\\D{8}\\S{2}\\D{5}\\S\\D{8}$")
        )]
        fn succeeds(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of(&[Feature::Repetition, Feature::NonDigit, Feature::NonSpace])
                .build();
            test_if_regexp_is_correct(regexp, expected_output, &test_cases);
            test_if_regexp_matches_test_cases(expected_output, test_cases);
        }
    }
}

mod non_digit_non_word_conversion {
    use super::*;

    mod no_repetition {
        use super::*;

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and 💩💩."],
                "^\\D\\D\\D\\D\\D\\D\\D\\D36\\D\\D\\D\\D\\D٣\\D\\D\\D\\D\\D\\D\\D\\D$"
            )
        )]
        fn succeeds(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of(&[Feature::NonDigit, Feature::NonWord])
                .build();
            test_if_regexp_is_correct(regexp, expected_output, &test_cases);
            test_if_regexp_matches_test_cases(expected_output, test_cases);
        }
    }

    mod repetition {
        use super::*;

        #[rstest(test_cases, expected_output,
            case(vec!["I   ♥♥♥ 36 and ٣ and 💩💩."], "^\\D{8}36\\D{5}٣\\D{8}$")
        )]
        fn succeeds(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of(&[Feature::Repetition, Feature::NonDigit, Feature::NonWord])
                .build();
            test_if_regexp_is_correct(regexp, expected_output, &test_cases);
            test_if_regexp_matches_test_cases(expected_output, test_cases);
        }
    }
}

mod non_space_non_word_conversion {
    use super::*;

    mod no_repetition {
        use super::*;

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and 💩💩."],
                "^\\S\\W\\W\\W\\W\\W\\W\\W\\S\\S\\W\\S\\S\\S\\W\\S\\W\\S\\S\\S\\W\\W\\W\\W$"
            )
        )]
        fn succeeds(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of(&[Feature::NonSpace, Feature::NonWord])
                .build();
            test_if_regexp_is_correct(regexp, expected_output, &test_cases);
            test_if_regexp_matches_test_cases(expected_output, test_cases);
        }
    }

    mod repetition {
        use super::*;

        #[rstest(test_cases, expected_output,
            case(vec!["I   ♥♥♥ 36 and ٣ and 💩💩."], "^\\S\\W{7}(\\S{2}\\W\\S){2}\\W\\S{3}\\W{4}$")
        )]
        fn succeeds(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of(&[Feature::Repetition, Feature::NonSpace, Feature::NonWord])
                .build();
            test_if_regexp_is_correct(regexp, expected_output, &test_cases);
            test_if_regexp_matches_test_cases(expected_output, test_cases);
        }
    }
}

mod non_digit_non_space_non_word_conversion {
    use super::*;

    mod no_repetition {
        use super::*;

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and 💩💩."],
                "^\\D\\D\\D\\D\\D\\D\\D\\D\\S\\S\\D\\D\\D\\D\\D\\S\\D\\D\\D\\D\\D\\D\\D\\D$"
            )
        )]
        fn succeeds(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of(&[Feature::NonDigit, Feature::NonSpace, Feature::NonWord])
                .build();
            test_if_regexp_is_correct(regexp, expected_output, &test_cases);
            test_if_regexp_matches_test_cases(expected_output, test_cases);
        }
    }

    mod repetition {
        use super::*;

        #[rstest(test_cases, expected_output,
            case(vec!["I   ♥♥♥ 36 and ٣ and 💩💩."], "^\\D{8}\\S{2}\\D{5}\\S\\D{8}$")
        )]
        fn succeeds(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of(&[
                    Feature::Repetition,
                    Feature::NonDigit,
                    Feature::NonSpace,
                    Feature::NonWord,
                ])
                .build();
            test_if_regexp_is_correct(regexp, expected_output, &test_cases);
            test_if_regexp_matches_test_cases(expected_output, test_cases);
        }
    }
}

mod digit_non_digit_conversion {
    use super::*;

    mod no_repetition {
        use super::*;

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and 💩💩."],
                "^\\D\\D\\D\\D\\D\\D\\D\\D\\d\\d\\D\\D\\D\\D\\D\\d\\D\\D\\D\\D\\D\\D\\D\\D$"
            )
        )]
        fn succeeds(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of(&[Feature::Digit, Feature::NonDigit])
                .build();
            test_if_regexp_is_correct(regexp, expected_output, &test_cases);
            test_if_regexp_matches_test_cases(expected_output, test_cases);
        }
    }

    mod repetition {
        use super::*;

        #[rstest(test_cases, expected_output,
            case(vec!["I   ♥♥♥ 36 and ٣ and 💩💩."], "^\\D{8}\\d{2}\\D{5}\\d\\D{8}$")
        )]
        fn succeeds(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of(&[Feature::Repetition, Feature::Digit, Feature::NonDigit])
                .build();
            test_if_regexp_is_correct(regexp, expected_output, &test_cases);
            test_if_regexp_matches_test_cases(expected_output, test_cases);
        }
    }
}

mod space_non_space_conversion {
    use super::*;

    mod no_repetition {
        use super::*;

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and 💩💩."],
                "^\\S\\s\\s\\s\\S\\S\\S\\s\\S\\S\\s\\S\\S\\S\\s\\S\\s\\S\\S\\S\\s\\S\\S\\S$"
            )
        )]
        fn succeeds(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of(&[Feature::Space, Feature::NonSpace])
                .build();
            test_if_regexp_is_correct(regexp, expected_output, &test_cases);
            test_if_regexp_matches_test_cases(expected_output, test_cases);
        }
    }

    mod repetition {
        use super::*;

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and 💩💩."],
                "^\\S\\s{3}\\S{3}\\s\\S(\\S\\s\\S{3}\\s){2}\\S{3}$"
            )
        )]
        fn succeeds(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of(&[Feature::Repetition, Feature::Space, Feature::NonSpace])
                .build();
            test_if_regexp_is_correct(regexp, expected_output, &test_cases);
            test_if_regexp_matches_test_cases(expected_output, test_cases);
        }
    }
}

mod word_non_word_conversion {
    use super::*;

    mod no_repetition {
        use super::*;

        #[rstest(test_cases, expected_output,
            case(
                vec!["I   ♥♥♥ 36 and ٣ and 💩💩."],
                "^\\w\\W\\W\\W\\W\\W\\W\\W\\w\\w\\W\\w\\w\\w\\W\\w\\W\\w\\w\\w\\W\\W\\W\\W$"
            )
        )]
        fn succeeds(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of(&[Feature::Word, Feature::NonWord])
                .build();
            test_if_regexp_is_correct(regexp, expected_output, &test_cases);
            test_if_regexp_matches_test_cases(expected_output, test_cases);
        }
    }

    mod repetition {
        use super::*;

        #[rstest(test_cases, expected_output,
            case(vec!["I   ♥♥♥ 36 and ٣ and 💩💩."], "^\\w\\W{7}(\\w{2}\\W\\w){2}\\W\\w{3}\\W{4}$")
        )]
        fn succeeds(test_cases: Vec<&str>, expected_output: &str) {
            let regexp = RegExpBuilder::from(&test_cases)
                .with_conversion_of(&[Feature::Repetition, Feature::Word, Feature::NonWord])
                .build();
            test_if_regexp_is_correct(regexp, expected_output, &test_cases);
            test_if_regexp_matches_test_cases(expected_output, test_cases);
        }
    }
}

fn test_if_regexp_is_correct(regexp: String, expected_output: &str, test_cases: &[&str]) {
    assert_eq!(
        regexp, expected_output,
        "\n\ninput: {:?}\nexpected: {}\nactual: {}\n\n",
        test_cases, expected_output, regexp
    );
}

fn test_if_regexp_matches_test_cases(expected_output: &str, test_cases: Vec<&str>) {
    let re = Regex::new(expected_output).unwrap();
    for test_case in test_cases {
        assert!(
            re.is_match(test_case),
            "\n\n\"{}\" does not match regex {}\n\n",
            test_case,
            expected_output
        );
    }
}
