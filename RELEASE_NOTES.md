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