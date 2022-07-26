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

#![allow(non_snake_case)]

use crate::builder::{
    RegExpBuilder as Builder, MINIMUM_REPETITIONS_MESSAGE, MINIMUM_SUBSTRING_LENGTH_MESSAGE,
    MISSING_TEST_CASES_MESSAGE,
};
use itertools::Itertools;
use wasm_bindgen::prelude::*;

/// This class builds regular expressions from user-provided test cases.
#[wasm_bindgen]
#[derive(Clone)]
pub struct RegExpBuilder {
    builder: Builder,
}

#[wasm_bindgen]
impl RegExpBuilder {
    /// Specifies the test cases to build the regular expression from.
    ///
    /// The test cases need not be sorted because `RegExpBuilder` sorts them internally.
    ///
    /// ⚠ Throws an error if `testCases` is empty.
    pub fn from(testCases: Box<[JsValue]>) -> Result<RegExpBuilder, JsValue> {
        let strs = testCases
            .iter()
            .filter_map(|it| it.as_string())
            .collect_vec();

        if strs.is_empty() {
            return Err(JsValue::from(MISSING_TEST_CASES_MESSAGE));
        }
        Ok(RegExpBuilder {
            builder: Builder::from(&strs),
        })
    }

    /// Tells `RegExpBuilder` to convert any Unicode decimal digit to character class `\d`.
    ///
    /// This method takes precedence over `withConversionOfWords` if both are set.
    /// Decimal digits are converted to `\d`, the remaining word characters to `\w`.
    ///
    /// This method takes precedence over `withConversionOfWhitespace` if both are set.
    /// Decimal digits are converted to `\d`, the remaining non-whitespace characters to `\S`.
    pub fn withConversionOfDigits(&mut self) -> RegExpBuilder {
        self.builder.config.is_digit_converted = true;
        self.clone()
    }

    /// Tells `RegExpBuilder` to convert any character which is not
    /// a Unicode decimal digit to character class `\D`.
    ///
    /// This method takes precedence over `withConversionOfNonWords` if both are set.
    /// Non-digits which are also non-word characters are converted to `\D`.
    ///
    /// This method takes precedence over `withConversionOfNonWhitespace` if both are set.
    /// Non-digits which are also non-space characters are converted to `\D`.
    pub fn withConversionOfNonDigits(&mut self) -> RegExpBuilder {
        self.builder.config.is_non_digit_converted = true;
        self.clone()
    }

    /// Tells `RegExpBuilder` to convert any Unicode whitespace character to character class `\s`.
    ///
    /// This method takes precedence over `withConversionOfNonDigits` if both are set.
    /// Whitespace characters are converted to `\s`, the remaining non-digit characters to `\D`.
    ///
    /// This method takes precedence over `withConversionOfNonWords` if both are set.
    /// Whitespace characters are converted to `\s`, the remaining non-word characters to `\W`.
    pub fn withConversionOfWhitespace(&mut self) -> RegExpBuilder {
        self.builder.config.is_space_converted = true;
        self.clone()
    }

    /// Tells `RegExpBuilder` to convert any character which is not
    /// a Unicode whitespace character to character class `\S`.
    pub fn withConversionOfNonWhitespace(&mut self) -> RegExpBuilder {
        self.builder.config.is_non_space_converted = true;
        self.clone()
    }

    /// Tells `RegExpBuilder` to convert any Unicode word character to character class `\w`.
    ///
    /// This method takes precedence over `withConversionOfNonDigits` if both are set.
    /// Word characters are converted to `\w`, the remaining non-digit characters to `\D`.
    ///
    /// This method takes precedence over `withConversionOfNonWhitespace` if both are set.
    /// Word characters are converted to `\w`, the remaining non-space characters to `\S`.
    pub fn withConversionOfWords(&mut self) -> RegExpBuilder {
        self.builder.config.is_word_converted = true;
        self.clone()
    }

    /// Tells `RegExpBuilder` to convert any character which is not
    /// a Unicode word character to character class `\W`.
    ///
    /// This method takes precedence over `withConversionOfNonWhitespace` if both are set.
    /// Non-words which are also non-space characters are converted to `\W`.
    pub fn withConversionOfNonWords(&mut self) -> RegExpBuilder {
        self.builder.config.is_non_word_converted = true;
        self.clone()
    }

