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

use std::cmp::Ordering;
use std::io::ErrorKind;
use std::path::PathBuf;

use itertools::Itertools;
use structopt::StructOpt;

use crate::dfa::DFA;
use crate::postprocessing::escape_non_ascii_chars;

#[macro_use]
mod macros;
mod ast;
mod dfa;
mod fmt;
mod postprocessing;

#[derive(StructOpt)]
#[structopt(author, about)]
struct CLI {
    #[structopt(
        value_name = "INPUT",
        required_unless = "file",
        conflicts_with = "file",
        help = "One or more strings separated by blank space"
    )]
    input: Vec<String>,

    #[structopt(
        name = "file",
        value_name = "FILE",
        short,
        long,
        parse(from_os_str),
        required_unless = "input",
        help = "Reads input strings from a file with each string on a separate line"
    )]
    file_path: Option<PathBuf>,

    #[structopt(
        name = "escape",
        long,
        help = "Replaces all non-ASCII characters with unicode escape sequences"
    )]
    escape_non_ascii_chars: bool,

    #[structopt(
        name = "with-surrogate-pairs",
        long,
        requires = "escape",
        help = "Converts astral code points to surrogate pairs if --escape is set"
    )]
    use_surrogate_pairs: bool,
}

fn main() {
    let cli = CLI::from_args();

    if !cli.input.is_empty() {
        let mut input = cli.input.iter().map(|it| it.as_str()).collect_vec();
        sort_input(&mut input);

        let regex = DFA::from(input).to_regex();
        if cli.escape_non_ascii_chars {
            println!(
                "{}",
                escape_non_ascii_chars(&regex, cli.use_surrogate_pairs)
            );
        } else {
            println!("{}", regex);
        }
    } else if let Some(file_path) = cli.file_path {
        match std::fs::read_to_string(&file_path) {
            Ok(file_content) => {
                let mut input = file_content.lines().collect_vec();
                sort_input(&mut input);

                let regex = DFA::from(input).to_regex();
                if cli.escape_non_ascii_chars {
                    println!(
                        "{}",
                        escape_non_ascii_chars(&regex, cli.use_surrogate_pairs)
                    );
                } else {
                    println!("{}", regex);
                }
            }
            Err(error) => match error.kind() {
                ErrorKind::NotFound => {
                    eprintln!("error: The file {:?} could not be found", file_path)
                }
                _ => eprintln!(
                    "error: The file {:?} was found but could not be opened",
                    file_path
                ),
            },
        }
    }
}

fn sort_input(strs: &mut Vec<&str>) {
    strs.sort();
    strs.dedup();
    strs.sort_by(|&a, &b| match a.len().cmp(&b.len()) {
        Ordering::Equal => a.cmp(&b),
        other => other,
    });
}
