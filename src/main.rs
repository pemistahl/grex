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

use grex::RegExpBuilder;
use itertools::Itertools;
use std::io::{BufRead, Error, ErrorKind, Read};
use std::path::PathBuf;
use structopt::clap::AppSettings::{AllowLeadingHyphen, ColoredHelp};
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(
    author = "© 2019-today Peter M. Stahl <pemistahl@gmail.com>",
    about = "Licensed under the Apache License, Version 2.0\n\
             Downloadable from https://crates.io/crates/grex\n\
             Source code at https://github.com/pemistahl/grex\n\n\
             grex generates regular expressions from user-provided test cases.",
    version_short = "v",
    global_settings = &[AllowLeadingHyphen, ColoredHelp]
)]
struct Cli {
    // --------------------
    // ARGS
    // --------------------
    #[structopt(
        value_name = "INPUT",
        required_unless = "file",
        conflicts_with = "file",
        help = "One or more test cases separated by blank space"
    )]
    input: Vec<String>,

    // --------------------
    // FLAGS
    // --------------------
    #[structopt(
        name = "digits",
        short,
        long,
        help = "Converts any Unicode decimal digit to \\d",
        long_help = "Converts any Unicode decimal digit to \\d.\n\n\
                     Takes precedence over --words if both are set.\n\
                     Decimal digits are converted to \\d, remaining word characters to \\w.\n\n\
                     Takes precedence over --non-spaces if both are set.\n\
                     Decimal digits are converted to \\d, remaining non-space characters to \\S.",
        display_order = 1
    )]
    is_digit_converted: bool,

    #[structopt(
        name = "non-digits",
        short = "D",
        long,
        help = "Converts any character which is not a Unicode decimal digit to \\D",
        long_help = "Converts any character which is not a Unicode decimal digit to \\D.\n\n\
                     Takes precedence over --non-words if both are set.\n\
                     Non-digits which are also non-word characters are converted to \\D.\n\n\
                     Takes precedence over --non-spaces if both are set.\n\
                     Non-digits which are also non-space characters are converted to \\D.",
        display_order = 2
    )]
    is_non_digit_converted: bool,

    #[structopt(
        name = "spaces",
        short,
        long,
        help = "Converts any Unicode whitespace character to \\s",
        long_help = "Converts any Unicode whitespace character to \\s.\n\n\
                     Takes precedence over --non-digits if both are set.\n\
                     Whitespace is converted to \\s, remaining non-digits to \\D.\n\n\
                     Takes precedence over --non-words if both are set.\n\
                     Whitespace is converted to \\s, remaining non-word characters to \\W.",
        display_order = 3
    )]
    is_space_converted: bool,

    #[structopt(
        name = "non-spaces",
        short = "S",
        long,
        help = "Converts any character which is not a Unicode whitespace character to \\S",
        display_order = 4
    )]
    is_non_space_converted: bool,

    #[structopt(
        name = "words",
        short,
        long,
        help = "Converts any Unicode word character to \\w",
        long_help = "Converts any Unicode word character to \\w.\n\n\
                     Takes precedence over --non-digits if both are set.\n\
                     Word characters are converted to \\w, remaining non-digits to \\D.\n\n\
                     Takes precedence over --non-spaces if both are set.\n\
                     Word characters are converted to \\w, remaining non-whitespace to \\S.",
        display_order = 5
    )]
    is_word_converted: bool,

    #[structopt(
        name = "non-words",
        short = "W",
        long,
        help = "Converts any character which is not a Unicode word character to \\W",
        long_help = "Converts any character which is not a Unicode word character to \\W.\n\n\
                     Takes precedence over --non-spaces if both are set.\n\
                     Non-word characters which are also non-whitespace are converted to \\W.",
        display_order = 6
    )]
    is_non_word_converted: bool,

    #[structopt(
        name = "repetitions",
        short,
        long,
        help = "Detects repeated non-overlapping substrings and\n\
                converts them to {min,max} quantifier notation",
        display_order = 7
    )]
    is_repetition_converted: bool,

    #[structopt(
        name = "escape",
        short,
        long,
        help = "Replaces all non-ASCII characters with unicode escape sequences",
        display_order = 8
    )]
    is_non_ascii_char_escaped: bool,

    #[structopt(
        name = "with-surrogates",
        long,
        requires = "escape",
        help = "Converts astral code points to surrogate pairs if --escape is set",
        display_order = 9
    )]
    is_astral_code_point_converted_to_surrogate: bool,

    #[structopt(
        name = "ignore-case",
        short,
        long,
        help = "Performs case-insensitive matching, letters match both upper and lower case",
        display_order = 10
    )]
    is_case_ignored: bool,

    #[structopt(
        name = "capture-groups",
        short = "g",
        long,
        help = "Replaces non-capturing groups by capturing ones",
        display_order = 11
    )]
    is_group_captured: bool,

    #[structopt(
        name = "verbose",
        short = "x",
        long,
        help = "Produces a nicer looking regular expression in verbose mode",
        display_order = 12
    )]
    is_verbose_mode_enabled: bool,

    #[structopt(
        name = "no-start-anchor",
        long,
        help = "Removes the caret anchor '^' from the resulting regular expression",
        long_help = "Removes the caret anchor '^' from the resulting regular expression.\n\n\
                     By default, the caret anchor is added to every generated regular\n\
                     expression which guarantees that the expression matches the test cases\n\
                     given as input only at the start of a string.\n\
                     This flag removes the anchor, thereby allowing to match the test cases also\n\
                     when they do not occur at the start of a string.",
        display_order = 13
    )]
    is_caret_anchor_disabled: bool,

    #[structopt(
        name = "no-end-anchor",
        long,
        help = "Removes the dollar sign anchor '$' from the resulting regular expression",
        long_help = "Removes the dollar sign anchor '$' from the resulting regular expression.\n\n\
                     By default, the dollar sign anchor is added to every generated regular\n\
                     expression which guarantees that the expression matches the test cases\n\
                     given as input only at the end of a string.\n\
                     This flag removes the anchor, thereby allowing to match the test cases also\n\
                     when they do not occur at the end of a string.",
        display_order = 14
    )]
    is_dollar_sign_anchor_disabled: bool,

    #[structopt(
        name = "no-anchors",
        long,
        help = "Removes the caret and dollar sign anchors from the resulting regular expression",
        long_help = "Removes the caret and dollar sign anchors from the resulting regular expression.\n\n\
                     By default, anchors are added to every generated regular expression\n\
                     which guarantee that the expression exactly matches only the test cases\n\
                     given as input and nothing else.\n\
                     This flag removes the anchors, thereby allowing to match the test cases also\n\
                     when they occur within a larger string that contains other content as well.",
        display_order = 15
    )]
    are_anchors_disabled: bool,

    #[structopt(
        name = "colorize",
        short,
        long,
        help = "Provides syntax highlighting for the resulting regular expression",
        display_order = 16
    )]
    is_output_colorized: bool,

    // --------------------
    // OPTIONS
    // --------------------
    #[structopt(
        name = "file",
        value_name = "FILE",
        short,
        long,
        parse(from_os_str),
        required_unless = "input",
        help = "Reads test cases on separate lines from a file",
        long_help = "Reads test cases on separate lines from a file.\n\n\
                     Lines may be ended with either a newline (`\\n`) or\n\
                     a carriage return with a line feed (`\\r\\n`).\n\
                     The final line ending is optional."
    )]
    file_path: Option<PathBuf>,

    #[structopt(
        name = "min-repetitions",
        value_name = "QUANTITY",
        long,
        default_value = "1",
        validator = repetition_options_validator,
        help = "Specifies the minimum quantity of substring repetitions\n\
                to be converted if --repetitions is set"
    )]
    minimum_repetitions: u32,

    #[structopt(
        name = "min-substring-length",
        value_name = "LENGTH",
        long,
        default_value = "1",
        validator = repetition_options_validator,
        help = "Specifies the minimum length a repeated substring must have\n\
                in order to be converted if --repetitions is set"
    )]
    minimum_substring_length: u32,
}

