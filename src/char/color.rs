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
use std::fmt::{Display, Formatter, Result};

pub enum ColorizableString {
    Asterisk,
    CapturingLeftParenthesis,
    Caret,
    Comma,
    DigitCharClass,
    DollarSign,
    EmptyString,
    Hyphen,
    IgnoreCaseFlag,
    LeftBrace,
    LeftBracket,
    NonCapturingLeftParenthesis,
    NonDigitCharClass,
    NonSpaceCharClass,
    NonWordCharClass,
    Number(u32),
    Other(String),
    Pipe,
    QuestionMark,
    RightBrace,
    RightBracket,
    RightParenthesis,
    SpaceCharClass,
    WordCharClass,
}

impl ColorizableString {
    pub fn from(value: &str) -> Self {
        match value {
            "\\d" => ColorizableString::DigitCharClass,
            "\\s" => ColorizableString::SpaceCharClass,
            "\\w" => ColorizableString::WordCharClass,
            "\\D" => ColorizableString::NonDigitCharClass,
            "\\S" => ColorizableString::NonSpaceCharClass,
            "\\W" => ColorizableString::NonWordCharClass,
            _ => ColorizableString::Other(value.to_string()),
        }
    }

    pub fn to_colorized_string(&self, is_output_colorized: bool) -> ColoredString {
        let string_repr = self.to_string();
        let repr = string_repr.as_str();

        if !is_output_colorized {
            return repr.clear();
        }

        match self {
            ColorizableString::IgnoreCaseFlag => repr.bright_yellow().on_black(),
            ColorizableString::Pipe => repr.red().bold(),
            ColorizableString::Asterisk | ColorizableString::QuestionMark => repr.purple().bold(),
            ColorizableString::Caret | ColorizableString::DollarSign => repr.yellow().bold(),
            ColorizableString::EmptyString | ColorizableString::Other(_) => repr.clear(),

            ColorizableString::NonCapturingLeftParenthesis
            | ColorizableString::CapturingLeftParenthesis
            | ColorizableString::RightParenthesis => repr.green().bold(),

            ColorizableString::Number(_)
            | ColorizableString::LeftBrace
            | ColorizableString::RightBrace
            | ColorizableString::Comma => repr.white().on_bright_blue(),

            ColorizableString::LeftBracket
            | ColorizableString::RightBracket
            | ColorizableString::Hyphen => repr.cyan().bold(),

            ColorizableString::DigitCharClass
            | ColorizableString::SpaceCharClass
            | ColorizableString::WordCharClass
            | ColorizableString::NonDigitCharClass
            | ColorizableString::NonSpaceCharClass
            | ColorizableString::NonWordCharClass => repr.black().on_bright_yellow(),
        }
    }
}

impl Display for ColorizableString {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}",
            match self {
                ColorizableString::Asterisk => "*".to_string(),
                ColorizableString::CapturingLeftParenthesis => "(".to_string(),
                ColorizableString::Caret => "^".to_string(),
                ColorizableString::Comma => ",".to_string(),
                ColorizableString::DigitCharClass => "\\d".to_string(),
                ColorizableString::DollarSign => "$".to_string(),
                ColorizableString::Hyphen => "-".to_string(),
                ColorizableString::IgnoreCaseFlag => "(?i)".to_string(),
                ColorizableString::LeftBrace => "{".to_string(),
                ColorizableString::LeftBracket => "[".to_string(),
                ColorizableString::EmptyString => "".to_string(),
                ColorizableString::NonCapturingLeftParenthesis => "(?:".to_string(),
                ColorizableString::NonDigitCharClass => "\\D".to_string(),
                ColorizableString::NonSpaceCharClass => "\\S".to_string(),
                ColorizableString::NonWordCharClass => "\\W".to_string(),
                ColorizableString::Number(number) => number.to_string(),
                ColorizableString::Other(value) => value.to_string(),
                ColorizableString::Pipe => "|".to_string(),
                ColorizableString::QuestionMark => "?".to_string(),
                ColorizableString::RightBrace => "}".to_string(),
                ColorizableString::RightBracket => "]".to_string(),
                ColorizableString::RightParenthesis => ")".to_string(),
                ColorizableString::SpaceCharClass => "\\s".to_string(),
                ColorizableString::WordCharClass => "\\w".to_string(),
            }
        )
    }
}
