![grex](logo.png)

<br>

[![Build Status](https://travis-ci.org/pemistahl/grex.svg?branch=master)](https://travis-ci.org/pemistahl/grex)
[![codecov](https://codecov.io/gh/pemistahl/grex/branch/master/graph/badge.svg)](https://codecov.io/gh/pemistahl/grex)
[![Crates.io](https://img.shields.io/crates/v/grex.svg)](https://crates.io/crates/grex)
[![Docs.rs](https://docs.rs/grex/badge.svg)](https://docs.rs/grex)
[![Downloads](https://img.shields.io/crates/d/grex.svg)](https://crates.io/crates/grex)
[![license](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](https://www.apache.org/licenses/LICENSE-2.0)

[![Linux Download](https://img.shields.io/badge/Linux%20Download-v1.0.0-blue?logo=Linux)](https://github.com/pemistahl/grex/releases/download/v1.0.0/grex-v1.0.0-x86_64-unknown-linux-musl.tar.gz)
[![MacOS Download](https://img.shields.io/badge/macOS%20Download-v1.0.0-blue?logo=Apple)](https://github.com/pemistahl/grex/releases/download/v1.0.0/grex-v1.0.0-x86_64-apple-darwin.tar.gz)
[![Windows Download](https://img.shields.io/badge/Windows%20Download-v1.0.0-blue?logo=Windows)](https://github.com/pemistahl/grex/releases/download/v1.0.0/grex-v1.0.0-x86_64-pc-windows-msvc.zip)

## <a name="table-of-contents"></a> Table of Contents
1. [What does this tool do?](#what-does-tool-do)
2. [Current features](#current-features)
3. [How to install?](#how-to-install)  
  3.1 [The command-line tool](#how-to-install-cli)  
  3.2 [The library](#how-to-install-library)
4. [How to use?](#how-to-use)  
  4.1 [The command-line tool](#how-to-use-cli)  
  4.2 [The library](#how-to-use-library)  
  4.3 [Examples](#examples)
5. [How to build?](#how-to-build)
6. [How does it work?](#how-does-it-work)
7. [Do you want to contribute?](#contribution)
 

## 1. <a name="what-does-tool-do"></a> What does this tool do? <sup>[Top ▲](#table-of-contents)</sup>

*grex* is a library as well as a command-line utility that is meant to simplify the often 
complicated and tedious task of creating regular expressions. It does so by automatically 
generating regular expressions from user-provided test cases. The produced expressions
are Perl-compatible regular expressions (PCRE) which are also compatible with the
regular expression parser in the [*regex*](https://crates.io/crates/regex) crate.
Other regular expression parsers or respective libraries from other programming languages 
have not been tested so far. 

This project has started as a Rust port of the JavaScript tool 
[*regexgen*](https://github.com/devongovett/regexgen) written by 
[Devon Govett](https://github.com/devongovett). Although a lot of further useful features 
could be added to it, its development was apparently ceased several years ago. The plan 
is now to add these new features to *grex* as Rust really shines when it comes to 
command-line tools. *grex* offers all features that *regexgen* provides, and more.

The philosophy of this project is to generate the most specific regular expression 
possible by default which exactly matches the given input only and nothing else. 
With the use of command-line flags (in the CLI tool) or preprocessing methods 
(in the library), more generalized expressions can be created.

## 2. <a name="current-features"></a> Current Features <sup>[Top ▲](#table-of-contents)</sup>
- literals
- character classes
- detection of common prefixes and suffixes
- detection of repeated substrings and conversion to `{min,max}` quantifier notation
- alternation using `|` operator
- optionality using `?` quantifier
- escaping of non-ascii characters, with optional conversion of astral code points to surrogate pairs
- fully Unicode-aware, correctly handles graphemes consisting of multiple Unicode symbols
- reading input strings from the command-line or from a file
- optional syntax highlighting for nicer output in supported terminals

## 3. <a name="how-to-install"></a> How to install? <sup>[Top ▲](#table-of-contents)</sup>

### 3.1 <a name="how-to-install-cli"></a> The command-line tool <sup>[Top ▲](#table-of-contents)</sup>

Pre-compiled 64-Bit binaries are available within the package managers [Scoop](https://scoop.sh) 
(for Windows) and [Homebrew](https://brew.sh) (for macOS).

#### Scoop
```
scoop install grex
```

#### Homebrew
```
brew install pemistahl/formulas/grex
```

Alternatively, you can download the self-contained executable for your platform above and put it 
in a place of your choice.

*grex* is also hosted on [crates.io](https://crates.io/crates/grex), 
the official Rust package registry. If you are a Rust developer and already have the Rust 
toolchain installed, you can install by compiling from source using 
[*cargo*](https://doc.rust-lang.org/cargo/), the Rust package manager:

```
cargo install grex
```

### 3.2 <a name="how-to-install-library"></a> The library <sup>[Top ▲](#table-of-contents)</sup>

In order to use *grex* as a library, simply add it as a dependency to your `Cargo.toml` file:

```toml
[dependencies]
grex = "1.0.0"
```

## 4. <a name="how-to-use"></a> How to use? <sup>[Top ▲](#table-of-contents)</sup>

### 4.1 <a name="how-to-use-cli"></a> The command-line tool <sup>[Top ▲](#table-of-contents)</sup>

```
$ grex -h

grex 1.0.0
© 2019-2020 Peter M. Stahl <pemistahl@gmail.com>
Licensed under the Apache License, Version 2.0
Downloadable from https://crates.io/crates/grex
Source code at https://github.com/pemistahl/grex

grex generates regular expressions from user-provided test cases.

USAGE:
    grex [FLAGS] <INPUT>... --file <FILE>

FLAGS:
    -d, --digits             Converts any Unicode decimal digit to \d
    -D, --non-digits         Converts any character which is not a Unicode decimal digit to \D
    -s, --spaces             Converts any Unicode whitespace character to \s
    -S, --non-spaces         Converts any character which is not a Unicode whitespace character to \S
    -w, --words              Converts any Unicode word character to \w
    -W, --non-words          Converts any character which is not a Unicode word character to \W
    -r, --repetitions        Detects repeated non-overlapping substrings and
                             converts them to {min,max} quantifier notation
    -e, --escape             Replaces all non-ASCII characters with unicode escape sequences
        --with-surrogates    Converts astral code points to surrogate pairs if --escape is set
    -c, --colorize           Provides syntax highlighting for the resulting regular expression
    -h, --help               Prints help information
    -v, --version            Prints version information

OPTIONS:
    -f, --file <FILE>    Reads test cases separated by newline characters from a file

ARGS:
    <INPUT>...    One or more test cases separated by blank space
 
```

### 4.2 <a name="how-to-use-library"></a> The library <sup>[Top ▲](#table-of-contents)</sup>

#### 4.2.1 Default settings

```rust
use grex::RegExpBuilder;

let regexp = RegExpBuilder::from(&["a", "aa", "aaa"]).build();
assert_eq!(regexp, "^a(aa?)?$");
```

#### 4.2.2 Convert repeated substrings

```rust
use grex::{Feature, RegExpBuilder};

let regexp = RegExpBuilder::from(&["a", "aa", "aaa"])
    .with_conversion_of(&[Feature::Repetition])
    .build();
assert_eq!(regexp, "^a{1,3}$");
```

#### 4.2.3 Convert to character classes

```rust
use grex::{Feature, RegExpBuilder};

let regexp = RegExpBuilder::from(&["a", "aa", "123"])
    .with_conversion_of(&[Feature::Digit, Feature::Word])
    .build();
assert_eq!(regexp, "^(\\d\\d\\d|\\w\\w|\\w)$");
```

#### 4.2.4 Escape non-ascii characters

```rust
use grex::RegExpBuilder;

let regexp = RegExpBuilder::from(&["You smell like 💩."])
    .with_escaping_of_non_ascii_chars(false)
    .build();
assert_eq!(regexp, "^You smell like \\u{1f4a9}\\.$");
```

#### 4.2.5 Escape astral code points using surrogate pairs

Old versions of JavaScript do not support unicode escape sequences for the astral code planes 
(range `U+010000` to `U+10FFFF`). In order to support these symbols in JavaScript regular 
expressions, the conversion to surrogate pairs is necessary. More information on that matter 
can be found [here](https://mathiasbynens.be/notes/javascript-unicode).

```rust
use grex::RegExpBuilder;

let regexp = RegExpBuilder::from(&["You smell like 💩."])
    .with_escaping_of_non_ascii_chars(true)
    .build();
assert_eq!(regexp, "^You smell like \\u{d83d}\\u{dca9}\\.$");
```

#### 4.2.6 Combine multiple features

```rust
use grex::{Feature, RegExpBuilder};

let regexp = RegExpBuilder::from(&["You smell like 💩💩💩."])
    .with_conversion_of(&[Feature::Repetition])
    .with_escaping_of_non_ascii_chars(false)
    .build();
assert_eq!(regexp, "^You smel{2} like \\u{1f4a9}{3}\\.$");
```

```rust
use grex::{Feature, RegExpBuilder};

let regexp = RegExpBuilder::from(&["a", "aa", "123"])
    .with_conversion_of(&[Feature::Repetition, Feature::Digit, Feature::Word])
    .build();
assert_eq!(regexp, "^(\\w{1,2}|\\d{3})$");
```

#### 4.2.7 Syntax highlighting

⚠ The method `with_syntax_highlighting()` may only be used if the resulting regular expression is meant to
be printed to the console. The regex string representation returned from enabling
this setting cannot be fed into the [*regex*](https://crates.io/crates/regex) crate.

```rust
use grex::RegExpBuilder;

let regexp = RegExpBuilder::from(&["a", "aa", "123"])
    .with_syntax_highlighting()
    .build();
```

### 4.3 <a name="examples"></a> Examples <sup>[Top ▲](#table-of-contents)</sup>

The following examples show the various supported regex syntax features:

```
$ grex a b c
^[a-c]$

$ grex a c d e f
^[ac-f]$

$ grex a b x de
^(de|[abx])$

$ grex a b bc
^(bc?|a)$

$ grex [a-z]
^\[a\-z\]$

$ grex -r b ba baa baaa
^b(a{1,3})?$

$ grex -r b ba baa baaaa
^b(a{1,2}|a{4})?$

$ grex y̆ a z
^(y̆|[az])$
Note: 
Grapheme y̆ consists of two Unicode symbols:
U+0079 (Latin Small Letter Y)
U+0306 (Combining Breve).

$ grex "I ♥ cake" "I ♥ cookies"
^I ♥ c(ookies|ake)$
Note:
Input containing blank space must be 
surrounded by quotation marks.
```

The string `"I ♥♥♥ 36 and ٣ and 💩💩."` serves as input for the following examples using the command-line notation:

```
$ grex <INPUT>
^I ♥♥♥ 36 and ٣ and 💩💩\.$

$ grex -e <INPUT>
^I \u{2665}\u{2665}\u{2665} 36 and \u{663} and \u{1f4a9}\u{1f4a9}\.$

$ grex -e --with-surrogates <INPUT>
^I \u{2665}\u{2665}\u{2665} 36 and \u{663} and \u{d83d}\u{dca9}\u{d83d}\u{dca9}\.$

$ grex -d <INPUT>
^I ♥♥♥ \d\d and \d and 💩💩\.$

$ grex -s <INPUT>
^I\s♥♥♥\s36\sand\s٣\sand\s💩💩\.$

$ grex -w <INPUT>
^\w ♥♥♥ \w\w \w\w\w \w \w\w\w 💩💩\.$

$ grex -D <INPUT>
^\D\D\D\D\D\D36\D\D\D\D\D٣\D\D\D\D\D\D\D\D$

$ grex -S <INPUT>
^\S \S\S\S \S\S \S\S\S \S \S\S\S \S\S\S$

$ grex -dsw <INPUT>
^\w\s♥♥♥\s\d\d\s\w\w\w\s\d\s\w\w\w\s💩💩\.$

$ grex -dswW <INPUT>
^\w\s\W\W\W\s\d\d\s\w\w\w\s\d\s\w\w\w\s\W\W\W$

$ grex -r <INPUT>
^I ♥{3} 36 and ٣ and 💩{2}\.$

$ grex -er <INPUT>
^I \u{2665}{3} 36 and \u{663} and \u{1f4a9}{2}\.$

$ grex -er --with-surrogates <INPUT>
^I \u{2665}{3} 36 and \u{663} and (\u{d83d}\u{dca9}){2}\.$

$ grex -dr <INPUT>
^I ♥{3} \d(\d and ){2}💩{2}\.$

$ grex -rs <INPUT>
^I\s♥{3}\s36\sand\s٣\sand\s💩{2}\.$

$ grex -rw <INPUT>
^\w ♥{3} \w(\w \w{3} ){2}💩{2}\.$

$ grex -Dr <INPUT>
^\D{6}36\D{5}٣\D{8}$

$ grex -rS <INPUT>
^\S \S(\S{2} ){2}\S{3} \S \S{3} \S{3}$

$ grex -rW <INPUT>
^I\W{5}36\Wand\W٣\Wand\W{4}$

$ grex -drsw <INPUT>
^\w\s♥{3}\s\d(\d\s\w{3}\s){2}💩{2}\.$

$ grex -drswW <INPUT>
^\w\s\W{3}\s\d(\d\s\w{3}\s){2}\W{3}$
```                                                                                                                            

## 5. <a name="how-to-build"></a> How to build? <sup>[Top ▲](#table-of-contents)</sup>

In order to build the source code yourself, you need the 
[stable Rust toolchain](https://www.rust-lang.org/tools/install) installed on your machine 
so that [*cargo*](https://doc.rust-lang.org/cargo/), the Rust package manager is available.

```
git clone https://github.com/pemistahl/grex.git
cd grex
cargo build
```

The source code is accompanied by an extensive test suite consisting of unit tests, integration 
tests and property tests. For running the unit and integration tests, simply say:

```
cargo test
```

Property tests are disabled by default with the `#[ignore]` annotation because they are 
very long-running. They are used for automatically generating test cases for regular 
expression conversion. If a test case is found that produces a wrong conversion, it is 
shrinked to the shortest test case possible that still produces a wrong result. 
This is a very useful tool for finding bugs. If you want to run these tests, say:

```
cargo test -- --ignored
```

## 6. <a name="how-does-it-work"></a> How does it work? <sup>[Top ▲](#table-of-contents)</sup>

1. A [deterministic finite automaton](https://en.wikipedia.org/wiki/Deterministic_finite_automaton) (DFA) is created from the input strings.

2. The number of states and transitions between states in the DFA is reduced by applying [Hopcroft's DFA minimization algorithm](https://en.wikipedia.org/wiki/DFA_minimization#Hopcroft.27s_algorithm).

3. The minimized DFA is expressed as a system of linear equations which are solved with [Brzozowski's algebraic method](http://cs.stackexchange.com/questions/2016/how-to-convert-finite-automata-to-regular-expressions#2392), resulting in the final regular expression.

## 7. <a name="contribution"></a> Do you want to contribute? <sup>[Top ▲](#table-of-contents)</sup>

In case you want to contribute something to *grex* even though it's in a very early stage of development, then I encourage you to do so nevertheless. Do you have ideas for cool features? Or have you found any bugs so far? Feel free to open an issue or send a pull request. It's very much appreciated. :-)
