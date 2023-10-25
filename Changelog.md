# Changelog

All notable changes to this project will be documented in this file.

## [0.3.0] - 2023-10-25

### Changed
- The backend handles a generic `Path` (instead of an owned `PathBuf`).

## [0.2.0] - 2023-10-10

### Changed

- New `std::fmt::Debug` implementation for `Id`.
- `nuts_container::backend::Backend` changed its interface.
- Update dependencies:
  * `nuts-container` 0.1.1 -> 0.2.0

## [0.1.1] - 2023-10-02

### Changed
- Update dependencies:
  * `getrandom` 0.2.8 -> 0.2.10
  * `log` 0.4.14 -> 0.4.20
  * `nuts-container` 0.1.0 -> 0.1.1
  * `nuts-bytes` 0.1.0 -> 0.1.1
  * `serde` 1.0.152 -> 1.0.188

### Fixed

- Backend cannot be created without overwrite option set

## [0.1.0] - 2023-10-02

Initial public release.

[0.1.0]: https://github.com/drobin/nuts-directory/tree/v0.1.0
[0.1.1]: https://github.com/drobin/nuts-directory/tree/v0.1.1
[0.2.0]: https://github.com/drobin/nuts-directory/tree/v0.2.0
[0.3.0]: https://github.com/drobin/nuts-directory/tree/v0.3.0
