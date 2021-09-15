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

#![allow(deprecated)]

mod builder;
mod component;
mod config;
mod feature;

#[allow(clippy::module_inception)]
mod regexp;

pub use builder::RegExpBuilder;
pub use component::Component;
pub use config::RegExpConfig;
pub use feature::Feature;
pub use regexp::RegExp;

#[cfg(test)]
mod tests {
    use crate::regexp::Feature;
    use crate::regexp::RegExpBuilder;

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
    #[should_panic(expected = "Quantity of minimum repetitions must not be zero")]
    fn regexp_builder_panics_if_minimum_repetitions_is_less_than_two() {
        RegExpBuilder::from(&["abc"]).with_minimum_repetitions(0);
    }

    #[test]
    #[should_panic(expected = "Minimum substring length must not be zero")]
    fn regexp_builder_panics_if_minimum_substring_length_is_zero() {
        RegExpBuilder::from(&["abc"]).with_minimum_substring_length(0);
    }
}
