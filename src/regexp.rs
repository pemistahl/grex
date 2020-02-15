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
use crate::dfa::DFA;
use crate::grapheme::GraphemeCluster;
use colored::Colorize;
use itertools::Itertools;
use std::clone::Clone;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter, Result};

#[derive(Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub(crate) struct RegExpConfig {
    pub(crate) conversion_features: Vec<Feature>,
    pub(crate) is_non_ascii_char_escaped: bool,
    pub(crate) is_astral_code_point_converted_to_surrogate: bool,
    pub(crate) is_output_colorized: bool,
}

impl RegExpConfig {
    pub(crate) fn new() -> Self {
        Self {
            conversion_features: vec![],
            is_non_ascii_char_escaped: false,
            is_astral_code_point_converted_to_surrogate: false,
            is_output_colorized: false,
        }
    }
}

/// This struct builds regular expressions from user-provided test cases.
pub struct RegExpBuilder {
    test_cases: Vec<String>,
    config: RegExpConfig,
}

impl RegExpBuilder {
    /// Specifies the test cases to build the regular expression from.
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
        Self::sort(test_cases);
        let grapheme_clusters = Self::grapheme_clusters(&test_cases, config);
        let dfa = DFA::from(grapheme_clusters, config);
        let ast = Expression::from(dfa, config);
        Self {
            ast,
            config: config.clone(),
        }
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

        if config
            .conversion_features
            .iter()
            .any(|it| it.is_char_class())
        {
            for cluster in clusters.iter_mut() {
                cluster.convert_to_char_classes();
            }
        }

        if config.conversion_features.contains(&Feature::Repetition) {
            for cluster in clusters.iter_mut() {
                cluster.convert_repetitions();
            }
        }

        clusters
    }
}

impl Display for RegExp {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let (left_anchor, right_anchor) = ["^", "$"]
            .iter()
            .map(|&it| {
                if self.config.is_output_colorized {
                    it.yellow().bold()
                } else {
                    it.clear()
                }
            })
            .collect_tuple()
            .unwrap();

        let (left_parenthesis, right_parenthesis) = ["(", ")"]
            .iter()
            .map(|&it| {
                if self.config.is_output_colorized {
                    it.green().bold()
                } else {
                    it.clear()
                }
            })
            .collect_tuple()
            .unwrap();

        match self.ast {
            Expression::Alternation(_, _) => write!(
                f,
                "{}{}{}{}{}",
                left_anchor,
                left_parenthesis,
                self.ast.to_string(),
                right_parenthesis,
                right_anchor
            ),
            _ => write!(f, "{}{}{}", left_anchor, self.ast.to_string(), right_anchor),
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
    #[should_panic]
    fn regexp_builder_panics_without_test_cases() {
        RegExpBuilder::from(&Vec::<String>::new());
    }

    #[test]
    #[should_panic]
    fn regexp_builder_panics_without_conversion_features() {
        RegExpBuilder::from(&["abc"]).with_conversion_of(&Vec::<Feature>::new());
    }
}
