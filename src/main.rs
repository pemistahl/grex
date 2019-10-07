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
use std::{
    fs::File,
    io::{self, stdin, BufRead, BufReader},
    path::PathBuf,
};
use structopt::StructOpt;

#[macro_use]
mod macros;
mod ast;
mod dfa;

#[derive(StructOpt)]
#[structopt(author, about)]
struct Cli {
    input: Vec<String>,
    #[structopt(short = "f", long = "file")]
    file: Option<PathBuf>,
}

fn handle_errors<E: std::error::Error>(e: E) -> ! {
    eprintln!("{}", e);
    std::process::exit(1)
}

fn main() {
    let cli = Cli::from_args();
    let input = if !cli.input.is_empty() {
        cli.input
    } else if let Some(f) = cli.file {
        BufReader::new(File::open(f).map_err(handle_errors).unwrap())
            .lines()
            .collect::<Result<Vec<_>, io::Error>>()
            .map_err(handle_errors)
            .unwrap()
    } else {
        stdin()
            .lock()
            .lines()
            .collect::<Result<Vec<_>, io::Error>>()
            .map_err(handle_errors)
            .unwrap()
    };
    println!("{}", DFA::from(input).to_regex());
}
