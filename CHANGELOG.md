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