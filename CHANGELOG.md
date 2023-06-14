# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.0.0] - 2023-06-14
### Added
- Add support for custom attributes for macros that declare other macros.

### Changed
- The minimum supported Rust version has been increased to 1.60.0.

## [0.1.0] - 2021-06-12
### Added
- Macros `define_macro`, `define_macros`, `define_{print, println, dbg}`, etc.
- Macros `define_panic_hook`, `define_init_panic_hook`, `init_panic_hook`.
- Traits `WriteStr`, `WriteBytes`, `Flush`.
- Writers `ConcatWriter`, `ConcatTryWriter`, `FmtWriter`, `FmtTryWriter`, `IoWriter`, `IoTryWriter`.
- Multiple closure wrappers for different function signatures.
- API documentation with examples.
- Tests and doc-tests.
- GitHub CI.

[Unreleased]: https://github.com/zheland/custom-print/compare/v1.0.0...HEAD
[1.0.0]: https://github.com/zheland/custom-print/compare/v0.1.0...v1.0.0
[0.1.0]: https://github.com/zheland/custom-print/compare/v0.0.0...v0.1.0
