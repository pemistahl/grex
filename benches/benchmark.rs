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

use criterion::{criterion_group, criterion_main, Criterion};
use grex::RegExpBuilder;
use itertools::Itertools;
use std::fs::File;
use std::io::Read;

fn load_test_cases() -> Vec<String> {
    let mut f = File::open("./benches/testcases.txt").expect("Test cases could not be loaded");
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    s.split("\n")
        .map(|test_case| test_case.to_string())
        .collect_vec()
}

fn benchmark_grex_with_default_settings(c: &mut Criterion) {
    let test_cases = load_test_cases();
    c.bench_function("grex with default settings", |bencher| {
        bencher.iter(|| RegExpBuilder::from(&test_cases).build())
    });
}

fn benchmark_grex_with_conversion_of_repetitions(c: &mut Criterion) {
    let test_cases = load_test_cases();
    c.bench_function("grex with conversion of repetitions", |bencher| {
        bencher.iter(|| {
            RegExpBuilder::from(&test_cases)
                .with_conversion_of_repetitions()
                .build()
        })
    });
}

fn benchmark_grex_with_conversion_of_digits(c: &mut Criterion) {
    let test_cases = load_test_cases();
    c.bench_function("grex with conversion of digits", |bencher| {
        bencher.iter(|| {
            RegExpBuilder::from(&test_cases)
                .with_conversion_of_digits()
                .build()
        })
    });
}

fn benchmark_grex_with_conversion_of_non_digits(c: &mut Criterion) {
    let test_cases = load_test_cases();
    c.bench_function("grex with conversion of non-digits", |bencher| {
        bencher.iter(|| {
            RegExpBuilder::from(&test_cases)
                .with_conversion_of_non_digits()
                .build()
        })
    });
}

fn benchmark_grex_with_conversion_of_words(c: &mut Criterion) {
    let test_cases = load_test_cases();
    c.bench_function("grex with conversion of words", |bencher| {
        bencher.iter(|| {
            RegExpBuilder::from(&test_cases)
                .with_conversion_of_words()
                .build()
        })
    });
}

fn benchmark_grex_with_conversion_of_non_words(c: &mut Criterion) {
    let test_cases = load_test_cases();
    c.bench_function("grex with conversion of non-words", |bencher| {
        bencher.iter(|| {
            RegExpBuilder::from(&test_cases)
                .with_conversion_of_non_words()
                .build()
        })
    });
}

fn benchmark_grex_with_conversion_of_whitespace(c: &mut Criterion) {
    let test_cases = load_test_cases();
    c.bench_function("grex with conversion of whitespace", |bencher| {
        bencher.iter(|| {
            RegExpBuilder::from(&test_cases)
                .with_conversion_of_whitespace()
                .build()
        })
    });
}

fn benchmark_grex_with_conversion_of_non_whitespace(c: &mut Criterion) {
    let test_cases = load_test_cases();
    c.bench_function("grex with conversion of non-whitespace", |bencher| {
        bencher.iter(|| {
            RegExpBuilder::from(&test_cases)
                .with_conversion_of_non_whitespace()
                .build()
        })
    });
}

fn benchmark_grex_with_case_insensitive_matching(c: &mut Criterion) {
    let test_cases = load_test_cases();
    c.bench_function("grex with case-insensitive matching", |bencher| {
        bencher.iter(|| {
            RegExpBuilder::from(&test_cases)
                .with_case_insensitive_matching()
                .build()
        })
    });
}

fn benchmark_grex_with_verbose_mode(c: &mut Criterion) {
    let test_cases = load_test_cases();
    c.bench_function("grex with verbose mode", |bencher| {
        bencher.iter(|| RegExpBuilder::from(&test_cases).with_verbose_mode().build())
    });
}

criterion_group!(
    benches,
    benchmark_grex_with_default_settings,
    benchmark_grex_with_conversion_of_repetitions,
    benchmark_grex_with_conversion_of_digits,
    benchmark_grex_with_conversion_of_non_digits,
    benchmark_grex_with_conversion_of_words,
    benchmark_grex_with_conversion_of_non_words,
    benchmark_grex_with_conversion_of_whitespace,
    benchmark_grex_with_conversion_of_non_whitespace,
    benchmark_grex_with_case_insensitive_matching,
    benchmark_grex_with_verbose_mode
);

criterion_main!(benches);