fn main() {
    let cli = Cli::from_args();
    handle_input(&cli, obtain_input(&cli));
}

fn obtain_input(cli: &Cli) -> Result<Vec<String>, Error> {
    let is_stdin_available = atty::isnt(atty::Stream::Stdin);

    if !cli.input.is_empty() {
        let is_single_item = cli.input.len() == 1;
        let is_hyphen = cli.input.get(0).unwrap() == "-";

        if is_single_item && is_hyphen && is_stdin_available {
            Ok(std::io::stdin()
                .lock()
                .lines()
                .map(|line| line.unwrap())
                .collect_vec())
        } else {
            Ok(cli.input.clone())
        }
    } else if let Some(file_path) = &cli.file_path {
        let is_hyphen = file_path.as_os_str() == "-";
        let path = if is_hyphen && is_stdin_available {
            let mut stdin_file_path = String::new();
            std::io::stdin().read_to_string(&mut stdin_file_path)?;
            PathBuf::from(stdin_file_path.trim())
        } else {
            file_path.to_path_buf()
        };
        match std::fs::read_to_string(&path) {
            Ok(file_content) => Ok(file_content.lines().map(|it| it.to_string()).collect_vec()),
            Err(error) => Err(error),
        }
    } else {
        Err(Error::new(
            ErrorKind::InvalidInput,
            "error: no valid input could be found whatsoever",
        ))
    }
}

