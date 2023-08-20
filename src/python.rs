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

use crate::builder::{
    RegExpBuilder, MINIMUM_REPETITIONS_MESSAGE, MINIMUM_SUBSTRING_LENGTH_MESSAGE,
    MISSING_TEST_CASES_MESSAGE,
};
use crate::config::RegExpConfig;
use lazy_static::lazy_static;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyType;
use regex::{Captures, Regex};

#[pymodule]
fn grex(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<RegExpBuilder>()?;
    Ok(())
}

#[pymethods]
impl RegExpBuilder {
    #[new]
    fn new(test_cases: Vec<String>) -> PyResult<Self> {
        if test_cases.is_empty() {
            Err(PyValueError::new_err(MISSING_TEST_CASES_MESSAGE))
        } else {
            Ok(Self {
                test_cases,
                config: RegExpConfig::new(),
            })
        }
    }

    /// Specify the test cases to build the regular expression from.
    ///
    /// The test cases need not be sorted because `RegExpBuilder` sorts them internally.
    ///
    /// Args:
    ///     test_cases (list[str]): The list of test cases
    ///
    /// Raises:
    ///     ValueError: if `test_cases` is empty
    #[classmethod]
    fn from_test_cases(_cls: &PyType, test_cases: Vec<String>) -> PyResult<Self> {
        Self::new(test_cases)
    }

    /// Convert any Unicode decimal digit to character class `\d`.
    ///
    /// This method takes precedence over `with_conversion_of_words` if both are set.
    /// Decimal digits are converted to `\d`, the remaining word characters to `\w`.
    ///
    /// This method takes precedence over `with_conversion_of_non_whitespace` if both are set.
    /// Decimal digits are converted to `\d`, the remaining non-whitespace characters to `\S`.
    #[pyo3(name = "with_conversion_of_digits")]
    fn py_with_conversion_of_digits(mut self_: PyRefMut<Self>) -> PyRefMut<Self> {
        self_.config.is_digit_converted = true;
        self_
    }

    /// Convert any character which is not a Unicode decimal digit to character class `\D`.
    ///
    /// This method takes precedence over `with_conversion_of_non_words` if both are set.
    /// Non-digits which are also non-word characters are converted to `\D`.
    ///
    /// This method takes precedence over `with_conversion_of_non_whitespace` if both are set.
    /// Non-digits which are also non-space characters are converted to `\D`.
    #[pyo3(name = "with_conversion_of_non_digits")]
    fn py_with_conversion_of_non_digits(mut self_: PyRefMut<Self>) -> PyRefMut<Self> {
        self_.config.is_non_digit_converted = true;
        self_
    }

    /// Convert any Unicode whitespace character to character class `\s`.
    ///
    /// This method takes precedence over `with_conversion_of_non_digits` if both are set.
    /// Whitespace characters are converted to `\s`, the remaining non-digit characters to `\D`.
    ///
    /// This method takes precedence over `with_conversion_of_non_words` if both are set.
    /// Whitespace characters are converted to `\s`, the remaining non-word characters to `\W`.
    #[pyo3(name = "with_conversion_of_whitespace")]
    fn py_with_conversion_of_whitespace(mut self_: PyRefMut<Self>) -> PyRefMut<Self> {
        self_.config.is_space_converted = true;
        self_
    }

    /// Convert any character which is not a Unicode whitespace character to character class `\S`.
    #[pyo3(name = "with_conversion_of_non_whitespace")]
    fn py_with_conversion_of_non_whitespace(mut self_: PyRefMut<Self>) -> PyRefMut<Self> {
        self_.config.is_non_space_converted = true;
        self_
    }

    /// Convert any Unicode word character to character class `\w`.
    ///
    /// This method takes precedence over `with_conversion_of_non_digits` if both are set.
    /// Word characters are converted to `\w`, the remaining non-digit characters to `\D`.
    ///
    /// This method takes precedence over `with_conversion_of_non_whitespace` if both are set.
    /// Word characters are converted to `\w`, the remaining non-space characters to `\S`.
    #[pyo3(name = "with_conversion_of_words")]
    fn py_with_conversion_of_words(mut self_: PyRefMut<Self>) -> PyRefMut<Self> {
        self_.config.is_word_converted = true;
        self_
    }

    /// Convert any character which is not a Unicode word character to character class `\W`.
    ///
    /// This method takes precedence over `with_conversion_of_non_whitespace` if both are set.
    /// Non-words which are also non-space characters are converted to `\W`.
    #[pyo3(name = "with_conversion_of_non_words")]
    fn py_with_conversion_of_non_words(mut self_: PyRefMut<Self>) -> PyRefMut<Self> {
        self_.config.is_non_word_converted = true;
        self_
    }

    /// Detect repeated non-overlapping substrings and convert them to `{min,max}` quantifier notation.
    #[pyo3(name = "with_conversion_of_repetitions")]
    fn py_with_conversion_of_repetitions(mut self_: PyRefMut<Self>) -> PyRefMut<Self> {
        self_.config.is_repetition_converted = true;
        self_
    }

    /// Enable case-insensitive matching of test cases so that letters match both upper and lower case.
    #[pyo3(name = "with_case_insensitive_matching")]
    fn py_with_case_insensitive_matching(mut self_: PyRefMut<Self>) -> PyRefMut<Self> {
        self_.config.is_case_insensitive_matching = true;
        self_
    }

