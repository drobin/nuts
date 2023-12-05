# Changelog

All notable changes to this project will be documented in this file.

## [0.2.4] - 2023-12-05

### Changed

- Removed Serde dependency
- Update dependencies:
  * `nuts-container` 0.1.1 -> 0.2.2
  * `nuts-bytes` 0.1.1 -> 0.2.2
  * `nuts-directory` 0.3.1 -> 0.3.2

## [0.2.3] - 2023-11-10

### Added

- `Archive::lookup()` to search for an entry

## [0.2.2] - 2023-11-07

### Added

- Put timestamps into entries

### Changed

- Remove `Mode` from public interface

## [0.2.1] - 2023-11-03

### Changed

- Move `container::BufContainer` to `pager::Pager`
- Correct handling of `Error::InvalidBlockSize` error
- Put id into `Error::InvalidType` error

## [0.2.0] - 2023-11-03

### Added

- The `Mode` type encodes the type and access rights and of an entry.

### Changed

- Refactoring of `Entry` & `EntryMut` which now includes the `Mode`.

## [0.1.1] - 2023-10-27

### Changed

- Refactoring of the header of the archive. The header block is guarded by a
  magic value.

## [0.1.0] - 2023-10-25

Initial public release.

[0.1.0]: https://github.com/drobin/nuts-archive/tree/v0.1.0
[0.1.1]: https://github.com/drobin/nuts-archive/tree/v0.1.1
[0.2.0]: https://github.com/drobin/nuts-archive/tree/v0.2.0
[0.2.1]: https://github.com/drobin/nuts-archive/tree/v0.2.1
[0.2.2]: https://github.com/drobin/nuts-archive/tree/v0.2.2
[0.2.3]: https://github.com/drobin/nuts-archive/tree/v0.2.3
[0.2.4]: https://github.com/drobin/nuts-archive/tree/v0.2.4
