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

use crate::ast::Expression;
use crate::color::colorize;
use crate::dfa::DFA;
use crate::grapheme::GraphemeCluster;
use colored::Colorize;
use itertools::Itertools;
use std::clone::Clone;
use std::cmp::Ordering;
use std::fmt::{Display, Error, Formatter, Result};
use std::io::ErrorKind;
use std::path::PathBuf;

#[derive(Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub(crate) struct RegExpConfig {
    pub(crate) conversion_features: Vec<Feature>,
    pub(crate) minimum_repetitions: u32,
    pub(crate) minimum_substring_length: u32,
    pub(crate) is_non_ascii_char_escaped: bool,
    pub(crate) is_astral_code_point_converted_to_surrogate: bool,
    pub(crate) is_output_colorized: bool,
}

impl RegExpConfig {
    pub(crate) fn new() -> Self {
        Self {
            conversion_features: vec![],
            minimum_repetitions: 2,
            minimum_substring_length: 1,
            is_non_ascii_char_escaped: false,
            is_astral_code_point_converted_to_surrogate: false,
            is_output_colorized: false,
        }
    }

    pub(crate) fn is_digit_converted(&self) -> bool {
        self.conversion_features.contains(&Feature::Digit)
    }

    pub(crate) fn is_non_digit_converted(&self) -> bool {
        self.conversion_features.contains(&Feature::NonDigit)
    }

    pub(crate) fn is_space_converted(&self) -> bool {
        self.conversion_features.contains(&Feature::Space)
    }

    pub(crate) fn is_non_space_converted(&self) -> bool {
        self.conversion_features.contains(&Feature::NonSpace)
    }

    pub(crate) fn is_word_converted(&self) -> bool {
        self.conversion_features.contains(&Feature::Word)
    }

    pub(crate) fn is_non_word_converted(&self) -> bool {
        self.conversion_features.contains(&Feature::NonWord)
    }

    pub(crate) fn is_repetition_converted(&self) -> bool {
        self.conversion_features.contains(&Feature::Repetition)
    }

    pub(crate) fn is_case_insensitive_matching(&self) -> bool {
        self.conversion_features
            .contains(&Feature::CaseInsensitivity)
    }

    pub(crate) fn is_capturing_group_enabled(&self) -> bool {
        self.conversion_features.contains(&Feature::CapturingGroup)
    }

    pub(crate) fn is_char_class_feature_enabled(&self) -> bool {
        self.conversion_features.iter().any(|it| it.is_char_class())
    }
}

/// This struct builds regular expressions from user-provided test cases.
pub struct RegExpBuilder {
    test_cases: Vec<String>,
    config: RegExpConfig,
}

impl RegExpBuilder {
    /// Specifies the test cases to build the regular expression from.
    ///
    /// The test cases need not be sorted because `RegExpBuilder` sorts them internally.
    ///
    /// ⚠ Panics if `test_cases` is empty.
    pub fn from<T: Clone + Into<String>>(test_cases: &[T]) -> Self {
        if test_cases.is_empty() {
            panic!("No test cases have been provided for regular expression generation");
        }
        Self {
            test_cases: test_cases.iter().cloned().map(|it| it.into()).collect_vec(),
            config: RegExpConfig::new(),
        }
    }

    /// Specifies a text file containing test cases to build the regular expression from.
    ///
    /// The test cases need not be sorted because `RegExpBuilder` sorts them internally.
    ///
    /// Each test case needs to be on a separate line.
    /// Lines may be ended with either a newline (`\n`) or
    /// a carriage return with a line feed (`\r\n`).
    /// The final line ending is optional.
    ///
    /// ⚠ Panics if:
    /// - the file cannot be found
    /// - the file's encoding is not valid UTF-8 data
    /// - the file cannot be opened because of conflicting permissions
    pub fn from_file<T: Into<PathBuf>>(file_path: T) -> Self {
        match std::fs::read_to_string(file_path.into()) {
            Ok(file_content) => Self {
                test_cases: file_content.lines().map(|it| it.to_string()).collect_vec(),
                config: RegExpConfig::new(),
            },
            Err(error) => match error.kind() {
                ErrorKind::NotFound => panic!("The specified file could not be found"),
                ErrorKind::InvalidData => {
                    panic!("The specified file's encoding is not valid UTF-8")
                }
                ErrorKind::PermissionDenied => {
                    panic!("Permission denied: The specified file could not be opened")
                }
                _ => panic!(error),
            },
        }
    }

    /// Tells `RegExpBuilder` which conversions should be performed during
    /// regular expression generation. The available conversion features
    /// are listed in the [`Feature`](./enum.Feature.html#variants) enum.
    ///
    /// ⚠ Panics if `features` is empty.
    pub fn with_conversion_of(&mut self, features: &[Feature]) -> &mut Self {
        if features.is_empty() {
            panic!("No conversion features have been provided for regular expression generation");
        }
        self.config.conversion_features = features.to_vec();
        self
    }

