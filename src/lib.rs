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

//! ## 1. What does this tool do?
//!
//! *grex* is a library as well as a command-line utility that is meant to simplify the often
//! complicated and tedious task of creating regular expressions. It does so by automatically
//! generating a single regular expression from user-provided test cases. The resulting
//! expression is guaranteed to match the test cases which it was generated from.
//!
//! This project has started as a Rust port of the JavaScript tool
//! [*regexgen*](https://github.com/devongovett/regexgen) written by
//! [Devon Govett](https://github.com/devongovett). Although a lot of further useful features
//! could be added to it, its development was apparently ceased several years ago. The plan
//! is now to add these new features to *grex* as Rust really shines when it comes to
//! command-line tools. *grex* offers all features that *regexgen* provides, and more.
//!
//! The philosophy of this project is to generate the most specific regular expression
//! possible by default which exactly matches the given input only and nothing else.
//! With the use of command-line flags (in the CLI tool) or preprocessing methods
//! (in the library), more generalized expressions can be created.
//!
//! The produced expressions are [Perl-compatible regular expressions](https://www.pcre.org)
//! which are also compatible with the regular expression parser in Rust's
//! [*regex crate*](https://crates.io/crates/regex).
//! Other regular expression parsers or respective libraries from other programming languages
//! have not been tested so far, but they ought to be mostly compatible as well.
//!
//! ## 2. Do I still need to learn to write regexes then?
//!
//! **Definitely, yes!** Using the standard settings, *grex* produces a regular expression that
//! is guaranteed to match only the test cases given as input and nothing else. This has been
//! verified by [property tests](https://github.com/pemistahl/grex/blob/main/tests/property_tests.rs).
//! However, if the conversion to shorthand character classes such as `\w` is enabled, the
//! resulting regex matches a much wider scope of test cases. Knowledge about the consequences of
//! this conversion is essential for finding a correct regular expression for your business domain.
//!
//! *grex* uses an algorithm that tries to find the shortest possible regex for the given test cases.
//! Very often though, the resulting expression is still longer or more complex than it needs to be.
//! In such cases, a more compact or elegant regex can be created only by hand.
//! Also, every regular expression engine has different built-in optimizations.
//! *grex* does not know anything about those and therefore cannot optimize its regexes
//! for a specific engine.
//!
//! **So, please learn how to write regular expressions!** The currently best use case for *grex*
//! is to find an initial correct regex which should be inspected by hand if further optimizations
//! are possible.
//!
//! ## 3. Current features
//!
//! - literals
//! - character classes
//! - detection of common prefixes and suffixes
//! - detection of repeated substrings and conversion to `{min,max}` quantifier notation
//! - alternation using `|` operator
//! - optionality using `?` quantifier
//! - escaping of non-ascii characters, with optional conversion of astral code points to surrogate pairs
//! - case-sensitive or case-insensitive matching
//! - capturing or non-capturing groups
//! - optional anchors `^` and `$`
//! - fully compliant to [Unicode Standard 15.0](https://unicode.org/versions/Unicode15.0.0)
//! - fully compatible with [*regex* crate 1.9.0+](https://crates.io/crates/regex)
//! - correctly handles graphemes consisting of multiple Unicode symbols
//! - reads input strings from the command-line or from a file
//! - produces more readable expressions indented on multiple using optional verbose mode
//!
//! ## 4. How to use?
//!
//! The code snippets below show how to use the public api.
//!
//! For [more detailed examples](https://github.com/pemistahl/grex/tree/main#53-examples), please
//! take a look at the project's readme file on GitHub.
//!
//! ### 4.1 Default settings
//!
//! Test cases are passed either from a collection via [`RegExpBuilder::from()`]
//! or from a file via [`RegExpBuilder::from_file()`].
//!
//! ```
//! use grex::RegExpBuilder;
//!
//! let regexp = RegExpBuilder::from(&["a", "aa", "aaa"]).build();
//! assert_eq!(regexp, "^a(?:aa?)?$");
//! ```
//!
//! ### 4.2 Convert to character classes
//!
//! ```
//! use grex::RegExpBuilder;
//!
//! let regexp = RegExpBuilder::from(&["a", "aa", "123"])
//!     .with_conversion_of_digits()
//!     .with_conversion_of_words()
//!     .build();
//! assert_eq!(regexp, "^(?:\\d\\d\\d|\\w(?:\\w)?)$");
//! ```
//!
//! ### 4.3 Convert repeated substrings
//!
//! ```
//! use grex::RegExpBuilder;
//!
//! let regexp = RegExpBuilder::from(&["aa", "bcbc", "defdefdef"])
//!     .with_conversion_of_repetitions()
//!     .build();
//! assert_eq!(regexp, "^(?:a{2}|(?:bc){2}|(?:def){3})$");
//! ```
//!
//! By default, *grex* converts each substring this way which is at least a single character long
//! and which is subsequently repeated at least once. You can customize these two parameters
//! if you like.
//!
//! In the following example, the test case `aa` is not converted to `a{2}` because the repeated
//! substring `a` has a length of 1, but the minimum substring length has been set to 2.
//!
//! ```
//! use grex::RegExpBuilder;
//!
//! let regexp = RegExpBuilder::from(&["aa", "bcbc", "defdefdef"])
//!     .with_conversion_of_repetitions()
//!     .with_minimum_substring_length(2)
//!     .build();
//! assert_eq!(regexp, "^(?:aa|(?:bc){2}|(?:def){3})$");
//! ```
//!
//! Setting a minimum number of 2 repetitions in the next example, only the test case `defdefdef`
//! will be converted because it is the only one that is repeated twice.
//!
//! ```
//! use grex::RegExpBuilder;
//!
//! let regexp = RegExpBuilder::from(&["aa", "bcbc", "defdefdef"])
//!     .with_conversion_of_repetitions()
//!     .with_minimum_repetitions(2)
//!     .build();
//! assert_eq!(regexp, "^(?:bcbc|aa|(?:def){3})$");
//! ```
//!
//! ### 4.4 Escape non-ascii characters
//!
//! ```
//! use grex::RegExpBuilder;
//!
//! let regexp = RegExpBuilder::from(&["You smell like 💩."])
//!     .with_escaping_of_non_ascii_chars(false)
//!     .build();
//! assert_eq!(regexp, "^You smell like \\u{1f4a9}\\.$");
//! ```
//!
//! Old versions of JavaScript do not support unicode escape sequences for
//! the astral code planes (range `U+010000` to `U+10FFFF`). In order to
//! support these symbols in JavaScript regular expressions, the conversion
//! to surrogate pairs is necessary. More information on that matter can be
//! found [here](https://mathiasbynens.be/notes/javascript-unicode).
//!
//! ```
//! use grex::RegExpBuilder;
//!
//! let regexp = RegExpBuilder::from(&["You smell like 💩."])
//!     .with_escaping_of_non_ascii_chars(true)
//!     .build();
//! assert_eq!(regexp, "^You smell like \\u{d83d}\\u{dca9}\\.$");
//! ```
//!
//! ### 4.5 Case-insensitive matching
//!
//! The regular expressions that *grex* generates are case-sensitive by default.
//! Case-insensitive matching can be enabled like so:
//!
//! ```
//! use grex::RegExpBuilder;
//!
//! let regexp = RegExpBuilder::from(&["big", "BIGGER"])
//!     .with_case_insensitive_matching()
//!     .build();
//! assert_eq!(regexp, "(?i)^big(?:ger)?$");
//! ```
//!
//! ### 4.6 Capturing Groups
//!
//! Non-capturing groups are used by default.
//! Extending the previous example, you can switch to capturing groups instead.
//!
//! ```
//! use grex::RegExpBuilder;
//!
//! let regexp = RegExpBuilder::from(&["big", "BIGGER"])
//!     .with_case_insensitive_matching()
//!     .with_capturing_groups()
//!     .build();
//! assert_eq!(regexp, "(?i)^big(ger)?$");
//! ```
//!
//! ### 4.7 Verbose mode
//!
//! If you find the generated regular expression hard to read, you can enable verbose mode.
//! The expression is then put on multiple lines and indented to make it more pleasant to the eyes.
//!
//! ```
//! use grex::RegExpBuilder;
//! use indoc::indoc;
//!
//! let regexp = RegExpBuilder::from(&["a", "b", "bcd"])
//!     .with_verbose_mode()
//!     .build();
//!
//! assert_eq!(regexp, indoc!(
//!     r#"
//!     (?x)
//!     ^
//!       (?:
//!         b
//!         (?:
//!           cd
//!         )?
//!         |
//!         a
//!       )
//!     $"#
//! ));
//! ```
//!
//! ### 4.8 Disable anchors
//!
//! By default, the anchors `^` and `$` are put around every generated regular expression in order
//! to ensure that it matches only the test cases given as input. Often enough, however, it is
//! desired to use the generated pattern as part of a larger one. For this purpose, the anchors
//! can be disabled, either separately or both of them.
//!
//! ```
//! use grex::RegExpBuilder;
//!
//! let regexp = RegExpBuilder::from(&["a", "aa", "aaa"])
//!     .without_anchors()
//!     .build();
//! assert_eq!(regexp, "a(?:aa?)?");
//! ```
//!
//! ### 5. How does it work?
//!
//! 1. A [deterministic finite automaton](https://en.wikipedia.org/wiki/Deterministic_finite_automaton) (DFA)
//! is created from the input strings.
//!
//! 2. The number of states and transitions between states in the DFA is reduced by applying
//! [Hopcroft's DFA minimization algorithm](https://en.wikipedia.org/wiki/DFA_minimization#Hopcroft.27s_algorithm).
//!
//! 3. The minimized DFA is expressed as a system of linear equations which are solved with
//! [Brzozowski's algebraic method](http://cs.stackexchange.com/questions/2016/how-to-convert-finite-automata-to-regular-expressions#2392),
//! resulting in the final regular expression.

#[macro_use]
mod macros;

mod builder;
mod cluster;
mod component;
mod config;
mod dfa;
mod expression;
mod format;
mod grapheme;
mod quantifier;
mod regexp;
mod substring;
mod unicode_tables;

#[cfg(feature = "python")]
mod python;

#[cfg(target_family = "wasm")]
mod wasm;

pub use builder::RegExpBuilder;

#[cfg(target_family = "wasm")]
pub use wasm::RegExpBuilder as WasmRegExpBuilder;
