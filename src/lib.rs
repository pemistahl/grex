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

//! ## 1. What does this tool do?
//!
//! *grex* is a library as well as a command-line utility that is meant to simplify the often complicated and tedious task of creating regular expressions. It does so by automatically generating regular expressions from user-provided test cases.
//!
//! This project has started as a Rust port of the JavaScript tool [*regexgen*](https://github.com/devongovett/regexgen) written by [Devon Govett](https://github.com/devongovett). Although a lot of further useful features could be added to it, its development was apparently ceased several years ago. The plan is now to add these new features to *grex* as Rust really shines when it comes to command-line tools. *grex* offers all features that *regexgen* provides, and more.
//!
//! The philosophy of this project is to generate the most specific regular expression possible by default which exactly matches the given input only and nothing else. With the use of command-line flags (in the CLI tool) or preprocessing methods (in the library), more generalized expressions can be created.
//!
//! ## 2. Current features
//!
//! - literals
//! - character classes
//! - detection of common prefixes and suffixes
//! - detection of repeated substrings and conversion to `{min,max}` quantifier notation
//! - alternation using `|` operator
//! - optionality using `?` quantifier
//! - escaping of non-ascii characters, with optional conversion of astral code points to surrogate pairs
//! - concatenation of all of the former
//! - reading input strings from the command-line or from a file
//!
//! ## 3. How to use?
//!
//! The code snippets below show how to use the public api.
//!
//! For [more detailed examples](https://github.com/pemistahl/grex/tree/master#examples), please take a look at the project's readme file on GitHub.
//!
//! ### 3.1 Default settings
//!
//! ```
//! let regexp = grex::RegExpBuilder::from(&["a", "aa", "aaa"]).build();
//! assert_eq!(regexp, "^a(aa?)?$");
//! ```
//!
//! ### 3.2 Convert repeated substrings
//!
//! ```
//! let regexp = grex::RegExpBuilder::from(&["a", "aa", "aaa"])
//!     .with_conversion_of(&[grex::Feature::Repetition])
//!     .build();
//! assert_eq!(regexp, "^a{1,3}$");
//! ```
//!
//! ### 3.3 Escape non-ascii characters
//!
//! ```
//! let regexp = grex::RegExpBuilder::from(&["You smell like 💩."])
//!     .with_escaping_of_non_ascii_chars(false)
//!     .build();
//! assert_eq!(regexp, "^You smell like \\u{1f4a9}\\.$");
//! ```
//!
//! ### 3.4 Escape astral code points using surrogate pairs
//!
//! Old versions of JavaScript do not support unicode escape sequences for the astral code planes (range `U+010000` to `U+10FFFF`). In order to support these symbols in JavaScript regular expressions, the conversion to surrogate pairs is necessary. More information on that matter can be found [here](https://mathiasbynens.be/notes/javascript-unicode).
//!
//! ```
//! let regexp = grex::RegExpBuilder::from(&["You smell like 💩."])
//!     .with_escaping_of_non_ascii_chars(true)
//!     .build();
//! assert_eq!(regexp, "^You smell like \\u{d83d}\\u{dca9}\\.$");
//! ```
//!
//! ### 3.5 Combine multiple features
//!
//! ```
//! let regexp = grex::RegExpBuilder::from(&["You smell like 💩💩💩."])
//!     .with_conversion_of(&[grex::Feature::Repetition])
//!     .with_escaping_of_non_ascii_chars(false)
//!     .build();
//! assert_eq!(regexp, "^You smel{2} like \\u{1f4a9}{3}\\.$");
//! ```

#[macro_use]
mod macros;

mod ast;
mod dfa;
mod fmt;
mod grapheme;
mod regexp;

pub use regexp::Feature;
pub use regexp::RegExpBuilder;