    /// Specifies the minimum quantity of substring repetitions to be converted if
    /// [`Feature::Repetition`](./enum.Feature.html#variant.Repetition)
    /// is set as one of the features in method
    /// [`with_conversion_of`](./struct.RegExpBuilder.html#method.with_conversion_of).
    ///
    /// If the quantity is not explicitly set with this method, a default value of 2 will be used.
    ///
    /// ⚠ Panics if `quantity` is less than 2.
    pub fn with_minimum_repetitions(&mut self, quantity: u32) -> &mut Self {
        if quantity < 2 {
            panic!(format!(
                "Quantity of minimum repetitions must not be less than 2 but is {}",
                quantity
            ));
        }
        self.config.minimum_repetitions = quantity;
        self
    }

    /// Specifies the minimum length a repeated substring must have in order to be converted if
    /// [`Feature::Repetition`](./enum.Feature.html#variant.Repetition)
    /// is set as one of the features in method
    /// [`with_conversion_of`](./struct.RegExpBuilder.html#method.with_conversion_of).
    ///
    /// If the length is not explicitly set with this method, a default value of 1 will be used.
    ///
    /// ⚠ Panics if `length` is zero.
    pub fn with_minimum_substring_length(&mut self, length: u32) -> &mut Self {
        if length == 0 {
            panic!("Minimum substring length must not be zero");
        }
        self.config.minimum_substring_length = length;
        self
    }

    /// Tells `RegExpBuilder` to convert non-ASCII characters to unicode escape sequences.
    /// The parameter `use_surrogate_pairs` specifies whether to convert astral code planes
    /// (range `U+010000` to `U+10FFFF`) to surrogate pairs.
    pub fn with_escaping_of_non_ascii_chars(&mut self, use_surrogate_pairs: bool) -> &mut Self {
        self.config.is_non_ascii_char_escaped = true;
        self.config.is_astral_code_point_converted_to_surrogate = use_surrogate_pairs;
        self
    }

    /// Tells `RegExpBuilder` to provide syntax highlighting for the resulting regular expression.
    ///
    /// ⚠ This method may only be used if the resulting regular expression is meant to
    /// be printed to the console. The regex string representation returned from enabling
    /// this setting cannot be fed into the [*regex*](https://crates.io/crates/regex) crate.
    pub fn with_syntax_highlighting(&mut self) -> &mut Self {
        self.config.is_output_colorized = true;
        self
    }

    /// Builds the actual regular expression using the previously given settings.
    /// Every generated regular expression is surrounded by the anchors `^` and `$`
    /// so that substrings not being part of the test cases are not matched accidentally.
    pub fn build(&mut self) -> String {
        RegExp::from(&mut self.test_cases, &self.config).to_string()
    }
}

pub(crate) struct RegExp {
    ast: Expression,
    config: RegExpConfig,
}

impl RegExp {
    fn from(test_cases: &mut Vec<String>, config: &RegExpConfig) -> Self {
        if config.is_case_insensitive_matching() {
            Self::convert_to_lowercase(test_cases);
        }
        Self::sort(test_cases);
        let grapheme_clusters = Self::grapheme_clusters(&test_cases, config);
        let dfa = DFA::from(grapheme_clusters, config);
        let ast = Expression::from(dfa, config);
        Self {
            ast,
            config: config.clone(),
        }
    }

    fn convert_to_lowercase(test_cases: &mut Vec<String>) {
        std::mem::replace(
            test_cases,
            test_cases.iter().map(|it| it.to_lowercase()).collect_vec(),
        );
    }

    fn sort(test_cases: &mut Vec<String>) {
        test_cases.sort();
        test_cases.dedup();
        test_cases.sort_by(|a, b| match a.len().cmp(&b.len()) {
            Ordering::Equal => a.cmp(&b),
            other => other,
        });
    }

    fn grapheme_clusters(test_cases: &[String], config: &RegExpConfig) -> Vec<GraphemeCluster> {
        let mut clusters = test_cases
            .iter()
            .map(|it| GraphemeCluster::from(it, config))
            .collect_vec();

        if config.is_char_class_feature_enabled() {
            for cluster in clusters.iter_mut() {
                cluster.convert_to_char_classes();
            }
        }

        if config.is_repetition_converted() {
            for cluster in clusters.iter_mut() {
                cluster.convert_repetitions();
            }
        }

        clusters
    }
}

impl Display for RegExp {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if let [case_insensitive_flag, left_anchor, right_anchor, left_non_capturing_parenthesis, left_capturing_parenthesis, right_parenthesis] =
            &colorize(
                vec!["(?i)", "^", "$", "(?:", "(", ")"],
                self.config.is_output_colorized,
            )[..]
        {
            let flag = if self.config.is_case_insensitive_matching() {
                case_insensitive_flag.clone()
            } else {
                "".clear()
            };

            match self.ast {
                Expression::Alternation(_, _) => {
                    let left_parenthesis = if self.config.is_capturing_group_enabled() {
                        left_capturing_parenthesis
                    } else {
                        left_non_capturing_parenthesis
                    };

                    write!(
                        f,
                        "{}{}{}{}{}{}",
                        flag,
                        left_anchor,
                        left_parenthesis,
                        self.ast.to_string(),
                        right_parenthesis,
                        right_anchor
                    )
                }
                _ => write!(
                    f,
                    "{}{}{}{}",
                    flag,
                    left_anchor,
                    self.ast.to_string(),
                    right_anchor
                ),
            }
        } else {
            Err(Error::default())
        }
    }
}

