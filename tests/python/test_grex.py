#
# Copyright © 2019-today Peter M. Stahl pemistahl@gmail.com
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
# http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either expressed or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

import inspect
import pytest
import re

from grex import RegExpBuilder


@pytest.mark.parametrize(
    "test_cases,expected_pattern",
    [
        pytest.param(["abc", "abd", "abe"], "^ab[c-e]$"),
    ]
)
def test_default_settings(test_cases, expected_pattern):
    pattern = RegExpBuilder.from_test_cases(test_cases).build()
    assert pattern == expected_pattern
    for test_case in test_cases:
        assert re.match(pattern, test_case)


@pytest.mark.parametrize(
    "test_cases,expected_pattern",
    [
        pytest.param(["My ♥ and 💩 is yours."], "^My \\u2665 and \\U0001f4a9 is yours\\.$"),
    ]
)
def test_escaping(test_cases, expected_pattern):
    pattern = (RegExpBuilder.from_test_cases(test_cases)
               .with_escaping_of_non_ascii_chars(use_surrogate_pairs=False)
               .build())
    assert pattern == expected_pattern
    for test_case in test_cases:
        assert re.match(pattern, test_case)


@pytest.mark.parametrize(
    "test_cases,expected_pattern",
    [
        pytest.param(["My ♥ and 💩 is yours."], "^My \\u2665 and \\ud83d\\udca9 is yours\\.$"),
    ]
)
def test_escaping_with_surrogate_pairs(test_cases, expected_pattern):
    pattern = (RegExpBuilder.from_test_cases(test_cases)
               .with_escaping_of_non_ascii_chars(use_surrogate_pairs=True)
               .build())
    assert pattern == expected_pattern
    # module re does not support matching surrogate pairs


@pytest.mark.parametrize(
    "test_cases,expected_pattern",
    [
        pytest.param(["efgh", "abcxy", "abcw"], "^(abc(xy|w)|efgh)$"),
    ]
)
def test_capturing_groups(test_cases, expected_pattern):
    pattern = (RegExpBuilder.from_test_cases(test_cases)
               .with_capturing_groups()
               .build())
    assert pattern == expected_pattern
    for test_case in test_cases:
        assert re.match(pattern, test_case)


@pytest.mark.parametrize(
    "test_cases,expected_pattern",
    [
        pytest.param(["efgh", "abcxy", "abcw"], "(?:abc(?:xy|w)|efgh)"),
    ]
)
def test_without_anchors(test_cases, expected_pattern):
    pattern = (RegExpBuilder.from_test_cases(test_cases)
               .without_anchors()
               .build())
    assert pattern == expected_pattern
    for test_case in test_cases:
        assert re.match(pattern, test_case)


@pytest.mark.parametrize(
    "test_cases,expected_pattern",
    [
        pytest.param(["ABC", "zBC", "abc", "AbC", "aBc"], "(?i)^[az]bc$"),
    ]
)
def test_case_insensitive_matching(test_cases, expected_pattern):
    pattern = (RegExpBuilder.from_test_cases(test_cases)
               .with_case_insensitive_matching()
               .build())
    assert pattern == expected_pattern
    for test_case in test_cases:
        assert re.match(pattern, test_case)


@pytest.mark.parametrize(
    "test_cases,expected_pattern",
    [
        pytest.param(
            ["[a-z]", "(d,e,f)"],
            inspect.cleandoc("""
                (?x)
                ^
                  (?:
                    \\(d,e,f\\)
                    |
                    \\[a\\-z\\]
                  )
                $
                """)
        ),
    ]
)
def test_verbose_mode(test_cases, expected_pattern):
    pattern = (RegExpBuilder.from_test_cases(test_cases)
               .with_verbose_mode()
               .build())
    assert pattern == expected_pattern
    for test_case in test_cases:
        assert re.match(pattern, test_case)


@pytest.mark.parametrize(
    "test_cases,expected_pattern",
    [
        pytest.param(
            ["Ä@Ö€Ü", "ä@ö€ü", "Ä@ö€Ü", "ä@Ö€ü"],
            inspect.cleandoc("""
                (?ix)
                ^
                  ä@ö€ü
                $
                """)
        )
    ]
)
def test_case_insensitive_matching_and_verbose_mode(test_cases, expected_pattern):
    pattern = (RegExpBuilder.from_test_cases(test_cases)
               .with_case_insensitive_matching()
               .with_verbose_mode()
               .build())
    assert pattern == expected_pattern
    for test_case in test_cases:
        assert re.match(pattern, test_case)


@pytest.mark.parametrize(
    "test_cases,expected_pattern",
    [
        pytest.param(["a", "b\nx\nx", "c"], "^(?:b(?:\\nx){2}|[ac])$"),
    ]
)
def test_conversion_of_repetitions(test_cases, expected_pattern):
    pattern = (RegExpBuilder.from_test_cases(test_cases)
               .with_conversion_of_repetitions()
               .build())
    assert pattern == expected_pattern
    for test_case in test_cases:
        assert re.match(pattern, test_case)


