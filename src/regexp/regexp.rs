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

use crate::ast::Expression;
use crate::char::GraphemeCluster;
use crate::fsm::Dfa;
use crate::regexp::config::RegExpConfig;
use crate::regexp::Component;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter, Result};

pub struct RegExp {
    ast: Expression,
    config: RegExpConfig,
}

impl RegExp {
    pub(crate) fn from(test_cases: &mut Vec<String>, config: &RegExpConfig) -> Self {
        if config.is_case_insensitive_matching() {
            Self::convert_to_lowercase(test_cases);
        }
        Self::sort(test_cases);
        let grapheme_clusters = Self::grapheme_clusters(test_cases, config);
        let mut dfa = Dfa::from(&grapheme_clusters, true, config);
        let mut ast = Expression::from(dfa, config);

        if config.is_start_anchor_disabled
            && config.is_end_anchor_disabled
            && !Self::is_each_test_case_matched(&mut ast, test_cases, config)
        {
            dfa = Dfa::from(&grapheme_clusters, false, config);
            ast = Expression::from(dfa, config);
        }

        Self {
            ast,
            config: config.clone(),
        }
    }

    fn convert_to_lowercase(test_cases: &mut Vec<String>) {
        *test_cases = test_cases.iter().map(|it| it.to_lowercase()).collect_vec();
    }

    fn sort(test_cases: &mut Vec<String>) {
        test_cases.sort();
        test_cases.dedup();
        test_cases.sort_by(|a, b| match a.len().cmp(&b.len()) {
            Ordering::Equal => a.cmp(b),
            other => other,
        });
    }

    fn grapheme_clusters(test_cases: &[String], config: &RegExpConfig) -> Vec<GraphemeCluster> {
        let mut clusters = test_cases
            .iter()
            .map(|it| GraphemeCluster::from(it, config))
            .collect_vec();

        if config.is_char_class_feature_enabled() {
            for cluster in clusters.iter_mut() {
                cluster.convert_to_char_classes();
            }
        }

        if config.is_repetition_converted() {
            for cluster in clusters.iter_mut() {
                cluster.convert_repetitions();
            }
        }

        clusters
    }

    fn is_each_test_case_matched(
        expr: &mut Expression,
        test_cases: &[String],
        config: &RegExpConfig,
    ) -> bool {
        let regex = if config.is_output_colorized {
            let color_replace_regex = Regex::new("\u{1b}\\[(?:\\d+;\\d+|0)m").unwrap();
            Regex::new(&*color_replace_regex.replace_all(&expr.to_string(), "")).unwrap()
        } else {
            Regex::new(&expr.to_string()).unwrap()
        };

        for _ in 1..test_cases.len() {
            if test_cases
                .iter()
                .all(|test_case| regex.find_iter(test_case).count() == 1)
            {
                return true;
            } else if let Expression::Alternation(options, _) = expr {
                options.rotate_right(1);
            } else if let Expression::Concatenation(first, second, _) = expr {
                let a: &mut Expression = first;
                let b: &mut Expression = second;

                if let Expression::Alternation(options, _) = a {
                    options.rotate_right(1);
                } else if let Expression::Alternation(options, _) = b {
                    options.rotate_right(1);
                }
            }
        }
        false
    }
}

impl Display for RegExp {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let ignore_case_flag = if self.config.is_case_insensitive_matching() {
            Component::IgnoreCaseFlag.to_repr(self.config.is_output_colorized)
        } else {
            String::new()
        };
        let caret = if self.config.is_start_anchor_disabled {
            String::new()
        } else {
            Component::Caret.to_repr(self.config.is_output_colorized)
        };
        let dollar_sign = if self.config.is_end_anchor_disabled {
            String::new()
        } else {
            Component::DollarSign.to_repr(self.config.is_output_colorized)
        };
        let mut regexp = match self.ast {
            Expression::Alternation(_, _) => {
                format!(
                    "{}{}{}{}",
                    ignore_case_flag,
                    caret,
                    if self.config.is_capturing_group_enabled() {
                        Component::CapturedParenthesizedExpression(self.ast.to_string())
                            .to_repr(self.config.is_output_colorized)
                    } else {
                        Component::UncapturedParenthesizedExpression(self.ast.to_string())
                            .to_repr(self.config.is_output_colorized)
                    },
                    dollar_sign
                )
            }
            _ => {
                format!(
                    "{}{}{}{}",
                    ignore_case_flag,
                    caret,
                    self.ast,
                    dollar_sign
                )
            }
        };