    /// Replace non-capturing groups by capturing ones.
    #[pyo3(name = "with_capturing_groups")]
    fn py_with_capturing_groups(mut self_: PyRefMut<Self>) -> PyRefMut<Self> {
        self_.config.is_capturing_group_enabled = true;
        self_
    }

    /// Specify the minimum quantity of substring repetitions to be converted if `with_conversion_of_repetitions` is set.
    ///
    /// If the quantity is not explicitly set with this method, a default value of 1 will be used.
    ///
    /// Args:
    ///     quantity (int): The minimum quantity of substring repetitions
    ///
    /// Raises:
    ///     ValueError: if `quantity` is zero
    #[pyo3(name = "with_minimum_repetitions")]
    fn py_with_minimum_repetitions(
        mut self_: PyRefMut<Self>,
        quantity: i32,
    ) -> PyResult<PyRefMut<Self>> {
        if quantity <= 0 {
            Err(PyValueError::new_err(MINIMUM_REPETITIONS_MESSAGE))
        } else {
            self_.config.minimum_repetitions = quantity as u32;
            Ok(self_)
        }
    }

    /// Specify the minimum length a repeated substring must have in order to be converted if `with_conversion_of_repetitions` is set.
    ///
    /// If the length is not explicitly set with this method, a default value of 1 will be used.
    ///
    /// Args:
    ///     length (int): The minimum substring length
    ///
    /// Raises:
    ///     ValueError: if `length` is zero
    #[pyo3(name = "with_minimum_substring_length")]
    fn py_with_minimum_substring_length(
        mut self_: PyRefMut<Self>,
        length: i32,
    ) -> PyResult<PyRefMut<Self>> {
        if length <= 0 {
            Err(PyValueError::new_err(MINIMUM_SUBSTRING_LENGTH_MESSAGE))
        } else {
            self_.config.minimum_substring_length = length as u32;
            Ok(self_)
        }
    }

    /// Convert non-ASCII characters to unicode escape sequences.
    ///
    /// The parameter `use_surrogate_pairs` specifies whether to convert astral code planes
    /// (range `U+010000` to `U+10FFFF`) to surrogate pairs.
    ///
    /// Args:
    ///     use_surrogate_pairs (bool): Whether to convert astral code planes to surrogate pairs
    #[pyo3(name = "with_escaping_of_non_ascii_chars")]
    fn py_with_escaping_of_non_ascii_chars(
        mut self_: PyRefMut<Self>,
        use_surrogate_pairs: bool,
    ) -> PyRefMut<Self> {
        self_.config.is_non_ascii_char_escaped = true;
        self_.config.is_astral_code_point_converted_to_surrogate = use_surrogate_pairs;
        self_
    }

    /// Produce a nicer looking regular expression in verbose mode.
    #[pyo3(name = "with_verbose_mode")]
    fn py_with_verbose_mode(mut self_: PyRefMut<Self>) -> PyRefMut<Self> {
        self_.config.is_verbose_mode_enabled = true;
        self_
    }

    /// Remove the caret anchor '^' from the resulting regular expression, thereby allowing to
    /// match the test cases also when they do not occur at the start of a string.
    #[pyo3(name = "without_start_anchor")]
    fn py_without_start_anchor(mut self_: PyRefMut<Self>) -> PyRefMut<Self> {
        self_.config.is_start_anchor_disabled = true;
        self_
    }

    /// Remove the dollar sign anchor '$' from the resulting regular expression, thereby allowing
    /// to match the test cases also when they do not occur at the end of a string.
    #[pyo3(name = "without_end_anchor")]
    fn py_without_end_anchor(mut self_: PyRefMut<Self>) -> PyRefMut<Self> {
        self_.config.is_end_anchor_disabled = true;
        self_
    }

    /// Remove the caret and dollar sign anchors from the resulting regular expression, thereby
    /// allowing to match the test cases also when they occur within a larger string that contains
    /// other content as well.
    #[pyo3(name = "without_anchors")]
    fn py_without_anchors(mut self_: PyRefMut<Self>) -> PyRefMut<Self> {
        self_.config.is_start_anchor_disabled = true;
        self_.config.is_end_anchor_disabled = true;
        self_
    }

    /// Build the actual regular expression using the previously given settings.
    #[pyo3(name = "build")]
    fn py_build(&mut self) -> String {
        let regexp = self.build();
        if self.config.is_non_ascii_char_escaped {
            replace_unicode_escape_sequences(regexp)
        } else {
            regexp
        }
    }
}

/// Replaces Rust Unicode escape sequences to Python Unicode escape sequences.
fn replace_unicode_escape_sequences(regexp: String) -> String {
    lazy_static! {
        static ref FOUR_CHARS_ESCAPE_SEQUENCE: Regex = Regex::new(r"\\u\{([0-9a-f]{4})\}").unwrap();
        static ref FIVE_CHARS_ESCAPE_SEQUENCE: Regex = Regex::new(r"\\u\{([0-9a-f]{5})\}").unwrap();
    }
    let mut replacement = FOUR_CHARS_ESCAPE_SEQUENCE
        .replace_all(&regexp, |caps: &Captures| format!("\\u{}", &caps[1]))
        .to_string();

    replacement = FIVE_CHARS_ESCAPE_SEQUENCE
        .replace_all(&replacement, |caps: &Captures| {
            format!("\\U000{}", &caps[1])
        })
        .to_string();

    replacement
}
