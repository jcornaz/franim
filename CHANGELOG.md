# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).


## [Unreleased]


## [0.1.1] - 2024-08-31

### Changed

* `Animation` is renamed to `AnimationState`.
  (a deprecated `type Animation = AnimationState`, so this change is not breaking)

### Added

* `AnimationState::from_frame_duration_secs`
* `AnimationState::update_secs`


## [0.1.0] - 2023-12-10

* `Animation` struct with `from_frame_duration`, `update` and `current_frame` methods
* Support `no_std` by disabling `std` feature

[Unreleased]: https://github.com/jcornaz/franim/compare/v0.1.1...HEAD
[0.1.1]: https://github.com/jcornaz/franim/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/jcornaz/franim/compare/...v0.1.0