        if regexp.contains('\u{b}') {
            regexp = regexp.replace('\u{b}', "\\v"); // U+000B Line Tabulation
        }

        write!(
            f,
            "{}",
            if self.config.is_verbose_mode_enabled {
                apply_verbose_mode(regexp, &self.config)
            } else {
                regexp
            }
        )
    }
}

fn apply_verbose_mode(regexp: String, config: &RegExpConfig) -> String {
    lazy_static! {
        static ref ASTERISK: String = Component::Asterisk.to_colored_string(true);
        static ref DIGIT: String = Component::CharClass("\\d".to_string()).to_colored_string(true);
        static ref HYPHEN: String = Component::Hyphen.to_colored_string(true);
        static ref LEFT_BRACKET: String = Component::LeftBracket.to_colored_string(true);
        static ref NON_DIGIT: String = Component::CharClass("\\D".to_string()).to_colored_string(true);
        static ref NON_SPACE: String = Component::CharClass("\\S".to_string()).to_colored_string(true);
        static ref NON_WORD: String = Component::CharClass("\\W".to_string()).to_colored_string(true);
        static ref QUESTION_MARK: String = Component::QuestionMark.to_colored_string(true);
        static ref REPETITION: String = Component::Repetition(0).to_colored_string(true);
        static ref REPETITION_RANGE: String =
            Component::RepetitionRange(0, 0).to_colored_string(true);
        static ref RIGHT_BRACKET: String = Component::RightBracket.to_colored_string(true);
        static ref RIGHT_PARENTHESIS: String = Component::RightParenthesis.to_colored_string(true);
        static ref SPACE: String = Component::CharClass("\\s".to_string()).to_colored_string(true);
        static ref WORD: String = Component::CharClass("\\w".to_string()).to_colored_string(true);
        static ref FIRST_INDENT_REVERSAL: Regex = Regex::new(&format!(
            "(?P<component1>{}|{}|{}|{}|{}|{}|{}|[^\u{1b}\\[0m]+)\n\\s+(?P<component2>{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{})",
            *DIGIT,
            *NON_DIGIT,
            *NON_SPACE,
            *NON_WORD,
            *RIGHT_PARENTHESIS,
            *SPACE,
            *WORD,
            *ASTERISK,
            *DIGIT,
            *HYPHEN,
            *LEFT_BRACKET,
            *NON_DIGIT,
            *NON_SPACE,
            *NON_WORD,
            *QUESTION_MARK,
            *REPETITION,
            *REPETITION_RANGE,
            *RIGHT_BRACKET,
            *SPACE,
            *WORD
        ))
        .unwrap();
        static ref SECOND_INDENT_REVERSAL: Regex = Regex::new(&format!(
            "(?P<component>{}|{})\n\\s+",
            *HYPHEN, *LEFT_BRACKET
        ))
        .unwrap();
        static ref THIRD_INDENT_REVERSAL: Regex = Regex::new(&format!(
            "(?P<component1>[^\u{1b}\\[0m]+(?:{}|{}|{}))\n\\s+(?P<component2>[^\u{1b}\\s]+)",
            *REPETITION, *REPETITION_RANGE, *RIGHT_BRACKET
        ))
        .unwrap();
        static ref FOURTH_INDENT_REVERSAL: Regex = Regex::new(&format!(
            "(?P<component1>(?:{}|{}|{}|{}|{}|{}))\n\\s+(?P<component2>[^\u{1b}\\s]+|{}|{}|{}|{}|{}|{}|{}|{})",
            *DIGIT,
            *NON_DIGIT,
            *NON_SPACE,
            *NON_WORD,
            *SPACE,
            *WORD,
            *DIGIT,
            *NON_DIGIT,
            *NON_SPACE,
            *NON_WORD,
            *REPETITION,
            *REPETITION_RANGE,
            *SPACE,
            *WORD
        ))
        .unwrap();
        static ref FIFTH_INDENT_REVERSAL: Regex =
            Regex::new(r"(?P<component1>\[[^\]]+\])\n\s+(?P<component2>[^\)\s]+)").unwrap();
        static ref COLOR_MODE_REGEX: Regex =
            Regex::new(r"\u{1b}\[\d+;\d+m[^\u{1b}]+\u{1b}\[0m|[^\u{1b}]+").unwrap();
        static ref VERBOSE_MODE_REGEX: Regex = Regex::new(
            r#"(?x)
            \(\?i\)
            |
            \[[^\]]+\]
            |
            \( (?: \?: )?
            |
            \) (?: \? | \{ \d+ (?: ,\d+ )? \} )?   
            |   
            [\^|$]
            |
            (?:
                (?: \\[\^$()|DdSsWw\\\ ] )+
                (?: \\* [^\^$|()\\] )*
            )+
            |
            (?:
                (?: \\* [^\^$()|\\] )+
                (?: \\[\^$()|DdSsWw\\\ ] )*
            )+
            "#
        )
        .unwrap();
    }

    let verbose_mode_flag = if config.is_case_insensitive_matching() {
        Component::IgnoreCaseAndVerboseModeFlag.to_repr(config.is_output_colorized)
    } else {
        Component::VerboseModeFlag.to_repr(config.is_output_colorized)
    };

    let mut verbose_regexp = vec![verbose_mode_flag];
    let mut nesting_level = if config.is_start_anchor_disabled {
        1
    } else {
        0
    };

    let regexp_with_replacements = regexp
        .replace(
            &Component::IgnoreCaseFlag.to_repr(config.is_output_colorized),
            "",
        )
        .replace('#', "\\#")
        .replace(' ', "\\s")
        .replace(' ', "\\s")
        .replace(' ', "\\s")
        .replace(' ', "\\s")
        .replace(' ', "\\s")
        .replace(' ', "\\s")
        .replace(' ', "\\s")
        .replace('\u{85}', "\\s")
        .replace('\u{a0}', "\\s")
        .replace('\u{1680}', "\\s")
        .replace('\u{2000}', "\\s")
        .replace('\u{2001}', "\\s")
        .replace('\u{2002}', "\\s")
        .replace('\u{2003}', "\\s")
        .replace('\u{2004}', "\\s")
        .replace('\u{2005}', "\\s")
        .replace('\u{2006}', "\\s")
        .replace('\u{2007}', "\\s")
        .replace('\u{2008}', "\\s")
        .replace('\u{2009}', "\\s")
        .replace('\u{200a}', "\\s")
        .replace('\u{200b}', "\\s")
        .replace('\u{2028}', "\\s")
        .replace('\u{2029}', "\\s")
        .replace('\u{202f}', "\\s")
        .replace('\u{205f}', "\\s")
        .replace('\u{3000}', "\\s")
        .replace(' ', "\\ ");

    if config.is_output_colorized {
        for regexp_match in COLOR_MODE_REGEX.find_iter(&regexp_with_replacements) {
            let element = regexp_match.as_str();
            if element.is_empty() {
                continue;
            }

            let is_colored_element = element.starts_with("\u{1b}[");
            if is_colored_element && (element.contains('$') || element.contains(')')) {
                nesting_level -= 1;
            }

            let indentation = "  ".repeat(nesting_level);
            verbose_regexp.push(format!("{}{}", indentation, element));

            if is_colored_element && (element.contains('^') || element.contains('(')) {
                nesting_level += 1;
            }
        }

        let joined_regexp = verbose_regexp.join("\n");
        let mut joined_regexp_with_replacements = FIRST_INDENT_REVERSAL
            .replace_all(&joined_regexp, "$component1$component2")
            .to_string();

        joined_regexp_with_replacements = SECOND_INDENT_REVERSAL
            .replace_all(&joined_regexp_with_replacements, "$component")
            .to_string();

        joined_regexp_with_replacements = THIRD_INDENT_REVERSAL
            .replace_all(&joined_regexp_with_replacements, "$component1$component2")
            .to_string();

        joined_regexp_with_replacements = FOURTH_INDENT_REVERSAL
            .replace_all(&joined_regexp_with_replacements, "$component1$component2")
            .to_string();

        joined_regexp_with_replacements
    } else {
        for regexp_match in VERBOSE_MODE_REGEX.find_iter(&regexp_with_replacements) {
            let element = regexp_match.as_str();
            if element.is_empty() {
                continue;
            }
            if element == "$" || element.starts_with(')') {
                nesting_level -= 1;
            }
            let indentation = "  ".repeat(nesting_level);
            verbose_regexp.push(format!("{}{}", indentation, element));

            if element == "^" || element.starts_with('(') {
                nesting_level += 1;
            }
        }

        let joined_regexp = verbose_regexp.join("\n");

        let joined_regexp_with_replacements = FIFTH_INDENT_REVERSAL
            .replace_all(&joined_regexp, "$component1$component2")
            .to_string();

        joined_regexp_with_replacements
    }
}
