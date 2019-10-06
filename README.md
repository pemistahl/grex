# grex

[![Build Status](https://travis-ci.org/pemistahl/grex.svg?branch=master)](https://travis-ci.org/pemistahl/grex)
[![codecov](https://codecov.io/gh/pemistahl/grex/branch/master/graph/badge.svg)](https://codecov.io/gh/pemistahl/grex)
[![Crates.io](https://img.shields.io/crates/v/grex.svg)](https://crates.io/crates/grex)
[![license](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](https://www.apache.org/licenses/LICENSE-2.0)

*grex* is a small command-line utility that is meant to simplify the often complicated and tedious task of creating regular expressions. It does so by automatically generating regular expressions from user-provided input strings that match the generated expression.

In its very early stage, this tool is a Rust port of [*regexgen*](https://github.com/devongovett/regexgen) which has been implemented in JavaScript and runs in a Node.js environment. The plan is, however, to add much more functionality to *grex* than *regexgen* provides. The development of the latter was ceased more than two years ago. Compared to *regexgen*, *grex* is currently lacking support for character classes. This feature will be added in the next version.

## Features
- literals
- detection of common prefixes and suffixes
- alternation using `|` operator
- repetition using `?` quantifier
- concatenation of all of the former

## How to use
```
$ grex -h
grex 0.1.0
Peter M. Stahl <pemistahl@gmail.com>
grex generates regular expressions that match user-provided input strings.

USAGE:
    grex <input>...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <input>... 
``` 
### Some examples
```
$ grex abc def
abc|def

$ grex a ab abc
a(bc?)?

$ grex thankful thoughtful
th(ought|ank)ful

$ grex 2.0-3.5 2.5-6.0
2\.(0\-3\.5|5\-6\.0)
```