@pytest.mark.parametrize(
    "test_cases,expected_pattern",
    [
        pytest.param(["My ♥♥♥ and 💩💩 is yours."], "^My \\u2665{3} and \\U0001f4a9{2} is yours\\.$"),
    ]
)
def test_escaping_and_conversion_of_repetitions(test_cases, expected_pattern):
    pattern = (RegExpBuilder.from_test_cases(test_cases)
               .with_escaping_of_non_ascii_chars(use_surrogate_pairs=False)
               .with_conversion_of_repetitions()
               .build())
    assert pattern == expected_pattern
    for test_case in test_cases:
        assert re.match(pattern, test_case)


@pytest.mark.parametrize(
    "test_cases,expected_pattern",
    [
        pytest.param(["a1b2c3"], "^a\\db\\dc\\d$"),
    ]
)
def test_conversion_of_digits(test_cases, expected_pattern):
    pattern = (RegExpBuilder.from_test_cases(test_cases)
               .with_conversion_of_digits()
               .build())
    assert pattern == expected_pattern
    for test_case in test_cases:
        assert re.match(pattern, test_case)


@pytest.mark.parametrize(
    "test_cases,expected_pattern",
    [
        pytest.param(["a1b2c3"], "^\\D1\\D2\\D3$"),
    ]
)
def test_conversion_of_non_digits(test_cases, expected_pattern):
    pattern = (RegExpBuilder.from_test_cases(test_cases)
               .with_conversion_of_non_digits()
               .build())
    assert pattern == expected_pattern
    for test_case in test_cases:
        assert re.match(pattern, test_case)


@pytest.mark.parametrize(
    "test_cases,expected_pattern",
    [
        pytest.param(["\n\t", "\r"], "^\\s(?:\\s)?$"),
    ]
)
def test_conversion_of_whitespace(test_cases, expected_pattern):
    pattern = (RegExpBuilder.from_test_cases(test_cases)
               .with_conversion_of_whitespace()
               .build())
    assert pattern == expected_pattern
    for test_case in test_cases:
        assert re.match(pattern, test_case)


@pytest.mark.parametrize(
    "test_cases,expected_pattern",
    [
        pytest.param(["a1 b2 c3"], "^\\S\\S \\S\\S \\S\\S$"),
    ]
)
def test_conversion_of_non_whitespace(test_cases, expected_pattern):
    pattern = (RegExpBuilder.from_test_cases(test_cases)
               .with_conversion_of_non_whitespace()
               .build())
    assert pattern == expected_pattern
    for test_case in test_cases:
        assert re.match(pattern, test_case)


@pytest.mark.parametrize(
    "test_cases,expected_pattern",
    [
        pytest.param(["abc", "1234"], "^\\w\\w\\w(?:\\w)?$"),
    ]
)
def test_conversion_of_words(test_cases, expected_pattern):
    pattern = (RegExpBuilder.from_test_cases(test_cases)
               .with_conversion_of_words()
               .build())
    assert pattern == expected_pattern
    for test_case in test_cases:
        assert re.match(pattern, test_case)


@pytest.mark.parametrize(
    "test_cases,expected_pattern",
    [
        pytest.param(["abc 1234"], "^abc\\W1234$"),
    ]
)
def test_conversion_of_non_words(test_cases, expected_pattern):
    pattern = (RegExpBuilder.from_test_cases(test_cases)
               .with_conversion_of_non_words()
               .build())
    assert pattern == expected_pattern
    for test_case in test_cases:
        assert re.match(pattern, test_case)


@pytest.mark.parametrize(
    "test_cases,expected_pattern",
    [
        pytest.param(["aababab"], "^aababab$"),
        pytest.param(["aabababab"], "^a(?:ab){4}$")
    ]
)
def test_minimum_repetitions(test_cases, expected_pattern):
    pattern = (RegExpBuilder.from_test_cases(test_cases)
               .with_conversion_of_repetitions()
               .with_minimum_repetitions(3)
               .build())
    assert pattern == expected_pattern
    for test_case in test_cases:
        assert re.match(pattern, test_case)


@pytest.mark.parametrize(
    "test_cases,expected_pattern",
    [
        pytest.param(["ababab"], "^ababab$"),
        pytest.param(["abcabcabc"], "^(?:abc){3}$")
    ]
)
def test_minimum_substring_length(test_cases, expected_pattern):
    pattern = (RegExpBuilder.from_test_cases(test_cases)
               .with_conversion_of_repetitions()
               .with_minimum_substring_length(3)
               .build())
    assert pattern == expected_pattern
    for test_case in test_cases:
        assert re.match(pattern, test_case)


def test_error_for_empty_test_cases():
    with pytest.raises(ValueError) as exception_info:
        RegExpBuilder.from_test_cases([])
    assert (
        exception_info.value.args[0] ==
        "No test cases have been provided for regular expression generation"
    )


def test_error_for_invalid_minimum_repetitions():
    with pytest.raises(ValueError) as exception_info:
        RegExpBuilder.from_test_cases(["abcd"]).with_minimum_repetitions(-4)
    assert (
        exception_info.value.args[0] ==
        "Quantity of minimum repetitions must be greater than zero"
    )


def test_error_for_invalid_minimum_substring_length():
    with pytest.raises(ValueError) as exception_info:
        RegExpBuilder.from_test_cases(["abcd"]).with_minimum_substring_length(-2)
    assert (
        exception_info.value.args[0] ==
        "Minimum substring length must be greater than zero"
    )
