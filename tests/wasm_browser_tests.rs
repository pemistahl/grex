/*
 * Copyright © 2019-today Peter M. Stahl pemistahl@gmail.com
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

#![cfg(target_family = "wasm")]

use grex::WasmRegExpBuilder;
use indoc::indoc;
use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn assert_regexpbuilder_succeeds() {
    let test_cases = Box::new([JsValue::from("hello"), JsValue::from("world")]);
    let builder = WasmRegExpBuilder::from(test_cases);
    assert!(builder.is_ok());
    let regexp = builder.unwrap().build();
    assert_eq!(regexp, "^(?:hello|world)$");
}

#[wasm_bindgen_test]
fn assert_regexpbuilder_fails() {
    let builder = WasmRegExpBuilder::from(Box::new([]));
    assert_eq!(
        builder.err(),
        Some(JsValue::from(
            "No test cases have been provided for regular expression generation"
        ))
    );
}

#[wasm_bindgen_test]
fn test_conversion_of_digits() {
    let test_cases = Box::new([JsValue::from("abc  "), JsValue::from("123")]);
    let regexp = WasmRegExpBuilder::from(test_cases)
        .unwrap()
        .withConversionOfDigits()
        .build();
    assert_eq!(regexp, "^(?:abc  |\\d\\d\\d)$");
}

#[wasm_bindgen_test]
fn test_conversion_of_non_digits() {
    let test_cases = Box::new([JsValue::from("abc  "), JsValue::from("123")]);
    let regexp = WasmRegExpBuilder::from(test_cases)
        .unwrap()
        .withConversionOfNonDigits()
        .build();
    assert_eq!(regexp, "^(?:\\D\\D\\D\\D\\D|123)$");
}

#[wasm_bindgen_test]
fn test_conversion_of_whitespace() {
    let test_cases = Box::new([JsValue::from("abc  "), JsValue::from("123")]);
    let regexp = WasmRegExpBuilder::from(test_cases)
        .unwrap()
        .withConversionOfWhitespace()
        .build();
    assert_eq!(regexp, "^(?:abc\\s\\s|123)$");
}

#[wasm_bindgen_test]
fn test_conversion_of_non_whitespace() {
    let test_cases = Box::new([JsValue::from("abc  "), JsValue::from("123")]);
    let regexp = WasmRegExpBuilder::from(test_cases)
        .unwrap()
        .withConversionOfNonWhitespace()
        .build();
    assert_eq!(regexp, "^\\S\\S\\S(?:  )?$");
}

#[wasm_bindgen_test]
fn test_conversion_of_words() {
    let test_cases = Box::new([JsValue::from("abc  "), JsValue::from("123")]);
    let regexp = WasmRegExpBuilder::from(test_cases)
        .unwrap()
        .withConversionOfWords()
        .build();
    assert_eq!(regexp, "^\\w\\w\\w(?:  )?$");
}

#[wasm_bindgen_test]
fn test_conversion_of_non_words() {
    let test_cases = Box::new([JsValue::from("abc  "), JsValue::from("123")]);
    let regexp = WasmRegExpBuilder::from(test_cases)
        .unwrap()
        .withConversionOfNonWords()
        .build();
    assert_eq!(regexp, "^(?:abc\\W\\W|123)$");
}

#[wasm_bindgen_test]
fn test_conversion_of_repetitions() {
    let test_cases = Box::new([JsValue::from("abc  "), JsValue::from("123")]);
    let regexp = WasmRegExpBuilder::from(test_cases)
        .unwrap()
        .withConversionOfRepetitions()
        .build();
    assert_eq!(regexp, "^(?:abc {2}|123)$");
}

#[wasm_bindgen_test]
fn test_case_insensitive_matching() {
    let test_cases = Box::new([
        JsValue::from("ABC"),
        JsValue::from("abc  "),
        JsValue::from("123"),
    ]);
    let regexp = WasmRegExpBuilder::from(test_cases)
        .unwrap()
        .withCaseInsensitiveMatching()
        .build();
    assert_eq!(regexp, "(?i)^(?:abc(?:  )?|123)$");
}

#[wasm_bindgen_test]
fn test_capturing_groups() {
    let test_cases = Box::new([JsValue::from("abc  "), JsValue::from("123")]);
    let regexp = WasmRegExpBuilder::from(test_cases)
        .unwrap()
        .withCapturingGroups()
        .build();
    assert_eq!(regexp, "^(abc  |123)$");
}

#[wasm_bindgen_test]
fn test_escaping_of_non_ascii_chars() {
    let test_cases = Box::new([
        JsValue::from("abc  "),
        JsValue::from("123"),
        JsValue::from("♥"),
    ]);
    let regexp = WasmRegExpBuilder::from(test_cases)
        .unwrap()
        .withEscapingOfNonAsciiChars(false)
        .build();
    assert_eq!(regexp, "^(?:abc  |123|\\u{2665})$");
}

#[wasm_bindgen_test]
fn test_verbose_mode() {
    let test_cases = Box::new([
        JsValue::from("abc  "),
        JsValue::from("123"),
        JsValue::from("♥"),
    ]);
    let regexp = WasmRegExpBuilder::from(test_cases)
        .unwrap()
        .withVerboseMode()
        .build();
    assert_eq!(
        regexp,
        indoc!(
            r#"
            (?x)
            ^
              (?:
                abc\ \ 
                |
                123
                |
                ♥
              )
            $"#
        )
    );
}

#[wasm_bindgen_test]
fn test_without_start_anchor() {
    let test_cases = Box::new([JsValue::from("abc  "), JsValue::from("123")]);
    let regexp = WasmRegExpBuilder::from(test_cases)
        .unwrap()
        .withoutStartAnchor()
        .build();
    assert_eq!(regexp, "(?:abc  |123)$");
}

#[wasm_bindgen_test]
fn test_without_end_anchor() {
    let test_cases = Box::new([JsValue::from("abc  "), JsValue::from("123")]);
    let regexp = WasmRegExpBuilder::from(test_cases)
        .unwrap()
        .withoutEndAnchor()
        .build();
    assert_eq!(regexp, "^(?:abc  |123)");
}

#[wasm_bindgen_test]
fn test_without_anchors() {
    let test_cases = Box::new([JsValue::from("abc  "), JsValue::from("123")]);
    let regexp = WasmRegExpBuilder::from(test_cases)
        .unwrap()
        .withoutAnchors()
        .build();
    assert_eq!(regexp, "(?:abc  |123)");
}

#[wasm_bindgen_test]
fn test_minimum_repetitions() {
    let test_cases = Box::new([JsValue::from("abc  "), JsValue::from("123")]);
    let builder = WasmRegExpBuilder::from(test_cases)
        .unwrap()
        .withMinimumRepetitions(0);
    assert_eq!(
        builder.err(),
        Some(JsValue::from(
            "Quantity of minimum repetitions must be greater than zero"
        ))
    );
}

#[wasm_bindgen_test]
fn test_minimum_substring_length() {
    let test_cases = Box::new([JsValue::from("abc  "), JsValue::from("123")]);
    let builder = WasmRegExpBuilder::from(test_cases)
        .unwrap()
        .withMinimumSubstringLength(0);
    assert_eq!(
        builder.err(),
        Some(JsValue::from(
            "Minimum substring length must be greater than zero"
        ))
    );
}
