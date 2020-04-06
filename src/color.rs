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

use colored::{ColoredString, Colorize};
use itertools::Itertools;

pub(crate) fn colorize(characters: Vec<&str>, is_output_colorized: bool) -> Vec<ColoredString> {
    characters
        .iter()
        .map(|&it| {
            if !is_output_colorized {
                return it.clear();
            }
            for c in it.chars() {
                if c.is_ascii_digit() {
                    return it.white().on_bright_blue();
                }
            }
            match it {
                "(?i)" => it.bright_yellow().on_black(),
                "(?:" | "(" | ")" => it.green().bold(),
                "{" | "}" | "," => it.white().on_bright_blue(),
                "[" | "]" | "-" => it.cyan().bold(),
                "^" | "$" => it.yellow().bold(),
                "*" | "?" => it.purple().bold(),
                "|" => it.red().bold(),
                "\\d" | "\\w" | "\\s" | "\\D" | "\\W" | "\\S" => it.black().on_bright_yellow(),
                _ => it.clear(),
            }
        })
        .collect_vec()
}
