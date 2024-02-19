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

from typing import List


class RegExpBuilder:
    """This class builds regular expressions from user-provided test cases."""

    @classmethod
    def from_test_cases(cls, test_cases: List[str]) -> "RegExpBuilder":
        """Specify the test cases to build the regular expression from.

        The test cases need not be sorted because `RegExpBuilder` sorts them internally.

        Args:
            test_cases (list[str]): The list of test cases

        Raises:
            ValueError: if `test_cases` is empty
        """

    def with_conversion_of_digits(self) -> "RegExpBuilder":
        """Convert any Unicode decimal digit to character class `\d`.

        This method takes precedence over `with_conversion_of_words` if both are set.
        Decimal digits are converted to `\d`, the remaining word characters to `\w`.

        This method takes precedence over `with_conversion_of_non_whitespace` if both are set.
        Decimal digits are converted to `\d`, the remaining non-whitespace characters to `\S`.
        """

    def with_conversion_of_non_digits(self) -> "RegExpBuilder":
        """Convert any character which is not a Unicode decimal digit to character class `\D`.

        This method takes precedence over `with_conversion_of_non_words` if both are set.
        Non-digits which are also non-word characters are converted to `\D`.

        This method takes precedence over `with_conversion_of_non_whitespace` if both are set.
        Non-digits which are also non-space characters are converted to `\D`.
        """

    def with_conversion_of_whitespace(self) -> "RegExpBuilder":
        """Convert any Unicode whitespace character to character class `\s`.

        This method takes precedence over `with_conversion_of_non_digits` if both are set.
        Whitespace characters are converted to `\s`, the remaining non-digit characters to `\D`.

        This method takes precedence over `with_conversion_of_non_words` if both are set.
        Whitespace characters are converted to `\s`, the remaining non-word characters to `\W`.
        """

    def with_conversion_of_non_whitespace(self) -> "RegExpBuilder":
        """Convert any character which is not a Unicode whitespace character to character class `\S`."""

    def with_conversion_of_words(self) -> "RegExpBuilder":
        """Convert any Unicode word character to character class `\w`.

        This method takes precedence over `with_conversion_of_non_digits` if both are set.
        Word characters are converted to `\w`, the remaining non-digit characters to `\D`.

        This method takes precedence over `with_conversion_of_non_whitespace` if both are set.
        Word characters are converted to `\w`, the remaining non-space characters to `\S`.
        """

    def with_conversion_of_non_words(self) -> "RegExpBuilder":
        """Convert any character which is not a Unicode word character to character class `\W`.

        This method takes precedence over `with_conversion_of_non_whitespace` if both are set.
        Non-words which are also non-space characters are converted to `\W`.
        """

    def with_conversion_of_repetitions(self) -> "RegExpBuilder":
        """Detect repeated non-overlapping substrings and to convert them to `{min,max}` quantifier notation."""

    def with_case_insensitive_matching(self) -> "RegExpBuilder":
        """Enable case-insensitive matching of test cases so that letters match both upper and lower case."""

    def with_capturing_groups(self) -> "RegExpBuilder":
        """Replace non-capturing groups with capturing ones."""

    def with_minimum_repetitions(self, quantity: int) -> "RegExpBuilder":
        """Specify the minimum quantity of substring repetitions to be converted
        if `with_conversion_of_repetitions` is set.

        If the quantity is not explicitly set with this method, a default value of 1 will be used.

        Args:
            quantity (int): The minimum quantity of substring repetitions

        Raises:
            ValueError: if `quantity` is zero
        """

    def with_minimum_substring_length(self, length: int) -> "RegExpBuilder":
        """Specify the minimum length a repeated substring must have in order
        to be converted if `with_conversion_of_repetitions` is set.

        If the length is not explicitly set with this method, a default value of 1 will be used.

        Args:
            length (int): The minimum substring length

        Raises:
            ValueError: if `length` is zero
        """

    def with_escaping_of_non_ascii_chars(self, use_surrogate_pairs: bool) -> "RegExpBuilder":
        """Convert non-ASCII characters to unicode escape sequences.

        The parameter `use_surrogate_pairs` specifies whether to convert astral
        code planes (range `U+010000` to `U+10FFFF`) to surrogate pairs.

        Args:
            use_surrogate_pairs (bool): Whether to convert astral code planes to surrogate pairs
        """

    def with_verbose_mode(self) -> "RegExpBuilder":
        """ Produce a nicer looking regular expression in verbose mode."""

    def without_start_anchor(self) -> "RegExpBuilder":
        """Remove the caret anchor '^' from the resulting regular expression,
        thereby allowing to match the test cases also when they do not occur
        at the start of a string.
        """

    def without_end_anchor(self) -> "RegExpBuilder":
        """Remove the dollar sign anchor '$' from the resulting regular expression,
        thereby allowing to match the test cases also when they do not occur
        at the end of a string.
        """

    def without_anchors(self) -> "RegExpBuilder":
        """Remove the caret and dollar sign anchors from the resulting regular expression,
        thereby allowing to match the test cases also when they occur within a larger
        string that contains other content as well.
        """

    def build(self) -> str:
        """Build the actual regular expression using the previously given settings."""
