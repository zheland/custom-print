# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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

[0.1.0]: https://github.com/zheland/custom_print/releases/tag/v0.1.0
