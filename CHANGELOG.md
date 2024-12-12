# Changelog

All notable changes to this project will be documented in this file. This
changelog complies with [Keep a
Changelog](https://keepachangelog.com/en/1.1.0/).

## [Unreleased]

## [0.3.0] - 2024-12-13

### Changed

  - Move and split functions into separate modules for modularity and reusability.
  - Write unit tests for each split function to ensure correctness.
  - Write documentation comments for each new function.

## [0.2.1] - 2024-04-15

### Added

  - Add a new command-line argument `--prime-factors` (short version `-p`) to
    print only the prime factors.

### Change

  - Update release profile settings to get smaller Rust executables.
  - Modify the `factorize` function to support printing only prime factors when
    the `prime_factors` flag is enabled.
  - Improve algorithm in `factorize`.

## [0.2.0] - 2024-03-10

### Changed

  - Change the command-line argument parsing from StructOpt to Clap.

### Added

  - Add feature to calculate LCM and GCD of numbers.

## [0.1.0] - 2023-12-03

### Added

  - Initial release: basic functionality to get the prime factors of each
    specified integer.

[unreleased]: https://github.com/walker84837/factorflow/compare/v0.3.0...HEAD
[0.3.0]: https://github.com/walker84837/factorflow/compare/v0.2.1...v0.3.0
[0.2.1]: https://github.com/walker84837/factorflow/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/walker84837/factorflow/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/walker84837/factorflow/releases/tag/v0.1.0
