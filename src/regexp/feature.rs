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
    pub(crate) fn is_char_class(&self) -> bool {
        match self {
            Feature::Repetition => false,
            _ => true,
        }
    }
}
