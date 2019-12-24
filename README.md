![grex](logo.png)

[![Build Status](https://travis-ci.org/pemistahl/grex.svg?branch=master)](https://travis-ci.org/pemistahl/grex)
[![codecov](https://codecov.io/gh/pemistahl/grex/branch/master/graph/badge.svg)](https://codecov.io/gh/pemistahl/grex)
[![Crates.io](https://img.shields.io/crates/v/grex.svg)](https://crates.io/crates/grex)
[![Docs.rs](https://docs.rs/grex/badge.svg)](https://docs.rs/grex)
[![Downloads](https://img.shields.io/crates/d/grex.svg)](https://crates.io/crates/grex)
[![license](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](https://www.apache.org/licenses/LICENSE-2.0)
---
[![Linux Download](https://img.shields.io/badge/Linux%20Download-v0.3.0-blue?logo=Linux)](https://github.com/pemistahl/grex/releases/download/v0.3.0/grex-v0.3.0-x86_64-unknown-linux-musl.tar.gz)
[![MacOS Download](https://img.shields.io/badge/macOS%20Download-v0.3.0-blue?logo=Apple)](https://github.com/pemistahl/grex/releases/download/v0.3.0/grex-v0.3.0-x86_64-apple-darwin.tar.gz)
[![Windows Download](https://img.shields.io/badge/Windows%20Download-v0.3.0-blue?logo=Windows)](https://github.com/pemistahl/grex/releases/download/v0.3.0/grex-v0.3.0-x86_64-pc-windows-msvc.zip)
---

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
5. [How does it work?](#how-does-it-work)
6. [Do you want to contribute?](#contribution)
 

## 1. <a name="what-does-tool-do"></a> What does this tool do? <sup>[Top ▲](#table-of-contents)</sup>

*grex* is a library as well as a command-line utility that is meant to simplify the often complicated and tedious task of creating regular expressions. It does so by automatically generating regular expressions from user-provided test cases.

This project has started as a Rust port of the JavaScript tool [*regexgen*](https://github.com/devongovett/regexgen) written by [Devon Govett](https://github.com/devongovett). Although a lot of further useful features could be added to it, its development was apparently ceased several years ago. The plan is now to add these new features to *grex* as Rust really shines when it comes to command-line tools. *grex* offers all features that *regexgen* provides, and more.

The philosophy of this project is to generate the most specific regular expression possible by default which exactly matches the given input only and nothing else. With the use of command-line flags (in the CLI tool) or preprocessing methods (in the library), more generalized expressions can be created.

## 2. <a name="current-features"></a> Current Features <sup>[Top ▲](#table-of-contents)</sup>
- literals
- character classes
- detection of common prefixes and suffixes
- detection of repeated substrings and conversion to `{min,max}` quantifier notation
- alternation using `|` operator
- optionality using `?` quantifier
- escaping of non-ascii characters, with optional conversion of astral code points to surrogate pairs
- concatenation of all of the former
- reading input strings from the command-line or from a file

## 3. <a name="how-to-install"></a> How to install? <sup>[Top ▲](#table-of-contents)</sup>

### 3.1 <a name="how-to-install-cli"></a> The command-line tool <sup>[Top ▲](#table-of-contents)</sup>

Pre-compiled 64-Bit binaries are available within the package managers [Scoop](https://scoop.sh) (for Windows) and [Homebrew](https://brew.sh) (for macOS and Linux).

#### Scoop
```
scoop install grex
```

#### Homebrew
```
brew tap pemistahl/formulas
brew install grex
```

Alternatively, you can download the self-contained executable for your platform above and put it in a place of your choice. *grex* is also hosted on [crates.io](https://crates.io/crates/grex), the official Rust package registry. If you are a Rust developer and already have the Rust toolchain installed, you can install by compiling from source using [*cargo*](https://doc.rust-lang.org/cargo/), the Rust package manager:

```
cargo install grex
```

### 3.2 <a name="how-to-install-library"></a> The library <sup>[Top ▲](#table-of-contents)</sup>

In order to use *grex* as a library, simply add it as a dependency to your `Cargo.toml` file:

```toml
[dependencies]
grex = "0.3.0"
```

## 4. <a name="how-to-use"></a> How to use? <sup>[Top ▲](#table-of-contents)</sup>

Every generated regular expression is surrounded by the anchors `^` and `$` so that it does not accidently match substrings.

### 4.1 <a name="how-to-use-cli"></a> The command-line tool <sup>[Top ▲](#table-of-contents)</sup>

```
$ grex --help
grex 0.3.0
Peter M. Stahl <pemistahl@gmail.com>
grex generates regular expressions from user-provided test cases.

USAGE:
    grex [FLAGS] <INPUT>... --file <FILE>

FLAGS:
    -r, --convert-repetitions    
            Detects repeated non-overlapping substrings and
            converts them to {min,max} quantifier notation
    
    -e, --escape                 
            Replaces all non-ASCII characters with unicode escape sequences

        --with-surrogates        
            Converts astral code points to surrogate pairs if --escape is set

    -h, --help                   
            Prints help information

    -v, --version                
            Prints version information

OPTIONS:
    -f, --file <FILE>    
            Reads test cases separated by newline characters from a file

ARGS:
    <INPUT>...    
            One or more test cases separated by blank space 
```

Input strings can be read from the command line or from a file. Every file must be encoded as UTF-8 and every input string must be on a separate line:
                                                                                                                             
```
$ grex -f my-input-file.txt
```

### 4.2 <a name="how-to-use-library"></a> The library <sup>[Top ▲](#table-of-contents)</sup>

#### Default settings

```rust
let regexp = grex::RegExpBuilder::from(&["a", "aa", "aaa"]).build();
assert_eq!(regexp, "^a(aa?)?$");
```

#### Convert repeated substrings

```rust
let regexp = grex::RegExpBuilder::from(&["a", "aa", "aaa"])
    .with_converted_repetitions()
    .build();
assert_eq!(regexp, "^a{1,3}$");
```

#### Escape non-ascii characters

```rust
let regexp = grex::RegExpBuilder::from(&["You smell like 💩."])
    .with_escaped_non_ascii_chars(false)
    .build();
assert_eq!(regexp, "^You smell like \\u{1f4a9}\\.$");
```

#### Escape astral code points using surrogate pairs

```rust
let regexp = grex::RegExpBuilder::from(&["You smell like 💩."])
    .with_escaped_non_ascii_chars(true)
    .build();
assert_eq!(regexp, "^You smell like \\u{d83d}\\u{dca9}\\.$");
```

#### Combine multiple features

```rust
let regexp = grex::RegExpBuilder::from(&["You smell like 💩💩💩."])
    .with_converted_repetitions()
    .with_escaped_non_ascii_chars(false)
    .build();
assert_eq!(regexp, "^You smel{2} like \\u{1f4a9}{3}\\.$");
```

### 4.3 <a name="examples"></a> Examples <sup>[Top ▲](#table-of-contents)</sup>

The following table showcases what *grex* can do:                                                                                                                           

| Input | Output | Note |
| ----- | ------ | ---- |
| `$ grex a b c` | `^[a-c]$` | |
| `$ grex a c d e f` | `^[ac-f]$` | |
| `$ grex 1 3 4 5 6` | `^[13-6]$` | |
| `$ grex a b x de` | <code>^de&#124;[abx]$</code> | |
| `$ grex a b bc` | <code>^bc?&#124;a$</code> | |
| `$ grex a aa aaa` | `^a(aa?)?$` | |
| `$ grex a ab abc` | `^a(bc?)?$` | |
| `$ grex 3.5 4.5 4,5` | <code>^3\\.5&#124;4[,.]5$</code> | |
| `$ grex [a-z]` | `^\[a\-z\]$` | Regex syntax characters are escaped. | 
| `$ grex y̆ a z` | <code>^[az]&#124;y̆$</code> | Grapheme `y̆` consists of two unicode symbols:<br>`U+0079` (Latin Small Letter Y)<br>`U+0306` (Combining Breve).<br>This is why it is not part of<br>the character class. |
| `$ grex "I ♥ cake" "I ♥ cookies"` | <code>^I ♥ c(ookies&#124;ake)$</code> | Input containing blank space must be<br>surrounded by quotation marks. |
| `$ grex "I \u{2665} cake"` | `^I ♥ cake$` | Unicode escape sequences are converted<br>back to the original unicode symbol. | 
| `$ grex -r aaa` | `^a{3}$` | |
| `$ grex -r abababa` | `^(ab){3}a$` | |
| `$ grex -r aababab` | `^a(ab){3}$` | |
| `$ grex -r abababaa` | `^(ab){3}a{2}$` | |
| `$ grex -r a aa aaa` | `^a{1,3}$` | |
| `$ grex -r b ba baa baaa` | `^b(a{1,3})?$` | |
| `$ grex -r b ba baa baaaa` | <code>^b(a{1,2}&#124;a{4})?$</code> | | 
| `$ grex -r xy̆y̆z xy̆y̆y̆z` | `^x(y̆){2,3}z$` | The parentheses are needed because<br>`y̆` consists of two unicode symbols. | 
| `$ grex -r xy̆y̆z xy̆y̆y̆y̆z` | <code>^x((y̆){2}&#124;(y̆){4})z$</code> | |
| `$ grex -r zyxx yxx` | `^z?yx{2}$` | | 
| `$ grex -r 4.5 44.5 44.55 4.55 ` | `^4{1,2}\.5{1,2}$` | | 
| `$ grex -r "I ♥♥ cake"` | `^I ♥{2} cake$` | |
| `$ grex -r "I \u{2665}\u{2665} cake"` | `^I ♥{2} cake$` | | 
| `$ grex -e "I ♥♥ you."` | `^I \u{2665}\u{2665} you\.$` | |
| `$ grex -e -r "I ♥♥ you."` | `^I \u{2665}{2} you\.$` | |
| `$ grex -e "You smell like 💩💩."` | `^You smell like \u{1f4a9}\u{1f4a9}\.$` | |
| `$ grex -e -r "You smell like 💩💩."` | `^You smell like \u{1f4a9}{2}\.$` | |
| `$ grex -e -r --with-surrogates "You smell like 💩💩."` | `^You smel{2} like (\u{d83d}\u{dca9}){2}\.$` | For languages such as older<br>JavaScript versions not supporting<br>astral codepoints (`U+010000` to `U+10FFFF`),<br>conversion to surrogate pairs is possible.<br>More info about this issue can be found [here](https://mathiasbynens.be/notes/javascript-unicode). |  

## 5. <a name="how-does-it-work"></a> How does it work? <sup>[Top ▲](#table-of-contents)</sup>

1. A [deterministic finite automaton](https://en.wikipedia.org/wiki/Deterministic_finite_automaton) (DFA) is created from the input strings.

2. The number of states and transitions between states in the DFA is reduced by applying [Hopcroft's DFA minimization algorithm](https://en.wikipedia.org/wiki/DFA_minimization#Hopcroft.27s_algorithm).

3. The minimized DFA is expressed as a system of linear equations which are solved with [Brzozowski's algebraic method](http://cs.stackexchange.com/questions/2016/how-to-convert-finite-automata-to-regular-expressions#2392), resulting in the final regular expression.

## 6. <a name="contribution"></a> Do you want to contribute? <sup>[Top ▲](#table-of-contents)</sup>

In case you want to contribute something to *grex* even though it's in a very early stage of development, then I encourage you to do so nevertheless. Do you have ideas for cool features? Or have you found any bugs so far? Feel free to open an issue or send a pull request. It's very much appreciated. :-)