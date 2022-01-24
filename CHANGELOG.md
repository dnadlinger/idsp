# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
* Getter methods for `PLL`, `RPLL`, `Lowpass`, `Unwrapper`
### Changed
* `Accu`, `Unwrapper`, `overflowing_sub` are now generic.
* Revert `to PLL::update()` returning the phase increment as that has less bias
  (while it does have more noise).
### Removed

## [0.6.0] - 2022-01-19
### Changed
* Let the `wrap` return value of `overlowing_sub()` be a `i32` in analogy to the
  remaining API functions for `i32` and the changes in `idsp v0.5.0`.

## [0.5.0] - 2022-01-19
### Changed
* The shift parameters (log2 gains, log2 time constants) have all been migrated
  from `u8` to `u32` to be consistent with `core`. This is also in preparation
  for `unchecked_shr()` and `unchecked_shl()`.
* `PLL::update()` does not return the phase increment but instead the actual
  frequency estimate.
* Additional zeros in the PLL transfer functions have been placed at Nyquist.

## [0.4.0] - 2021-12-13

### Added
* Deriving `Serialize` for `IIR` and `IIR (int)` to support miniconf updates.

## [0.3.0] - 2021-11-02

### Removed
* Removed `nightly` feature as it was broken in 0.2.0 and hard to fix with
  generics. Instead use `num_cast::clamp`.

## [0.2.0] - 2021-11-01

### Changed
* IIR is now generic over the float type (f32 and f64)

## [0.1.0] - 2021-09-15

Library initially released on crates.io

[Unreleased]: https://github.com/quartiq/idsp/compare/v0.6.0...HEAD
[0.6.0]: https://github.com/quartiq/idsp/releases/tag/v0.6.0
[0.5.0]: https://github.com/quartiq/idsp/releases/tag/v0.5.0
[0.4.0]: https://github.com/quartiq/idsp/releases/tag/v0.4.0
[0.3.0]: https://github.com/quartiq/idsp/releases/tag/v0.3.0
[0.2.0]: https://github.com/quartiq/idsp/releases/tag/v0.2.0
[0.1.0]: https://github.com/quartiq/idsp/releases/tag/v0.1.0
