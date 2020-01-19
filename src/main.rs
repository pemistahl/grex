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

use std::io::{Error, ErrorKind};
use std::path::PathBuf;

use itertools::Itertools;
use structopt::StructOpt;

use grex::{Feature, RegExpBuilder};

#[derive(StructOpt)]
#[structopt(author, about, version_short = "v")]
struct CLI {
    #[structopt(
        value_name = "INPUT",
        required_unless = "file",
        conflicts_with = "file",
        long_help = "One or more test cases separated by blank space"
    )]
    input: Vec<String>,

    #[structopt(
        name = "file",
        value_name = "FILE",
        short,
        long,
        parse(from_os_str),
        required_unless = "input",
        long_help = "Reads test cases separated by newline characters from a file"
    )]
    file_path: Option<PathBuf>,

    #[structopt(
        name = "digits",
        short,
        long,
        long_help = "Converts any Unicode decimal digit to \\d",
        display_order = 1
    )]
    is_digit_converted: bool,

    #[structopt(
        name = "words",
        short,
        long,
        long_help = "Converts any Unicode word character to \\w\n(overrides --digits if set)",
        display_order = 2
    )]
    is_word_converted: bool,

    #[structopt(
        name = "spaces",
        short,
        long,
        long_help = "Converts any Unicode whitespace character to \\s",
        display_order = 3
    )]
    is_space_converted: bool,

    #[structopt(
        name = "repetitions",
        short,
        long,
        long_help = "Detects repeated non-overlapping substrings and\nconverts them to {min,max} quantifier notation",
        display_order = 4
    )]
    is_repetition_converted: bool,

    #[structopt(
        name = "escape",
        short,
        long,
        long_help = "Replaces all non-ASCII characters with unicode escape sequences",
        display_order = 5
    )]
    is_non_ascii_char_escaped: bool,

    #[structopt(
        name = "with-surrogates",
        long,
        requires = "escape",
        long_help = "Converts astral code points to surrogate pairs if --escape is set",
        display_order = 6
    )]
    is_astral_code_point_converted_to_surrogate: bool,
}

fn main() {
    let cli = CLI::from_args();
    handle_input(&cli, obtain_input(&cli));
}

fn obtain_input(cli: &CLI) -> Result<Vec<String>, Error> {
    if !cli.input.is_empty() {
        Ok(cli.input.clone())
    } else if let Some(file_path) = &cli.file_path {
        match std::fs::read_to_string(&file_path) {
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

fn handle_input(cli: &CLI, input: Result<Vec<String>, Error>) {
    match input {
        Ok(test_cases) => {
            let mut builder = RegExpBuilder::from(&test_cases);
            let mut conversion_features = vec![];

            if cli.is_digit_converted {
                conversion_features.push(Feature::Digit);
            }

            if cli.is_word_converted {
                conversion_features.push(Feature::Word);
            }

            if cli.is_space_converted {
                conversion_features.push(Feature::Space);
            }

            if cli.is_repetition_converted {
                conversion_features.push(Feature::Repetition);
            }

            if !conversion_features.is_empty() {
                builder.with_conversion_of(&conversion_features);
            }

            if cli.is_non_ascii_char_escaped {
                builder.with_escaping_of_non_ascii_chars(
                    cli.is_astral_code_point_converted_to_surrogate,
                );
            }

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
