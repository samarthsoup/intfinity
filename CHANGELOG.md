# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2024-09-03
### Added
- Initial creation of `Intfinity` type for handling finite and infinite values.
- Implemented arithmetic operations (`Add`, `Sub`, `Mul`, `Div`) for `Intfinity<T>`.
- Implemented custom traits (`CheckedAdd`, `CheckedSub`, `CheckedMul`, `CheckedDiv`, `Zero`, `Negate`) to properly handle infinity arithmetic.

## [0.2.0] - 2024-09-04
### Added
- Implemented comparison traits (`PartialEq`, `Eq`, `PartialOrd`, `Ord`) for `Intfinity<T>`.
- Introduced an `indeterminate_cmp` function for `Intfinity<T>` when you want to handle a case where `+inf != +inf`.
- Introduced the `intfinity!` macro for easy initilization of new `Intfinity<T>` objects.

### Changed
- Introduced implicit `is_zero` by defining the mandatory `zero` function in `Zero` trait.

## [0.2.1] - 2024-09-04
### Changed
- Refactored unit tests.
- Altered example in `README.md`.

## [0.2.2] - 2024-09-04
### Changed
- Changed `Ord` and `Eq` traits of `Intfinity` to derived macros rather than custom implementation as they work the same way.

## [0.3.0] - 2024-09-05
### Added
- `SingleInfiniteNumber` and `DoubleInfiniteNumber` enums for safe handling of numerical types with infinity on one side or both.

### Replaced
- `Intfinity` enum is now renamed as `DoubleInfiniteNumber`.

## [0.3.1] - 2024-09-06
### Added
- The doctor's report just came in, `intfinity` is now `no-std`!

## [0.3.2] - 2024-09-29
### Replaced
- Manual implementations for numeric primitives have been replaced with a macro, which has saved LOC.