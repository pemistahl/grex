/*
 * Copyright © 2019 Peter M. Stahl pemistahl@gmail.com
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

use itertools::Itertools;

pub(crate) fn escape_non_ascii_chars(regex: &str, use_surrogate_pairs: bool) -> String {
    let surrogate_range = '\u{10000}'..'\u{10ffff}';
    regex
        .chars()
        .map(|it| {
            if it.is_ascii() {
                it.to_string()
            } else if use_surrogate_pairs && surrogate_range.contains(&it) {
                convert_to_surrogate_pair(it)
            } else {
                it.escape_unicode().to_string()
            }
        })
        .join("")
}

fn convert_to_surrogate_pair(c: char) -> String {
    c.encode_utf16(&mut [0; 2])
        .iter()
        .map(|it| format!("\\u{{{:x}}}", it))
        .join("")
}

#[cfg(test)]
mod tests {
    use super::*;
    use maplit::hashmap;

    #[test]
    fn test_escaping_non_ascii_chars() {
        let input = "My ♥ and 💩 is yours.";
        assert_eq!(
            escape_non_ascii_chars(input, false),
            "My \\u{2665} and \\u{1f4a9} is yours."
        );
        assert_eq!(
            escape_non_ascii_chars(input, true),
            "My \\u{2665} and \\u{d83d}\\u{dca9} is yours."
        );
    }

    #[test]
    fn test_surrogate_pair_conversion() {
        let params = hashmap![
            '💩' => "\\u{d83d}\\u{dca9}",
            '\u{10000}' => "\\u{d800}\\u{dc00}",
            '\u{12345}' => "\\u{d808}\\u{df45}",
            '\u{10FFFF}' => "\\u{dbff}\\u{dfff}"
        ];

        for (c, expected_surrogate_pair) in params {
            assert_eq!(convert_to_surrogate_pair(c), expected_surrogate_pair);
        }
    }
}