/// This enum specifies the supported conversion features which can be passed to method
/// [`RegExpBuilder.with_conversion_of`](./struct.RegExpBuilder.html#method.with_conversion_of).
#[derive(Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub enum Feature {
    /// This feature converts any Unicode decimal digit to character class `\d`.
    ///
    /// It takes precedence over the
    /// [`Word`](./enum.Feature.html#variant.Word) feature if both are set.
    /// Decimal digits are converted to `\d`, the remaining word characters to `\w`.
    ///
    /// It takes precedence over the
    /// [`NonSpace`](./enum.Feature.html#variant.NonSpace) feature if both are set.
    /// Decimal digits are converted to `\d`, the remaining non-whitespace characters to `\S`.
    Digit,

    /// This feature converts any character which is not
    /// a Unicode decimal digit to character class `\D`.
    ///
    /// It takes precedence over the
    /// [`NonWord`](./enum.Feature.html#variant.NonWord) feature if both are set.
    /// Non-digits which are also non-word characters are converted to `\D`.
    ///
    /// It takes precedence over the
    /// [`NonSpace`](./enum.Feature.html#variant.NonSpace) feature if both are set.
    /// Non-digits which are also non-space characters are converted to `\D`.
    NonDigit,

    /// This feature converts any Unicode whitespace character to character class `\s`.
    ///
    /// It takes precedence over the
    /// [`NonDigit`](./enum.Feature.html#variant.NonDigit) feature if both are set.
    /// Whitespace characters are converted to `\s`, the remaining non-digit characters to `\D`.
    ///
    /// It takes precedence over the
    /// [`NonWord`](./enum.Feature.html#variant.NonWord) feature if both are set.
    /// Whitespace characters are converted to `\s`, the remaining non-word characters to `\W`.
    Space,

    /// This feature converts any character which is not
    /// a Unicode whitespace character to character class `\S`.
    NonSpace,

    /// This feature converts any Unicode word character to character class `\w`.
    ///
    /// It takes precedence over the
    /// [`NonDigit`](./enum.Feature.html#variant.NonDigit) feature if both are set.
    /// Word characters are converted to `\w`, the remaining non-digit characters to `\D`.
    ///
    /// It takes precedence over the
    /// [`NonSpace`](./enum.Feature.html#variant.NonSpace) feature if both are set.
    /// Word characters are converted to `\w`, the remaining non-space characters to `\S`.
    Word,

    /// This feature converts any character which is not
    /// a Unicode word character to character class `\W`.
    ///
    /// It takes precedence over the
    /// [`NonSpace`](./enum.Feature.html#variant.NonSpace) feature if both are set.
    /// Non-words which are also non-space characters are converted to `\W`.
    NonWord,

    /// This feature detects repeated non-overlapping substrings and
    /// converts them to `{min,max}` quantifier notation.
    Repetition,

    /// This feature enables case-insensitive matching of test cases
    /// so that letters match both upper and lower case.
    CaseInsensitivity,

    /// This feature replaces non-capturing groups by capturing ones.
    CapturingGroup,
}

impl Feature {
    fn is_char_class(&self) -> bool {
        match self {
            Feature::Repetition => false,
            _ => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Feature, RegExpBuilder};

    #[test]
    #[should_panic(expected = "No test cases have been provided for regular expression generation")]
    fn regexp_builder_panics_without_test_cases() {
        RegExpBuilder::from(&Vec::<String>::new());
    }

    #[test]
    #[should_panic(
        expected = "No conversion features have been provided for regular expression generation"
    )]
    fn regexp_builder_panics_without_conversion_features() {
        RegExpBuilder::from(&["abc"]).with_conversion_of(&Vec::<Feature>::new());
    }

    #[test]
    #[should_panic(expected = "The specified file could not be found")]
    fn regexp_builder_panics_if_file_does_not_exist() {
        RegExpBuilder::from_file("/path/to/non-existing/file");
    }

    #[test]
    #[should_panic(expected = "Quantity of minimum repetitions must not be less than 2 but is 1")]
    fn regexp_builder_panics_if_minimum_repetitions_is_less_than_two() {
        RegExpBuilder::from(&["abc"]).with_minimum_repetitions(1);
    }

    #[test]
    #[should_panic(expected = "Minimum substring length must not be zero")]
    fn regexp_builder_panics_if_minimum_substring_length_is_zero() {
        RegExpBuilder::from(&["abc"]).with_minimum_substring_length(0);
    }
}
