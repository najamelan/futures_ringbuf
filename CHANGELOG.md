# futures_ringbuf Changelog

## [Unreleased]

  [Unreleased]: https://github.com/najamelan/futures_ringbuf/compare/0.4...dev

## [0.4.0]

  [0.4.0]: https://github.com/najamelan/futures_ringbuf/compare/0.3.1...0.4.0
  
### Added 

  - **BREAKING CHANGE**: Update dependencies, including _ringbuf_ to version `0.3` and 
    _tokio-util_ to version `0.7`.
  - `Dictator.seed` method to read the seed used on creation.
  
### Changed 

  - Renamed `Dictator::seed` method to `new_seed`.

## [0.3.1]

  [0.3.1]: https://github.com/najamelan/futures_ringbuf/compare/0.3.0...0.3.1

### Updated
  - CONTRIBUTING.md
  - external_doc gets removed in rustdoc 1.54.

### Fixed
  - Move CI to github actions


## [0.3.0] - 2020-11-03

  [0.3.0]: https://github.com/najamelan/futures_ringbuf/compare/0.2.1...0.3.0

## Removed
  - **BREAKING CHANGE**: Get rid of tokio version of traits in favor of `tokio_util::compat`.
  - **BREAKING CHANGE**: Get rid of feature flags.

## Added
  - `Sketchy`: A way to randomize behavior of network mocks.

## [0.2.1] - 2020-04-15

  [0.2.1]: https://github.com/najamelan/futures_ringbuf/compare/0.2.0...0.2.1

### Updated
  - Update to futures_codec 0.4.

### Fixed
  - fix docs to not break on stable.
  - fix CI configuration.


## [0.2.0] - 2019-01-15

  [0.2.0]: https://github.com/najamelan/futures_ringbuf/compare/0.1.7...0.2.0

### Added
  - Implement tokio AsyncRead/Write. This is behind a feature flag.


## [0.1.7] - 2019-11-12

  [0.1.7]: https://github.com/najamelan/futures_ringbuf/compare/0.1.6...0.1.7

### Updated
  - Update to ringbuf 0.2.

### Fixed
  - Fix a bug where a waker wasn't woken up in endpoint when the connection get's closed.


## [0.1.6] - 2019-11-12

  [0.1.6]: https://github.com/najamelan/futures_ringbuf/compare/0.1.5...0.1.6

### Updated
  - Update to futures 0.3.
  - Test on stable as well as nightly.


## [0.1.5] - 2019-10-10

  [0.1.5]: https://github.com/najamelan/futures_ringbuf/compare/0.1.4...0.1.5

### Added
  - Add Endpoint to mock full duplex connection.


## [0.1.4] - 2019-09-29

  [0.1.4]: https://github.com/najamelan/futures_ringbuf/compare/0.1.3...0.1.4

### Fixed
  - cleanup readme
  - fix wasm tests

## [0.1.3] - 2019-09-28

  [0.1.3]: https://github.com/najamelan/futures_ringbuf/compare/0.1.2...0.1.3

### Updated
  - update dependencies

### Fixed
  - fix docs.rs readme

## [0.1.2] - 2019-08-15

  [0.1.2]: https://github.com/najamelan/futures_ringbuf/compare/0.1.1...0.1.2

### Fixed
- fix category slug that was lost in git history

## [0.1.1] - 2019-08-15

  [0.1.1]: https://github.com/najamelan/futures_ringbuf/compare/0.1.0...0.1.1

### Updated
- test wasm support on CI
- update dependencies that merged bug fixes since yesterday

## [0.1.0] - 2019-08-14

- initial release.


