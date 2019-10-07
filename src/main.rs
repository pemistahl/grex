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

use crate::dfa::DFA;
use itertools::Itertools;
use std::io::ErrorKind;
use std::path::PathBuf;
use structopt::StructOpt;

#[macro_use]
mod macros;
mod ast;
mod dfa;

#[derive(StructOpt)]
#[structopt(author, about)]
struct CLI {
    #[structopt(required_unless = "file", conflicts_with = "file")]
    input: Vec<String>,

    #[structopt(
        name = "file",
        short,
        long,
        parse(from_os_str),
        required_unless = "input"
    )]
    file_path: Option<PathBuf>,
}

fn main() {
    let cli = CLI::from_args();

    if !cli.input.is_empty() {
        let input = cli.input.iter().map(|it| it.as_str()).collect_vec();
        println!("{}", DFA::from(input).to_regex());
    } else if let Some(file_path) = cli.file_path {
        match std::fs::read_to_string(file_path) {
            Ok(file_content) => {
                let input = file_content.lines().collect_vec();
                println!("{}", DFA::from(input).to_regex());
            }
            Err(error) => match error.kind() {
                ErrorKind::NotFound => eprintln!("Error: The provided file could not be found"),
                _ => eprintln!("Error: The provided file was found but could not be opened"),
            },
        }
    }
}