fn handle_input(cli: &Cli, input: Result<Vec<String>, Error>) {
    match input {
        Ok(test_cases) => {
            let mut builder = RegExpBuilder::from(&test_cases);

            if cli.is_digit_converted {
                builder.with_conversion_of_digits();
            }

            if cli.is_non_digit_converted {
                builder.with_conversion_of_non_digits();
            }

            if cli.is_space_converted {
                builder.with_conversion_of_whitespace();
            }

            if cli.is_non_space_converted {
                builder.with_conversion_of_non_whitespace();
            }

            if cli.is_word_converted {
                builder.with_conversion_of_words();
            }

            if cli.is_non_word_converted {
                builder.with_conversion_of_non_words();
            }

            if cli.is_repetition_converted {
                builder.with_conversion_of_repetitions();
            }

            if cli.is_case_ignored {
                builder.with_case_insensitive_matching();
            }

            if cli.is_group_captured {
                builder.with_capturing_groups();
            }

            if cli.is_non_ascii_char_escaped {
                builder.with_escaping_of_non_ascii_chars(
                    cli.is_astral_code_point_converted_to_surrogate,
                );
            }

            if cli.is_verbose_mode_enabled {
                builder.with_verbose_mode();
            }

            if cli.is_caret_anchor_disabled {
                builder.without_start_anchor();
            }

            if cli.is_dollar_sign_anchor_disabled {
                builder.without_end_anchor();
            }

            if cli.are_anchors_disabled {
                builder.without_anchors();
            }

            if cli.is_output_colorized {
                builder.with_syntax_highlighting();
            }

            builder
                .with_minimum_repetitions(cli.minimum_repetitions)
                .with_minimum_substring_length(cli.minimum_substring_length);

            let regexp = builder.build();

            println!("{}", regexp);
        }
        Err(error) => match error.kind() {
            ErrorKind::NotFound => eprintln!("error: the specified file could not be found"),
            ErrorKind::InvalidData => {
                eprintln!("error: the specified file's encoding is not valid UTF-8")
            }
            ErrorKind::PermissionDenied => {
                eprintln!("permission denied: the specified file could not be opened")
            }
            _ => eprintln!("error: {}", error),
        },
    }
}

fn repetition_options_validator(value: String) -> Result<(), String> {
    match value.parse::<u32>() {
        Ok(parsed_value) => {
            if parsed_value > 0 {
                Ok(())
            } else {
                Err(String::from("Value must not be zero"))
            }
        }
        Err(_) => Err(String::from("Value is not a valid unsigned integer")),
    }
}
