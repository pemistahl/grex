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

#[derive(Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct RegExpConfig {
    pub(crate) minimum_repetitions: u32,
    pub(crate) minimum_substring_length: u32,
    pub(crate) is_digit_converted: bool,
    pub(crate) is_non_digit_converted: bool,
    pub(crate) is_space_converted: bool,
    pub(crate) is_non_space_converted: bool,
    pub(crate) is_word_converted: bool,
    pub(crate) is_non_word_converted: bool,
    pub(crate) is_repetition_converted: bool,
    pub(crate) is_case_insensitive_matching: bool,
    pub(crate) is_capturing_group_enabled: bool,
    pub(crate) is_non_ascii_char_escaped: bool,
    pub(crate) is_astral_code_point_converted_to_surrogate: bool,
    pub(crate) is_verbose_mode_enabled: bool,
    pub(crate) is_start_anchor_disabled: bool,
    pub(crate) is_end_anchor_disabled: bool,
    pub(crate) is_output_colorized: bool,
}

impl RegExpConfig {
    pub(crate) fn new() -> Self {
        Self {
            minimum_repetitions: 1,
            minimum_substring_length: 1,
            is_digit_converted: false,
            is_non_digit_converted: false,
            is_space_converted: false,
            is_non_space_converted: false,
            is_word_converted: false,
            is_non_word_converted: false,
            is_repetition_converted: false,
            is_case_insensitive_matching: false,
            is_capturing_group_enabled: false,
            is_non_ascii_char_escaped: false,
            is_astral_code_point_converted_to_surrogate: false,
            is_verbose_mode_enabled: false,
            is_start_anchor_disabled: false,
            is_end_anchor_disabled: false,
            is_output_colorized: false,
        }
    }

    pub(crate) fn is_char_class_feature_enabled(&self) -> bool {
        self.is_digit_converted
            || self.is_non_digit_converted
            || self.is_space_converted
            || self.is_non_space_converted
            || self.is_word_converted
            || self.is_non_word_converted
            || self.is_case_insensitive_matching
            || self.is_capturing_group_enabled
    }
}
