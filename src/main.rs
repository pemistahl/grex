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

use std::io::{Error, ErrorKind};
use std::path::PathBuf;

use itertools::Itertools;
use structopt::StructOpt;

use grex::RegExpBuilder;

#[derive(StructOpt)]
#[structopt(author, about)]
struct CLI {
    #[structopt(
        value_name = "INPUT",
        required_unless = "file",
        conflicts_with = "file",
        help = "One or more test cases separated by blank space"
    )]
    input: Vec<String>,

    #[structopt(
        name = "file",
        value_name = "FILE",
        short,
        long,
        parse(from_os_str),
        required_unless = "input",
        help = "Reads test cases separated by newline characters from a file"
    )]
    file_path: Option<PathBuf>,

    #[structopt(
        name = "finite-repetition",
        long,
        help = "Detects repeated substrings and conflates them using {min,max} quantifiers"
    )]
    use_finite_repetition: bool,

    #[structopt(
        name = "escape",
        long,
        help = "Replaces all non-ASCII characters with unicode escape sequences"
    )]
    escape_non_ascii_chars: bool,

    #[structopt(
        name = "with-surrogates",
        long,
        requires = "escape",
        help = "Converts astral code points to surrogate pairs if --escape is set"
    )]
    use_surrogate_pairs: bool,
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
            let mut builder = RegExpBuilder::from(test_cases);

            if cli.use_finite_repetition {
                builder.with_finite_repetition();
            }
            if cli.escape_non_ascii_chars {
                builder.with_escaped_non_ascii_chars(cli.use_surrogate_pairs);
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
