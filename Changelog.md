# Changelog

All notable changes to this project will be documented in this file.

## [0.2.1] - 2023-10-13

### Added

- `MemoryBackend` has a variable block size.

## [0.2.0] - 2023-10-10

### Changed

- `Backend::aquire()` has now a `buf` argument.
- New internal en-/decryption API: `CipherContext` replaces methods on the
  `Cipher` type.

## [0.1.1] - 2023-10-02

### Changed

- Update dependencies:
  * `log` 0.4.14 -> 0.4.20
  * `nuts-bytes` 0.1.0 -> 0.1.1
  * `serde` 1.0.152 -> 1.0.188

## [0.1.0] - 2023-08-29

Initial public release.

[0.1.0]: https://github.com/drobin/nuts-container/tree/v0.1.0
[0.1.1]: https://github.com/drobin/nuts-container/tree/v0.1.1
[0.2.0]: https://github.com/drobin/nuts-container/tree/v0.2.0
[0.2.1]: https://github.com/drobin/nuts-container/tree/v0.2.1
