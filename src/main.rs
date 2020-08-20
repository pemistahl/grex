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

use grex::{Feature, RegExpBuilder};
use itertools::Itertools;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;
use structopt::clap::AppSettings;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(
    author = "© 2019-2020 Peter M. Stahl <pemistahl@gmail.com>",
    about = "Licensed under the Apache License, Version 2.0\n\
             Downloadable from https://crates.io/crates/grex\n\
             Source code at https://github.com/pemistahl/grex\n\n\
             grex generates regular expressions from user-provided test cases.",
    version_short = "v",
    global_setting = AppSettings::AllowLeadingHyphen
)]
struct CLI {
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
        name = "colorize",
        short,
        long,
        help = "Provides syntax highlighting for the resulting regular expression",
        display_order = 12
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
        name = "exclusions",
        value_name = "INPUT",
        long,
        conflicts_with = "exclusions-file",
        help = "One or more negative test cases separated by blank space"
    )]
    exclusions: Option<Vec<String>>,

    #[structopt(
        name = "exclusions-file",
        value_name = "FILE",
        long,
        parse(from_os_str),
        help = "Reads negative test cases on separate lines from a file",
        long_help = "Reads negative test cases on separate lines from a file.\n\n\
                     Lines may be ended with either a newline (`\\n`) or\n\
                     a carriage return with a line feed (`\\r\\n`).\n\
                     The final line ending is optional."
    )]
    exclusions_file_path: Option<PathBuf>,

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
    let cli = CLI::from_args();
    handle_input(&cli, obtain_input(&cli));
}

fn read_input_file(file_path: &PathBuf) -> Result<Vec<String>, Error> {
    let file_content = std::fs::read_to_string(&file_path)?;

    Ok(file_content.lines().map(|it| it.to_string()).collect_vec())
}

fn obtain_input(cli: &CLI) -> Result<Vec<String>, Error> {
    if !cli.input.is_empty() {
        Ok(cli.input.clone())
    } else if let Some(file_path) = &cli.file_path {
        read_input_file(file_path)
    } else {
        Err(Error::new(
            ErrorKind::InvalidInput,
            "error: no valid input could be found whatsoever",
        ))
    }
}

fn file_error(source: &str, error: Error) {
    match error.kind() {
        ErrorKind::NotFound => eprintln!("error: the specified {} file could not be found", source),
        ErrorKind::InvalidData => eprintln!(
            "error: the specified {} file's encoding is not valid UTF-8",
            source
        ),
        ErrorKind::PermissionDenied => eprintln!(
            "permission denied: the {} specified file could not be opened",
            source
        ),
        _ => eprintln!("error on {} file: {}", source, error),
    }
}

fn build_builder(cli: &CLI, test_cases: &[String]) -> RegExpBuilder {
    let mut builder = RegExpBuilder::from(&test_cases);

    if let Some(exclusions) = &cli.exclusions {
        builder.with_negative_matches(&exclusions);
    } else if let Some(exclusions_file_path) = &cli.exclusions_file_path {
        match read_input_file(exclusions_file_path) {
            Ok(lines) => {
                builder.with_negative_matches(&lines);
            }
            Err(error) => {
                file_error("negative match", error);
                panic!();
            }
        }
    }

    let mut conversion_features = vec![];

    if cli.is_digit_converted {
        conversion_features.push(Feature::Digit);
    }

    if cli.is_non_digit_converted {
        conversion_features.push(Feature::NonDigit);
    }

    if cli.is_space_converted {
        conversion_features.push(Feature::Space);
    }

    if cli.is_non_space_converted {
        conversion_features.push(Feature::NonSpace);
    }

    if cli.is_word_converted {
        conversion_features.push(Feature::Word);
    }

    if cli.is_non_word_converted {
        conversion_features.push(Feature::NonWord);
    }

    if cli.is_repetition_converted {
        conversion_features.push(Feature::Repetition);
    }

    if cli.is_case_ignored {
        conversion_features.push(Feature::CaseInsensitivity);
    }

    if cli.is_group_captured {
        conversion_features.push(Feature::CapturingGroup);
    }

    if !conversion_features.is_empty() {
        builder.with_conversion_of(&conversion_features);
    }

    if cli.is_non_ascii_char_escaped {
        builder.with_escaping_of_non_ascii_chars(cli.is_astral_code_point_converted_to_surrogate);
    }

    if cli.is_output_colorized {
        builder.with_syntax_highlighting();
    }

    builder
        .with_minimum_repetitions(cli.minimum_repetitions)
        .with_minimum_substring_length(cli.minimum_substring_length);

    builder
}

fn handle_input(cli: &CLI, input: Result<Vec<String>, Error>) {
    match input {
        Ok(test_cases) => {
            let mut builder = build_builder(cli, &test_cases);

            let regexp = builder.build();

            println!("{}", regexp);
        }
        Err(error) => file_error("input", error),
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