    /// Tells `RegExpBuilder` to detect repeated non-overlapping substrings and
    /// to convert them to `{min,max}` quantifier notation.
    pub fn withConversionOfRepetitions(&mut self) -> RegExpBuilder {
        self.builder.config.is_repetition_converted = true;
        self.clone()
    }

    /// Tells `RegExpBuilder` to enable case-insensitive matching of test cases
    /// so that letters match both upper and lower case.
    pub fn withCaseInsensitiveMatching(&mut self) -> RegExpBuilder {
        self.builder.config.is_case_insensitive_matching = true;
        self.clone()
    }

    /// Tells `RegExpBuilder` to replace non-capturing groups by capturing ones.
    pub fn withCapturingGroups(&mut self) -> RegExpBuilder {
        self.builder.config.is_capturing_group_enabled = true;
        self.clone()
    }

    /// Tells `RegExpBuilder` to convert non-ASCII characters to unicode escape sequences.
    /// The parameter `useSurrogatePairs` specifies whether to convert astral code planes
    /// (range `U+010000` to `U+10FFFF`) to surrogate pairs.
    pub fn withEscapingOfNonAsciiChars(&mut self, useSurrogatePairs: bool) -> RegExpBuilder {
        self.builder.config.is_non_ascii_char_escaped = true;
        self.builder
            .config
            .is_astral_code_point_converted_to_surrogate = useSurrogatePairs;
        self.clone()
    }

    /// Tells `RegExpBuilder` to produce a nicer looking regular expression in verbose mode.
    pub fn withVerboseMode(&mut self) -> RegExpBuilder {
        self.builder.config.is_verbose_mode_enabled = true;
        self.clone()
    }

    /// Tells `RegExpBuilder` to remove the caret anchor '^' from the resulting regular
    /// expression, thereby allowing to match the test cases also when they do not occur
    /// at the start of a string.
    pub fn withoutStartAnchor(&mut self) -> RegExpBuilder {
        self.builder.config.is_start_anchor_disabled = true;
        self.clone()
    }

    /// Tells `RegExpBuilder` to remove the dollar sign anchor '$' from the resulting regular
    /// expression, thereby allowing to match the test cases also when they do not occur
    /// at the end of a string.
    pub fn withoutEndAnchor(&mut self) -> RegExpBuilder {
        self.builder.config.is_end_anchor_disabled = true;
        self.clone()
    }

    /// Tells `RegExpBuilder` to remove the caret and dollar sign anchors from the resulting
    /// regular expression, thereby allowing to match the test cases also when they occur
    /// within a larger string that contains other content as well.
    pub fn withoutAnchors(&mut self) -> RegExpBuilder {
        self.builder.config.is_start_anchor_disabled = true;
        self.builder.config.is_end_anchor_disabled = true;
        self.clone()
    }

    /// Specifies the minimum quantity of substring repetitions to be converted
    /// if `withConversionOfRepetitions` is set.
    ///
    /// If the quantity is not explicitly set with this method, a default value of 1 will be used.
    ///
    /// ⚠ Throws an error if `quantity` is zero.
    pub fn withMinimumRepetitions(&mut self, quantity: u32) -> Result<RegExpBuilder, JsValue> {
        if quantity < 1 {
            return Err(JsValue::from(MINIMUM_REPETITIONS_MESSAGE));
        }
        self.builder.config.minimum_repetitions = quantity;
        Ok(self.clone())
    }

    /// Specifies the minimum length a repeated substring must have in order to be converted
    /// if `withConversionOfRepetitions` is set.
    ///
    /// If the length is not explicitly set with this method, a default value of 1 will be used.
    ///
    /// ⚠ Throws an error if `length` is zero.
    pub fn withMinimumSubstringLength(&mut self, length: u32) -> Result<RegExpBuilder, JsValue> {
        if length < 1 {
            return Err(JsValue::from(MINIMUM_SUBSTRING_LENGTH_MESSAGE));
        }
        self.builder.config.minimum_substring_length = length;
        Ok(self.clone())
    }

    /// Builds the actual regular expression using the previously given settings.
    pub fn build(&mut self) -> String {
        self.builder.build()
    }
}
