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

use crate::ast::Quantifier;
use std::fmt::{Display, Formatter, Result};

pub enum Component {
    Asterisk,
    CapturedLeftParenthesis,
    CapturedParenthesizedExpression(String),
    Caret,
    CharClass(String),
    DollarSign,
    Hyphen,
    IgnoreCaseFlag,
    IgnoreCaseAndVerboseModeFlag,
    LeftBracket,
    Pipe,
    Quantifier(Quantifier),
    QuestionMark,
    Repetition(u32),
    RepetitionRange(u32, u32),
    RightBracket,
    RightParenthesis,
    UncapturedLeftParenthesis,
    UncapturedParenthesizedExpression(String),
    VerboseModeFlag,
}

impl Component {
    pub fn to_repr(&self, is_output_colorized: bool) -> String {
        match is_output_colorized {
            true => self.to_colored_string(false),
            false => self.to_string(),
        }
    }

    pub fn to_colored_string(&self, is_escaped: bool) -> String {
        match self {
            Component::Asterisk => Self::purple_bold(&self.to_string(), is_escaped),
            Component::CapturedLeftParenthesis => Self::green_bold(&self.to_string(), is_escaped),
            Component::CapturedParenthesizedExpression(expr) => {
                format!(
                    "{}{}{}",
                    Component::CapturedLeftParenthesis.to_colored_string(is_escaped),
                    expr,
                    Component::RightParenthesis.to_colored_string(is_escaped)
                )
            }
            Component::Caret => Self::yellow_bold(&self.to_string(), is_escaped),
            Component::CharClass(value) => Self::black_on_bright_yellow(value, is_escaped),
            Component::DollarSign => Self::yellow_bold(&self.to_string(), is_escaped),
            Component::Hyphen => Self::cyan_bold(&self.to_string(), is_escaped),
            Component::IgnoreCaseFlag => {
                Self::bright_yellow_on_black(&self.to_string(), is_escaped)
            }
            Component::IgnoreCaseAndVerboseModeFlag => {
                Self::bright_yellow_on_black(&self.to_string(), is_escaped)
            }
            Component::LeftBracket => Self::cyan_bold(&self.to_string(), is_escaped),
            Component::Pipe => Self::red_bold(&self.to_string(), is_escaped),
            Component::Quantifier(_) => Self::purple_bold(&self.to_string(), is_escaped),
            Component::QuestionMark => Self::purple_bold(&self.to_string(), is_escaped),
            Component::Repetition(_) => Self::white_on_bright_blue(&self.to_string(), is_escaped),
            Component::RepetitionRange(_, _) => {
                Self::white_on_bright_blue(&self.to_string(), is_escaped)
            }
            Component::RightBracket => Self::cyan_bold(&self.to_string(), is_escaped),
            Component::RightParenthesis => Self::green_bold(&self.to_string(), is_escaped),
            Component::UncapturedLeftParenthesis => Self::green_bold(&self.to_string(), is_escaped),
            Component::UncapturedParenthesizedExpression(expr) => {
                format!(
                    "{}{}{}",
                    Component::UncapturedLeftParenthesis.to_colored_string(is_escaped),
                    expr,
                    Component::RightParenthesis.to_colored_string(is_escaped)
                )
            }
            Component::VerboseModeFlag => {
                Self::bright_yellow_on_black(&self.to_string(), is_escaped)
            }
        }
    }

    fn black_on_bright_yellow(value: &str, is_escaped: bool) -> String {
        Self::color_code("103;30", value, is_escaped)
    }

    fn bright_yellow_on_black(value: &str, is_escaped: bool) -> String {
        Self::color_code("40;93", value, is_escaped)
    }

    fn cyan_bold(value: &str, is_escaped: bool) -> String {
        Self::color_code("1;36", value, is_escaped)
    }

    fn green_bold(value: &str, is_escaped: bool) -> String {
        Self::color_code("1;32", value, is_escaped)
    }

    fn purple_bold(value: &str, is_escaped: bool) -> String {
        Self::color_code("1;35", value, is_escaped)
    }

    fn red_bold(value: &str, is_escaped: bool) -> String {
        Self::color_code("1;31", value, is_escaped)
    }

    fn white_on_bright_blue(value: &str, is_escaped: bool) -> String {
        Self::color_code("104;37", value, is_escaped)
    }

    fn yellow_bold(value: &str, is_escaped: bool) -> String {
        Self::color_code("1;33", value, is_escaped)
    }

    fn color_code(code: &str, value: &str, is_escaped: bool) -> String {
        if is_escaped {
            format!("\u{1b}\\[{}m\\{}\u{1b}\\[0m", code, value)
        } else {
            format!("\u{1b}[{}m{}\u{1b}[0m", code, value)
        }
    }
}

impl Display for Component {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}",
            match self {
                Component::Asterisk => "*".to_string(),
                Component::CapturedLeftParenthesis => "(".to_string(),
                Component::CapturedParenthesizedExpression(expr) => format!(
                    "{}{}{}",
                    Component::CapturedLeftParenthesis,
                    expr,
                    Component::RightParenthesis
                ),
                Component::Caret => "^".to_string(),
                Component::CharClass(value) => value.clone(),
                Component::DollarSign => "$".to_string(),
                Component::Hyphen => "-".to_string(),
                Component::IgnoreCaseFlag => "(?i)".to_string(),
                Component::IgnoreCaseAndVerboseModeFlag => "(?ix)".to_string(),
                Component::LeftBracket => "[".to_string(),
                Component::Pipe => "|".to_string(),
                Component::Quantifier(quantifier) => quantifier.to_string(),
                Component::QuestionMark => "?".to_string(),
                Component::Repetition(num) =>
                    if *num == 0 {
                        "{\\d+\\}".to_string()
                    } else {
                        format!("{{{}}}", num)
                    },
                Component::RepetitionRange(min, max) =>
                    if *min == 0 && *max == 0 {
                        "{\\d+,\\d+\\}".to_string()
                    } else {
                        format!("{{{},{}}}", min, max)
                    },
                Component::RightBracket => "]".to_string(),
                Component::RightParenthesis => ")".to_string(),
                Component::UncapturedLeftParenthesis => "(?:".to_string(),
                Component::UncapturedParenthesizedExpression(expr) => format!(
                    "{}{}{}",
                    Component::UncapturedLeftParenthesis,
                    expr,
                    Component::RightParenthesis
                ),
                Component::VerboseModeFlag => "(?x)".to_string(),
            }
        )
    }
}
