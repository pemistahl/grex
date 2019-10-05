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
use structopt::StructOpt;

#[macro_use]
mod macros;
mod ast;
mod dfa;

#[derive(StructOpt)]
#[structopt(author, about)]
struct Cli {
    #[structopt(required = true)]
    input: Vec<String>,
}

fn main() {
    let cli = Cli::from_args();
    let input = cli.input.iter().map(|it| it.as_str()).collect_vec();
    println!("{}", DFA::from(input).to_regex());
}
