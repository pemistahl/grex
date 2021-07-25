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

use crate::regexp::Feature;

#[derive(Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct RegExpConfig {
    pub(crate) conversion_features: Vec<Feature>,
    pub(crate) minimum_repetitions: u32,
    pub(crate) minimum_substring_length: u32,
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
            conversion_features: vec![],
            minimum_repetitions: 1,
            minimum_substring_length: 1,
            is_non_ascii_char_escaped: false,
            is_astral_code_point_converted_to_surrogate: false,
            is_verbose_mode_enabled: false,
            is_start_anchor_disabled: false,
            is_end_anchor_disabled: false,
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
