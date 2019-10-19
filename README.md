# grex

[![Build Status](https://travis-ci.org/pemistahl/grex.svg?branch=master)](https://travis-ci.org/pemistahl/grex)
[![codecov](https://codecov.io/gh/pemistahl/grex/branch/master/graph/badge.svg)](https://codecov.io/gh/pemistahl/grex)
[![Crates.io](https://img.shields.io/crates/v/grex.svg)](https://crates.io/crates/grex)
[![license](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](https://www.apache.org/licenses/LICENSE-2.0)

#### Download

Pre-compiled 64-bit executables are provided here for Linux, macOS and Windows. Older releases can be found on the [release page](https://github.com/pemistahl/grex/releases).

[![Linux Download](https://img.shields.io/badge/Linux%20Download-v0.2.0-blue?logo=Linux)](https://github.com/pemistahl/grex/releases/download/v0.2.0/grex-v0.2.0-x86_64-unknown-linux-musl.tar.gz)
[![MacOS Download](https://img.shields.io/badge/macOS%20Download-v0.2.0-blue?logo=Apple)](https://github.com/pemistahl/grex/releases/download/v0.2.0/grex-v0.2.0-x86_64-apple-darwin.tar.gz)
[![Windows Download](https://img.shields.io/badge/Windows%20Download-v0.2.0-blue?logo=Windows)](https://github.com/pemistahl/grex/releases/download/v0.2.0/grex-v0.2.0-x86_64-pc-windows-msvc.zip)

## What does this tool do?

*grex* is a small command-line utility that is meant to simplify the often complicated and tedious task of creating regular expressions. It does so by automatically generating regular expressions from user-provided input strings.

This project has started as a Rust port of the JavaScript tool [*regexgen*](https://github.com/devongovett/regexgen) written by [Devon Govett](https://github.com/devongovett). Although a lot of further useful features could be added to it, its development was apparently ceased several years ago. The plan is now to add these new features to *grex* as Rust really shines when it comes to command-line tools. *grex* offers all features that *regexgen* provides, and a little bit more already.

In the current version, *grex* generates the most specific regular expression possible which exactly matches the given input only and nothing else. This is and always will be the default setting. In later releases, the tool will be able to create more generalized expressions by using wildcards. These generalization features will have to be explicitly enabled by respective command-line flags and options.

## Current Features
- literals
- character classes
- detection of common prefixes and suffixes
- alternation using `|` operator
- optionality using `?` quantifier
- concatenation of all of the former
- reading input strings from the command-line or from a file

## How to install?

You can download the self-contained executable for your platform above and put it in a place of your choice. *grex* is also hosted on [crates.io](https://crates.io/crates/grex), the official Rust package registry. If you are a Rust developer and already have the Rust toolchain installed, the easiest way to download and install is by compiling from source using [*cargo*](https://doc.rust-lang.org/cargo/), the Rust package manager:

```
cargo install grex
```

Support for other popular package managers will be added soon.

## How to use?
```
$ grex -h
grex 0.2.0
Peter M. Stahl <pemistahl@gmail.com>
grex generates regular expressions from user-provided input strings.

USAGE:
    grex <INPUT>... --file <FILE>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --file <FILE>    Reads input strings from a file with each string on a separate line

ARGS:
    <INPUT>...    One or more strings separated by blank space 
```

The quickest way is to provide input strings on the command line, separated by spaces:

```
$ grex a ab abc
^a(bc?)?$
```

If an input string contains space characters, it needs to be surrounded by quotation marks:

```
$ grex "I ♥ cake" "I ♥ cookies"
^I ♥ c(ookies|ake)$
```

Every generated regular expression is surrounded by the anchors `^` and `$` so that it does not accidently match substrings. Unicode symbols which do not belong to the ASCII character set are not escaped by default because programming languages use different notations for unicode escape sequences. It is planned to support different escape sequence notations in the future by providing command-line options.

*grex* does not operate on scalar values but on grapheme clusters. If a grapheme cluster consists of more than one scalar value, then this is considered correctly. The letter `y̆` in the following example consists of the unicode symbols U+0079 (Latin Small Letter Y) and U+0306 (Combining Breve). Therefore, it cannot be part of the character class as this is for single characters only.

```
$ grex y̆ a z
^[az]|y̆$
```

Input strings can be read from a file as well. Every file must be encoded as UTF-8 and every input string must be on a separate line:

```
$ grex -f my-input-file.txt
```

Some more examples:

```
$ grex a b c
^[a-c]$

$ grex a c d e f
^[ac-f]$

$ grex a b x de
^de|[abx]$

$ grex 1 3 4 5 6
^[13-6]$

$ grex a b bc
^bc?|a$

$ grex a b bcd
^b(cd)?|a$

$ grex abx cdx
^(ab|cd)x$

$ grex 3.5 4.5 4,5
^3\.5|4[,.]5$
```

## How does it work?

1. A [deterministic finite automaton](https://en.wikipedia.org/wiki/Deterministic_finite_automaton) (DFA) is created from the input strings.

2. The number of states and transitions between states in the DFA is reduced by applying [Hopcroft's DFA minimization algorithm](https://en.wikipedia.org/wiki/DFA_minimization#Hopcroft.27s_algorithm).

3. The minimized DFA is expressed as a system of linear equations which are solved with [Brzozowski's algebraic method](http://cs.stackexchange.com/questions/2016/how-to-convert-finite-automata-to-regular-expressions#2392), resulting in the final regular expression.

## Do you want to contribute?

In case you want to contribute something to *grex* even though it's in a very early stage of development, then I encourage you to do so nevertheless. Do you have ideas for cool features? Or have you found any bugs so far? Feel free to open an issue or send a pull request. It's very much appreciated. :-)