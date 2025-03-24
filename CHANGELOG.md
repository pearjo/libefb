# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- This CHANGELOG file

## [0.2.0] - 2025-03-24

### Added

- `measurements` module for physical quantities e.g. a length.

### Changed

- Use new measurement types wherever the core measurements were used.

### Removed

- Measurement types that were part of the `core` module.

## [0.1.0] - 2025-03-16

### Added

- Parser for ARINC 424 and OpenAir.
- FMS that reads navigation data and decodes a route.
- Aircraft performance entry.
- Fuel planning and Mass & Balance.

[unreleased]: https://github.com/pearjo/libefb/compare/0.2.0...HEAD
[0.2.0]: https://github.com/pearjo/libefb/compare/0.1.0...0.2.0
[0.1.0]: https://github.com/pearjo/libefb/releases/tag/0.1.0
