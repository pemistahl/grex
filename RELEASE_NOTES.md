## grex 1.4.4 (released on 24 Aug 2023)

### Bug Fixes
- The Python release workflow was incorrect as it produced too many wheels for upload.
  This has been fixed.

## grex 1.4.3 (released on 24 Aug 2023)

### Features
- Python bindings are now available for the library. Use grex within any Python software. (#172)

### Changes
- All dependencies have been updated to their latest versions.

## grex 1.4.2 (released on 26 Jul 2023)

### Improvements
- All characters from the current Unicode standard 15.0 are now fully supported. (#128)
- A proper exit code is now returned if the provided user input cannot be handled by the CLI.
  Big thanks to @spenserblack for the respective pull request. (#165)

### Changes
- It is not possible anymore to call `RegExpBuilder.with_syntax_highlighting()` in the library
  as it only makes sense for the CLI.
- The dependency `atty` has been removed in favor of `std::io::IsTerminal` in Rust >= 1.70.0.
  As a result, Rust >= 1.70.0 is now needed to compile the CLI. 
- All remaining dependencies have been updated to their latest versions.

### Bug Fixes
- Several bugs have been fixed that caused incorrect expressions to be generated in rare cases.

## grex 1.4.1 (released on 21 Oct 2022)

### Changes
- `clap` has been updated to version 4.0. The help output by `grex -h` now looks a little different.

### Bug Fixes
- A bug in the grapheme segmentation was fixed that caused test cases which contain backslashes to produce
  incorrect regular expressions.

## grex 1.4.0 (released on 26 Jul 2022)

### Features
- The library can now be compiled to WebAssembly and be used in any JavaScript project. (#82)
- The supported character set for regular expression generation has been updated to the current Unicode Standard 14.0.
- `structopt` has been replaced with `clap` providing much nicer help output for the command-line tool.

### Improvements
- The regular expression generation performance has been significantly improved, especially for generating very long
  expressions from a large set of test cases. This has been accomplished by reducing the number of memory allocations,
  removing deprecated code and applying several minor optimizations.

### Bug Fixes
- Several bugs have been fixed that caused incorrect expressions to be generated in rare cases.

## grex 1.3.0 (released on 15 Sep 2021)

### Features
- anchors can now be disabled so that the generated expression can be used as part of a larger one (#30)
- the command-line tool can now be used within Unix pipelines (#45)

### Changes
- Additional methods have been added to `RegExpBuilder` in order to replace the enum `Feature` and make the library API more consistent. (#47)

### Bug Fixes
- Under rare circumstances, the conversion of repetitions did not work. This has been fixed. (#36)

## grex 1.2.0 (released on 28 Mar 2021)

### Features
- verbose mode is now supported with the `--verbose` flag to produce regular expressions which are easier to read (#17)

## grex 1.1.0 (released on 17 Apr 2020)

### Features
- case-insensitive matching regexes are now supported with the `--ignore-case` command-line flag or with `Feature::CaseInsensitivity` in the library (#23)
- non-capturing groups are now the default; capturing groups can be enabled with the `--capture-groups` command-line flag or with `Feature::CapturingGroup` in the library (#15)
- a lower bound for the conversion of repeated substrings can now be set by specifying `--min-repetitions` and `--min-substring-length` or using the library methods `RegExpBuilder.with_minimum_repetitions()` and `RegExpBuilder.with_minimum_substring_length()` (#10)
- test cases can now be passed from a file within the library as well using `RegExpBuilder::from_file()` (#13)

### Changes

- the rules for the conversion of test cases to shorthand character classes have been updated to be compliant to the newest Unicode Standard 13.0 (#21)
- the dependency on the unmaintained linked-list crate has been removed (#24)

### Bug Fixes

- test cases starting with a hyphen are now correctly parsed on the command-line (#12)
- the common substring detection algorithm now uses optionality expressions where possible instead of redundant union operations (#22)

### Test Coverage
- new unit tests, integration tests and property tests have been added

## grex 1.0.0 (released on 02 Feb 2020)

### Features
- conversion to character classes `\d`, `\D`, `\s`, `\S`, `\w`, `\W` is now supported
- repetition detection now works with arbitrarily nested expressions. Input strings such as `aaabaaab` which were previously converted to `^(aaab){2}$` are now converted to `^(a{3}b){2}$`.
- optional syntax highlighting for the produced regular expressions can now be enabled using the `--colorize` command-line flag or with the library method `RegExpBuilder.with_syntax_highlighting()`

### Test Coverage
- new unit tests, integration tests and property tests have been added

## grex 0.3.2 (released on 12 Jan 2020)

### Test Coverage
- new property tests have been added that revealed new bugs

### Bug Fixes
- entire rewrite of the repetition detection algorithm
- the former algorithm produced wrong regular expressions or even panicked for certain test cases

## grex 0.3.1 (released on 06 Jan 2020)

### Test Coverage
- property tests have been added using the [proptest](https://crates.io/crates/proptest) crate 
- big thanks go to [Christophe Biocca](https://github.com/christophebiocca) for pointing me to the concept of property tests in the first place and for writing an initial implementation of these tests

### Bug Fixes
- some regular expression specific characters were not escaped correctly in the generated expression
- expressions consisting of a single alternation such as `^(abc|xyz)$` were missing the outer parentheses. This caused an erroneous match of strings such as `abc123` or `456xyz` because of precedence rules.
- the created DFA was wrong for repetition conversion in some corner cases. The input `a, aa, aaa, aaaa, aaab` previously returned the expression `^a{1,4}b?$` which erroneously matches `aaaab`. Now the correct expression `^(a{3}b|a{1,4})$` is returned.

### Documentation
- some minor documentation updates

## grex 0.3.0 (released on 24 Dec 2019)

### Features
- *grex* is now also available as a library
- escaping of non-ascii characters is now supported with the `-e` flag
- astral code points can be converted to surrogate with the `--with-surrogates` flag
- repeated non-overlapping substrings can be converted to `{min,max}` quantifier notation using the `-r` flag

### Bug Fixes
- many many many bug fixes :-O

## grex 0.2.0 (released on 20 Oct 2019)

### Features
- character classes are now supported
- input strings can now be read from a text file

### Changes
- unicode characters are not escaped anymore by default
- the performance of the DFA minimization algorithm has been improved for large DFAs
- regular expressions are now always surrounded by anchors `^` and `$`

### Bug Fixes
- fixed a bug that caused a panic when giving an empty string as input

## grex 0.1.0 (released on 06 Oct 2019)

This is the very first release of *grex*. It aims at simplifying the construction of regular expressions based on matching example input.

### Features
- literals
- detection of common prefixes and suffixes
- alternation using `|` operator
- optionality using `?` quantifier
- concatenation of all of the former
