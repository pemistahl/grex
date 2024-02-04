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

use crate::quantifier::Quantifier;
use std::fmt::{Display, Formatter, Result};

pub(crate) enum Component {
    CapturedLeftParenthesis,
    CapturedParenthesizedExpression(String, bool, bool),
    Caret(bool),
    CharClass(String),
    DollarSign(bool),
    Hyphen,
    IgnoreCaseFlag,
    IgnoreCaseAndVerboseModeFlag,
    LeftBracket,
    Pipe,
    Quantifier(Quantifier, bool),
    Repetition(u32, bool),
    RepetitionRange(u32, u32, bool),
    RightBracket,
    RightParenthesis,
    UncapturedLeftParenthesis,
    UncapturedParenthesizedExpression(String, bool, bool),
    VerboseModeFlag,
}

impl Component {
    pub(crate) fn to_repr(&self, is_output_colorized: bool) -> String {
        match is_output_colorized {
            true => self.to_colored_string(false),
            false => self.to_string(),
        }
    }

    pub(crate) fn to_colored_string(&self, is_escaped: bool) -> String {
        match self {
            Component::CapturedLeftParenthesis => Self::green_bold(&self.to_string(), is_escaped),
            Component::CapturedParenthesizedExpression(
                expr,
                is_verbose_mode_enabled,
                has_final_line_break,
            ) => {
                if *is_verbose_mode_enabled {
                    if *has_final_line_break {
                        format!(
                            "\n{}\n{}\n{}\n",
                            Component::CapturedLeftParenthesis.to_colored_string(is_escaped),
                            expr,
                            Component::RightParenthesis.to_colored_string(is_escaped)
                        )
                    } else {
                        format!(
                            "\n{}\n{}\n{}",
                            Component::CapturedLeftParenthesis.to_colored_string(is_escaped),
                            expr,
                            Component::RightParenthesis.to_colored_string(is_escaped)
                        )
                    }
                } else {
                    format!(
                        "{}{}{}",
                        Component::CapturedLeftParenthesis.to_colored_string(is_escaped),
                        expr,
                        Component::RightParenthesis.to_colored_string(is_escaped)
                    )
                }
            }
            Component::Caret(is_verbose_mode_enabled) => {
                if *is_verbose_mode_enabled {
                    format!(
                        "{}\n",
                        Self::yellow_bold(&Component::Caret(false).to_string(), is_escaped)
                    )
                } else {
                    Self::yellow_bold(&self.to_string(), is_escaped)
                }
            }
            Component::CharClass(value) => Self::black_on_bright_yellow(value, is_escaped),
            Component::DollarSign(is_verbose_mode_enabled) => {
                if *is_verbose_mode_enabled {
                    format!(
                        "\n{}",
                        Self::yellow_bold(&Component::DollarSign(false).to_string(), is_escaped)
                    )
                } else {
                    Self::yellow_bold(&self.to_string(), is_escaped)
                }
            }
            Component::Hyphen => Self::cyan_bold(&self.to_string(), is_escaped),
            Component::IgnoreCaseFlag => {
                Self::bright_yellow_on_black(&self.to_string(), is_escaped)
            }
            Component::IgnoreCaseAndVerboseModeFlag => {
                format!("{}\n", Self::bright_yellow_on_black("(?ix)", is_escaped))
            }
            Component::LeftBracket => Self::cyan_bold(&self.to_string(), is_escaped),
            Component::Pipe => Self::red_bold(&self.to_string(), is_escaped),
            Component::Quantifier(quantifier, is_verbose_mode_enabled) => {
                if *is_verbose_mode_enabled {
                    format!(
                        "{}\n",
                        Self::purple_bold(&quantifier.to_string(), is_escaped)
                    )
                } else {
                    Self::purple_bold(&self.to_string(), is_escaped)
                }
            }
            Component::Repetition(num, is_verbose_mode_enabled) => {
                if *is_verbose_mode_enabled {
                    format!(
                        "{}\n",
                        Self::white_on_bright_blue(
                            &Component::Repetition(*num, false).to_string(),
                            is_escaped
                        )
                    )
                } else {
                    Self::white_on_bright_blue(&self.to_string(), is_escaped)
                }
            }
            Component::RepetitionRange(min, max, is_verbose_mode_enabled) => {
                if *is_verbose_mode_enabled {
                    format!(
                        "{}\n",
                        Self::white_on_bright_blue(
                            &Component::RepetitionRange(*min, *max, false).to_string(),
                            is_escaped
                        )
                    )
                } else {
                    Self::white_on_bright_blue(&self.to_string(), is_escaped)
                }
            }
            Component::RightBracket => Self::cyan_bold(&self.to_string(), is_escaped),
            Component::RightParenthesis => Self::green_bold(&self.to_string(), is_escaped),
            Component::UncapturedLeftParenthesis => Self::green_bold(&self.to_string(), is_escaped),
            Component::UncapturedParenthesizedExpression(
                expr,
                is_verbose_mode_enabled,
                has_final_line_break,
            ) => {
                if *is_verbose_mode_enabled {
                    if *has_final_line_break {
                        format!(
                            "\n{}\n{}\n{}\n",
                            Component::UncapturedLeftParenthesis.to_colored_string(is_escaped),
                            expr,
                            Component::RightParenthesis.to_colored_string(is_escaped)
                        )
                    } else {
                        format!(
                            "\n{}\n{}\n{}",
                            Component::UncapturedLeftParenthesis.to_colored_string(is_escaped),
                            expr,
                            Component::RightParenthesis.to_colored_string(is_escaped)
                        )
                    }
                } else {
                    format!(
                        "{}{}{}",
                        Component::UncapturedLeftParenthesis.to_colored_string(is_escaped),
                        expr,
                        Component::RightParenthesis.to_colored_string(is_escaped)
                    )
                }
            }
            Component::VerboseModeFlag => {
                format!("{}\n", Self::bright_yellow_on_black("(?x)", is_escaped))
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
                Component::CapturedLeftParenthesis => "(".to_string(),
                Component::CapturedParenthesizedExpression(
                    expr,
                    is_verbose_mode_enabled,
                    has_final_line_break,
                ) =>
                    if *is_verbose_mode_enabled {
                        if *has_final_line_break {
                            format!(
                                "\n{}\n{}\n{}\n",
                                Component::CapturedLeftParenthesis,
                                expr,
                                Component::RightParenthesis
                            )
                        } else {
                            format!(
                                "\n{}\n{}\n{}",
                                Component::CapturedLeftParenthesis,
                                expr,
                                Component::RightParenthesis
                            )
                        }
                    } else {
                        format!(
                            "{}{}{}",
                            Component::CapturedLeftParenthesis,
                            expr,
                            Component::RightParenthesis
                        )
                    },
                Component::Caret(is_verbose_mode_enabled) =>
                    if *is_verbose_mode_enabled {
                        "^\n".to_string()
                    } else {
                        "^".to_string()
                    },
                Component::CharClass(value) => value.clone(),
                Component::DollarSign(is_verbose_mode_enabled) =>
                    if *is_verbose_mode_enabled {
                        "\n$".to_string()
                    } else {
                        "$".to_string()
                    },
                Component::Hyphen => "-".to_string(),
                Component::IgnoreCaseFlag => "(?i)".to_string(),
                Component::IgnoreCaseAndVerboseModeFlag => "(?ix)\n".to_string(),
                Component::LeftBracket => "[".to_string(),
                Component::Pipe => "|".to_string(),
                Component::Quantifier(quantifier, is_verbose_mode_enabled) =>
                    if *is_verbose_mode_enabled {
                        format!("{}\n", quantifier)
                    } else {
                        quantifier.to_string()
                    },
                Component::Repetition(num, is_verbose_mode_enabled) => {
                    if *num == 0 && *is_verbose_mode_enabled {
                        "{\\d+\\}\n".to_string()
                    } else if *num == 0 {
                        "{\\d+\\}".to_string()
                    } else if *is_verbose_mode_enabled {
                        format!("{{{}}}\n", num)
                    } else {
                        format!("{{{}}}", num)
                    }
                }
                Component::RepetitionRange(min, max, is_verbose_mode_enabled) => {
                    if *min == 0 && *max == 0 && *is_verbose_mode_enabled {
                        "{\\d+,\\d+\\}\n".to_string()
                    } else if *min == 0 && *max == 0 {
                        "{\\d+,\\d+\\}".to_string()
                    } else if *is_verbose_mode_enabled {
                        format!("{{{},{}}}\n", min, max)
                    } else {
                        format!("{{{},{}}}", min, max)
                    }
                }
                Component::RightBracket => "]".to_string(),
                Component::RightParenthesis => ")".to_string(),
                Component::UncapturedLeftParenthesis => "(?:".to_string(),
                Component::UncapturedParenthesizedExpression(
                    expr,
                    is_verbose_mode_enabled,
                    has_final_line_break,
                ) => {
                    if *is_verbose_mode_enabled {
                        if *has_final_line_break {
                            format!(
                                "\n{}\n{}\n{}\n",
                                Component::UncapturedLeftParenthesis,
                                expr,
                                Component::RightParenthesis
                            )
                        } else {
                            format!(
                                "\n{}\n{}\n{}",
                                Component::UncapturedLeftParenthesis,
                                expr,
                                Component::RightParenthesis
                            )
                        }
                    } else {
                        format!(
                            "{}{}{}",
                            Component::UncapturedLeftParenthesis,
                            expr,
                            Component::RightParenthesis
                        )
                    }
                }
                Component::VerboseModeFlag => "(?x)\n".to_string(),
            }
        )
    }
}
