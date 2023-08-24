<div align="center">

![grex](https://raw.githubusercontent.com/pemistahl/grex/main/logo.png)

<br>

[![build status](https://github.com/pemistahl/grex/actions/workflows/python-build.yml/badge.svg)](https://github.com/pemistahl/grex/actions/workflows/python-build.yml)
[![codecov](https://codecov.io/gh/pemistahl/grex/branch/main/graph/badge.svg)](https://codecov.io/gh/pemistahl/grex)
[![demo](https://img.shields.io/badge/-Demo%20Website-orange?logo=HTML5&labelColor=white)](https://pemistahl.github.io/grex-js/)
![supported Python versions](https://img.shields.io/badge/Python-%3E%3D%203.8-blue?logo=Python&logoColor=yellow)
[![pypi](https://img.shields.io/badge/PYPI-v1.0.0-blue?logo=PyPI&logoColor=yellow)](https://pypi.org/project/grex)
[![license](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](https://www.apache.org/licenses/LICENSE-2.0)
</div>

<br>

## 1. What does this library do?

*grex* is a library that is meant to simplify the often complicated and tedious
task of creating regular expressions. It does so by automatically generating a
single regular expression from user-provided test cases. The resulting
expression is guaranteed to match the test cases which it was generated from.

This project has started as a [Rust port](https://github.com/pemistahl/grex) of
the JavaScript tool [*regexgen*](https://github.com/devongovett/regexgen)
written by [Devon Govett](https://github.com/devongovett). Although a lot of
further useful features could be added to it, its development was apparently
ceased several years ago. The Rust library offers new features and extended
Unicode support. With the help of [PyO3](https://github.com/PyO3/pyo3) and 
[Maturin](https://github.com/PyO3/maturin), the library has been compiled to a 
Python extension module so that it can be used within any Python software as well.

The philosophy of this project is to generate the most specific regular expression
possible by default which exactly matches the given input only and nothing else.
With the use of preprocessing methods, more generalized expressions can be created.

The produced expressions are [Perl-compatible regular expressions](https://www.pcre.org) which are also
compatible with the [regular expression module](https://docs.python.org/3/library/re.html) in Python's 
standard library.

There is a [demo website](https://pemistahl.github.io/grex-js/) available where you can give grex a try.

![demo website](https://raw.githubusercontent.com/pemistahl/grex/main/website.jpg)

## 2. Do I still need to learn to write regexes then?

**Definitely, yes!** Using the standard settings, *grex* produces a regular expression that is guaranteed
to match only the test cases given as input and nothing else. However, if the conversion to shorthand
character classes such as `\w` is enabled, the resulting regex matches a much wider scope of test cases.
Knowledge about the consequences of this conversion is essential for finding a correct regular expression
for your business domain.

*grex* uses an algorithm that tries to find the shortest possible regex for the given test cases.
Very often though, the resulting expression is still longer or more complex than it needs to be.
In such cases, a more compact or elegant regex can be created only by hand.
Also, every regular expression engine has different built-in optimizations. *grex* does not know anything
about those and therefore cannot optimize its regexes for a specific engine.

**So, please learn how to write regular expressions!** The currently best use case for *grex* is to find
an initial correct regex which should be inspected by hand if further optimizations are possible.

## 3. Current Features

- literals
- character classes
- detection of common prefixes and suffixes
- detection of repeated substrings and conversion to `{min,max}` quantifier notation
- alternation using `|` operator
- optionality using `?` quantifier
- escaping of non-ascii characters, with optional conversion of astral code points to surrogate pairs
- case-sensitive or case-insensitive matching
- capturing or non-capturing groups
- optional anchors `^` and `$`
- fully compliant to [Unicode Standard 15.0](https://unicode.org/versions/Unicode15.0.0)
- correctly handles graphemes consisting of multiple Unicode symbols
- produces more readable expressions indented on multiple using optional verbose mode
- optional syntax highlighting for nicer output in supported terminals

## 4. How to install?

*grex* is available in the [Python Package Index](https://pypi.org/project/grex) and can be installed with:

```
pip install grex
```

The current version 1.0.0 corresponds to the latest version 1.4.4 of the Rust
library and command-line tool.

## 5. How to use?

This library contains a single class named `RegExpBuilder` that can be imported like so:

```python
from grex import RegExpBuilder
```

### 5.1 Default settings

```python
pattern = RegExpBuilder.from_test_cases(["a", "aa", "aaa"]).build()
assert pattern == "^a(?:aa?)?$"
```

### 5.2 Convert to character classes

```python
pattern = (RegExpBuilder.from_test_cases(["a", "aa", "123"])
    .with_conversion_of_digits()
    .with_conversion_of_words()
    .build())
assert pattern == "^(?:\\d\\d\\d|\\w(?:\\w)?)$"
```

### 5.3 Convert repeated substrings

```python
pattern = (RegExpBuilder.from_test_cases(["aa", "bcbc", "defdefdef"])
    .with_conversion_of_repetitions()
    .build())
assert pattern == "^(?:a{2}|(?:bc){2}|(?:def){3})$"
```

By default, *grex* converts each substring this way which is at least a single character long
and which is subsequently repeated at least once. You can customize these two parameters if you like.

In the following example, the test case `aa` is not converted to `a{2}` because the repeated substring
`a` has a length of 1, but the minimum substring length has been set to 2.

```python
pattern = (RegExpBuilder.from_test_cases(["aa", "bcbc", "defdefdef"])
    .with_conversion_of_repetitions()
    .with_minimum_substring_length(2)
    .build())
assert pattern == "^(?:aa|(?:bc){2}|(?:def){3})$"
```

Setting a minimum number of 2 repetitions in the next example, only the test case `defdefdef` will be
converted because it is the only one that is repeated twice.

```python
pattern = (RegExpBuilder.from_test_cases(["aa", "bcbc", "defdefdef"])
    .with_conversion_of_repetitions()
    .with_minimum_repetitions(2)
    .build())
assert pattern == "^(?:bcbc|aa|(?:def){3})$"
```

### 5.4 Escape non-ascii characters

```python
pattern = (RegExpBuilder.from_test_cases(["You smell like 💩."])
    .with_escaping_of_non_ascii_chars(use_surrogate_pairs=False)
    .build())
assert pattern == "^You smell like \\U0001f4a9\\.$"
```

Old versions of JavaScript do not support unicode escape sequences for the astral code planes
(range `U+010000` to `U+10FFFF`). In order to support these symbols in JavaScript regular
expressions, the conversion to surrogate pairs is necessary. More information on that matter
can be found [here](https://mathiasbynens.be/notes/javascript-unicode).

```python
pattern = (RegExpBuilder.from_test_cases(["You smell like 💩."])
    .with_escaping_of_non_ascii_chars(use_surrogate_pairs=True)
    .build())
assert pattern == "^You smell like \\ud83d\\udca9\\.$"
```

### 5.5 Case-insensitive matching

The regular expressions that *grex* generates are case-sensitive by default.
Case-insensitive matching can be enabled like so:

```python
pattern = (RegExpBuilder.from_test_cases(["big", "BIGGER"])
    .with_case_insensitive_matching()
    .build())
assert pattern == "(?i)^big(?:ger)?$"
```

### 5.6 Capturing Groups

Non-capturing groups are used by default.
Extending the previous example, you can switch to capturing groups instead.

```python
pattern = (RegExpBuilder.from_test_cases(["big", "BIGGER"])
    .with_case_insensitive_matching()
    .with_capturing_groups()
    .build())
assert pattern == "(?i)^big(ger)?$"
```

### 5.7 Verbose mode

If you find the generated regular expression hard to read, you can enable verbose mode.
The expression is then put on multiple lines and indented to make it more pleasant to the eyes.

```python
import inspect

pattern = (RegExpBuilder.from_test_cases(["a", "b", "bcd"])
    .with_verbose_mode()
    .build())

assert pattern == inspect.cleandoc("""
    (?x)
    ^
      (?:
        b
        (?:
          cd
        )?
        |
        a
      )
    $
    """
)
```

### 5.8 Disable anchors

By default, the anchors `^` and `$` are put around every generated regular expression in order
to ensure that it matches only the test cases given as input. Often enough, however, it is
desired to use the generated pattern as part of a larger one. For this purpose, the anchors
can be disabled, either separately or both of them.

```python
pattern = (RegExpBuilder.from_test_cases(["a", "aa", "aaa"])
    .without_anchors()
    .build())
assert pattern == "a(?:aa?)?"
```

## 6. How to build?

In order to build the source code yourself, you need the
[stable Rust toolchain](https://www.rust-lang.org/tools/install) installed on your machine
so that [*cargo*](https://doc.rust-lang.org/cargo/), the Rust package manager is available.

```shell
git clone https://github.com/pemistahl/grex.git
cd grex
cargo build
```

To build the Python extension module, create a virtual environment and install [Maturin](https://github.com/PyO3/maturin).

```shell
python -m venv /path/to/virtual/environment
source /path/to/virtual/environment/bin/activate
pip install maturin
maturin build
```

The Rust source code is accompanied by an extensive test suite consisting of unit tests, integration
tests and property tests. For running them, simply say:

```shell
cargo test
```

Additional Python tests can be run after installing pytest which is an optional dependency:

```shell
maturin develop --extras=test
pytest tests/python/test_grex.py
```
